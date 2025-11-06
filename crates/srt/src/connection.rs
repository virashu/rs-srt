use std::{
    net::{SocketAddr, UdpSocket},
    sync::{
        Mutex,
        atomic::{AtomicU32, Ordering},
    },
    time::SystemTime,
};

use anyhow::Result;

use crate::{
    constants::HANDSHAKE_MAGIC_CODE,
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

    /// Ack sequence number
    ack_counter: AtomicU32,
    /// # of packets received since last ack was sent
    received_since_ack: AtomicU32,

    last_ack_timestamp: Mutex<SystemTime>,

    /// Package sequence number of last received data packet
    last_received: AtomicU32,

    rtt: AtomicU32,
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
            return Err(anyhow::anyhow!("Failed to unwrap handshake"));
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
            return Err(anyhow::anyhow!("Failed to unwrap handshake"));
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

        Ok(Self {
            on_data,
            socket,
            stream_id,
            established,
            addr,
            peer_srt_socket_id,
            ack_counter: AtomicU32::new(1),
            last_ack_timestamp: Mutex::new(SystemTime::UNIX_EPOCH),
            received_since_ack: AtomicU32::default(),
            last_received: AtomicU32::default(),
            rtt: AtomicU32::default(),
        })
    }

    pub fn inc_ack(&self) -> u32 {
        self.ack_counter.fetch_add(1, Ordering::Relaxed)
    }

    pub fn check_ack(&self) -> bool {
        self.received_since_ack
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some((x + 1) % 60))
            == Ok(0)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn pack(&self, content: PacketContent) -> Result<Packet> {
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
                let sent = self.last_ack_timestamp.lock().unwrap();
                let delta = SystemTime::now().duration_since(*sent).unwrap();
                self.rtt
                    .store(delta.as_micros().try_into()?, Ordering::Relaxed);
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

            self.send(PacketContent::Control(ControlPacketInfo::Nak(Nak {
                lost_packet: packet_number - 1,
            })))?;
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

        if self.check_ack() {
            let rtt = self.rtt.load(Ordering::Relaxed) + 1_000_000;
            // let ack = PacketContent::Control(ControlPacketInfo::Ack(Ack::Full {
            //     ack_number: self.inc_ack(),
            //     last_ackd_packet_sequence_number: data.packet_sequence_number + 1,
            //     rtt,
            //     rtt_variance: 0,
            //     available_buffer_size: 1,
            //     packets_receiving_rate: 1,
            //     estimated_link_capacity: 1,
            //     receiving_rate: 1,
            // }));
            let ack = PacketContent::Control(ControlPacketInfo::Ack(Ack::Small {
                last_ackd_packet_sequence_number: data.packet_sequence_number + 1,
                rtt,
                rtt_variance: 100,
                available_buffer_size: 1,
            }));
            // let ack = PacketContent::Control(ControlPacketInfo::Ack(Ack::Light {
            //     last_ackd_packet_sequence_number: data.packet_sequence_number + 1,
            // }));
            *self.last_ack_timestamp.lock().unwrap() = SystemTime::now();
            tracing::trace!("srt | outbound | control | {ack:?}");
            self.send(ack)?;
        }

        let mpeg_packet = &data.content[..];

        if let Some(callback) = &self.on_data {
            callback(self, mpeg_packet);
        }

        Ok(())
    }

    pub fn handle(&self, pack: &Packet) -> Result<()> {
        match &pack.content {
            PacketContent::Control(control) => self.handle_control(control)?,
            PacketContent::Data(data) => self.handle_data(data)?,
        }

        Ok(())
    }
}
