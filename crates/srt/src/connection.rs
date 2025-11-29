use std::{
    net::{SocketAddr, UdpSocket},
    sync::{
        Mutex,
        atomic::{AtomicU32, Ordering},
    },
    time::{Instant, SystemTime},
};

use anyhow::{Result, bail};

use crate::{
    constants::{FULL_ACK_INTERVAL, HANDSHAKE_MAGIC_CODE, RTT_INIT, RTT_VAR_INIT},
    packet::{
        Packet, PacketContent,
        control::{ControlPacketInfo, ack::Ack, handshake::Handshake, nak::Nak},
        data::DataPacketInfo,
    },
    server::OnDataHandler,
};

pub struct Connection<'c> {
    socket: &'c UdpSocket,
    on_data: Option<&'c OnDataHandler>,

    // Srt info
    pub stream_id: Option<String>,
    pub established: SystemTime,
    pub addr: SocketAddr,
    pub peer_srt_socket_id: u32,

    // /// # of packets received since last ack was sent
    // received_since_ack: AtomicU32,
    /// Ack sequence number
    ack_counter: AtomicU32,

    /// Timestamp of the last **sent** Ack
    /// (used to caltulate RTT)
    last_ack_timestamp: Mutex<Instant>,

    /// Package sequence number of last received data packet
    last_received: AtomicU32,

    /// <add link>
    rtt: AtomicU32,
    /// <add link>
    rtt_var: AtomicU32,
}

impl<'c> Connection<'c> {
    pub fn establish_v5(socket: &'c UdpSocket, on_data: Option<&'c OnDataHandler>) -> Result<Self> {
        let mut buf = [0; 200];

        tracing::debug!("Waiting for a handshake...");

        //
        // Induction
        //

        let (n, addr) = socket.recv_from(&mut buf)?;
        let data = &buf[..n];

        tracing::debug!("Connection: {addr}");

        let in_packet = Packet::from_raw(data)?;
        let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content
        else {
            bail!("Failed to unwrap handshake");
        };

        let out_packet_v5 = Packet {
            timestamp: in_packet.timestamp + 1,
            dest_socket_id: handshake.srt_socket_id,
            content: PacketContent::Control(ControlPacketInfo::Handshake(Handshake {
                version: 5,
                extension_field: HANDSHAKE_MAGIC_CODE,
                srt_socket_id: 42,
                syn_cookie: 42,
                ..handshake
            })),
        };
        socket.send_to(&out_packet_v5.to_raw(), addr)?;

        tracing::debug!("Completed Induction");

        //
        // Conclusion
        //

        let (n, addr) = socket.recv_from(&mut buf)?;
        let data = &buf[..n];

        let in_packet = Packet::from_raw(data)?;
        let PacketContent::Control(ControlPacketInfo::Handshake(handshake)) = in_packet.content
        else {
            bail!("Failed to unwrap handshake");
        };

        let peer_srt_socket_id = handshake.srt_socket_id;
        let stream_id = handshake
            .stream_id_extension
            .as_ref()
            .map(|x| x.stream_id.clone());

        let out_packet_v5 = Packet {
            timestamp: in_packet.timestamp + 1,
            dest_socket_id: handshake.srt_socket_id,
            content: PacketContent::Control(ControlPacketInfo::Handshake(handshake)),
        };
        socket.send_to(&out_packet_v5.to_raw(), addr)?;

        tracing::debug!("Completed Conclusion");
        tracing::debug!("Done!");

        let established = SystemTime::now();

        Ok(Self::new(
            socket,
            on_data,
            stream_id,
            established,
            addr,
            peer_srt_socket_id,
        ))
    }

    fn new(
        socket: &'c UdpSocket,
        on_data: Option<&'c OnDataHandler>,
        stream_id: Option<String>,
        established: SystemTime,
        addr: SocketAddr,
        peer_srt_socket_id: u32,
    ) -> Self {
        Self {
            on_data,
            socket,
            stream_id,
            established,
            addr,
            peer_srt_socket_id,

            ack_counter: AtomicU32::new(1),
            last_ack_timestamp: Mutex::new(Instant::now()),
            // received_since_ack: AtomicU32::new(0),
            last_received: AtomicU32::new(0),

            rtt: AtomicU32::new(RTT_INIT),
            rtt_var: AtomicU32::new(RTT_VAR_INIT),
        }
    }

    pub(crate) fn inc_ack(&self) -> u32 {
        self.ack_counter.fetch_add(1, Ordering::Relaxed)
    }

