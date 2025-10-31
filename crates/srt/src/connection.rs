use std::{
    net::{SocketAddr, UdpSocket},
    sync::atomic::{AtomicU32, Ordering},
    time::SystemTime,
};

use crate::packet::{Packet, PacketContent};

/// Maybe some config??
pub struct Connection {
    pub stream_id: Option<String>,
    pub est: SystemTime,
    pub addr: SocketAddr,
    pub peer_srt_socket_id: u32,
    pub ack_counter: AtomicU32,
}

impl Connection {
    pub fn inc_ack(&self) -> u32 {
        self.ack_counter.fetch_add(1, Ordering::Relaxed)
    }

    pub fn send(&self, socket: &UdpSocket, content: PacketContent) -> anyhow::Result<()> {
        #[allow(clippy::cast_possible_truncation)]
        let packet = Packet {
            timestamp: SystemTime::now().duration_since(self.est)?.as_micros() as u32,
            dest_socket_id: self.peer_srt_socket_id,
            content,
        };

        socket.send_to(&packet.to_raw(), self.addr)?;

        Ok(())
    }
}
