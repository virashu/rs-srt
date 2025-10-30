use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use crate::packet::{Packet, PacketContent};

pub struct Connection {
    pub est: SystemTime,
    pub addr: SocketAddr,
    pub peer_srt_socket_id: u32,
}

impl Connection {
    pub fn send(&self, socket: &UdpSocket, content: PacketContent) -> anyhow::Result<()> {
        let packet = Packet {
            timestamp: SystemTime::now()
                .duration_since(self.est)
                .unwrap()
                .as_micros() as u32,
            dest_socket_id: self.peer_srt_socket_id,
            content,
        };

        socket.send_to(&packet.to_raw(), self.addr)?;

        Ok(())
    }
}