    // pub(crate) fn check_ack(&self) -> bool {
    //     self.received_since_ack
    //         .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some((x + 1) % 64))
    //         == Ok(0)
    // }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn pack(&self, content: PacketContent) -> Result<Packet> {
        Ok(Packet {
            timestamp: SystemTime::now()
                .duration_since(self.established)?
                .as_micros() as u32,
            dest_socket_id: self.peer_srt_socket_id,
            content,
        })
    }

    pub fn send(&self, content: PacketContent) -> Result<()> {
        self.socket
            .send_to(&self.pack(content)?.to_raw(), self.addr)?;

        Ok(())
    }

    fn handle_control(&self, control: &ControlPacketInfo) -> Result<()> {
        tracing::trace!("srt | inbound | control | {control:?}");

        match control {
            ControlPacketInfo::KeepAlive => {
                let keep_alive = PacketContent::Control(ControlPacketInfo::KeepAlive);
                tracing::trace!("srt | outbound | control | {keep_alive:?}");
                self.send(keep_alive)?;
            }
            ControlPacketInfo::AckAck(_) => {
                // Calculate RTT
                // RTT = 7/8 * RTT + 1/8 * rtt
                // RTTVar = 3/4 * RTTVar + 1/4 * abs(RTT - rtt)

                let sent = self.last_ack_timestamp.lock().unwrap();
                let rtt_new: u32 = sent.elapsed().as_micros().try_into().unwrap();

                let rtt_old = self
                    .rtt
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
                        Some(x * 7 / 8 + rtt_new / 8)
                    })
                    .unwrap();

                self.rtt_var
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |rtt_var| {
                        Some(rtt_var * 3 / 4 + (rtt_old).abs_diff(rtt_new) / 4)
                    })
                    .unwrap();
            }
            _ => (),
        }

        Ok(())
    }

    fn handle_data(&self, data: &DataPacketInfo) -> Result<()> {
        let packet_number = data.packet_sequence_number;
        let prev_packet_number = self.last_received.swap(packet_number, Ordering::Relaxed);
        if prev_packet_number + 1 != packet_number && data.message_number != 1 {
            tracing::warn!("Missed {} packets", packet_number - prev_packet_number - 1);

            self.send(PacketContent::Control(ControlPacketInfo::Nak(
                Nak::Single {
                    lost_packet: packet_number - 1,
                },
            )))?;
        }

        tracing::trace!(
            "srt | inbound | data | Data {{ packet_sequence_number: {:?}, position: {:?}, order: {:?}, encryption: {:?}, retransmitted: {:?}, message_number: {:?}, length: {:?} }}",
            data.packet_sequence_number,
            data.position,
            data.order,
            data.encryption,
            data.retransmitted,
            data.message_number,
            data.content.len()
        );

        self.send_full_ack()?;

        // if self.check_ack() {
        //     let ack = PacketContent::Control(ControlPacketInfo::Ack(Ack::Light {
        //         last_ackd_packet_sequence_number: data.packet_sequence_number + 1,
        //     }));
        //     tracing::trace!("srt | outbound | control | {ack:?}");
        //     self.send(ack)?;
        // }

        let mpeg_packet = &data.content[..];

        if let Some(callback) = &self.on_data {
            callback(self, mpeg_packet);
        }

        Ok(())
    }

    pub(crate) fn handle(&self, pack: &Packet) -> Result<()> {
        self.update()?;

        match &pack.content {
            PacketContent::Control(control) => self.handle_control(control)?,
            PacketContent::Data(data) => self.handle_data(data)?,
        }

        Ok(())
    }

    fn send_full_ack(&self) -> Result<()> {
        let ack = PacketContent::Control(ControlPacketInfo::Ack(Ack::Full {
            ack_number: self.inc_ack(),
            last_ackd_packet_sequence_number: self.last_received.load(Ordering::Relaxed) + 1,
            rtt: self.rtt.load(Ordering::Relaxed),
            rtt_variance: self.rtt_var.load(Ordering::Relaxed),
            available_buffer_size: 1,
            packets_receiving_rate: 1,
            estimated_link_capacity: 1,
            receiving_rate: 1,
        }));
        tracing::trace!("srt | outbound | control | {ack:?}");
        self.send(ack)
    }

    pub(crate) fn update(&self) -> Result<()> {
        let mut last_ack_timestamp = self.last_ack_timestamp.lock().unwrap();
        let micros: u32 = last_ack_timestamp.elapsed().as_micros().try_into().unwrap();

        if micros > FULL_ACK_INTERVAL {
            *last_ack_timestamp = Instant::now();
            self.send_full_ack()?;
        }

        Ok(())
    }
}
