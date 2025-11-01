use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    connection::Connection,
    ops::handshake_v5,
    packet::{
        Packet, PacketContent,
        control::{ControlPacketInfo, ack::Ack},
    },
};

type OnConnectHandler = dyn Fn(&Connection);
type OnDiscnnectHandler = dyn Fn(&Connection);
type OnDataHandler = dyn Fn(&Connection, &[u8]);

pub struct Server {
    socket: UdpSocket,
    connections: HashMap<SocketAddr, Connection>,
    on_connect: Option<Box<OnConnectHandler>>,
    on_disconnect: Option<Box<OnDiscnnectHandler>>,
    on_data: Option<Box<OnDataHandler>>,
}

impl Server {
    pub fn new() -> anyhow::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:9000")?;

        Ok(Self {
            socket,
            connections: HashMap::new(),
            on_connect: None,
            on_disconnect: None,
            on_data: None,
        })
    }

    pub fn on_connect(&mut self, f: &'static OnConnectHandler) {
        self.on_connect = Some(Box::new(f));
    }

    pub fn on_disconnect(&mut self, f: &'static OnDiscnnectHandler) {
        self.on_disconnect = Some(Box::new(f));
    }

    pub fn on_data(&mut self, f: &'static OnDataHandler) {
        self.on_data = Some(Box::new(f));
    }

    fn handle(&self, conn: &Connection, pack: &Packet) -> anyhow::Result<()> {
        match &pack.content {
            PacketContent::Control(control) => {
                tracing::debug!("IN: Control: {control:?}");
            }
            PacketContent::Data(data) => {
                // println!("{data:?}");
                tracing::debug!(
                    "IN: Data {{ packet_sequence_number: {:?}, position: {:?}, order: {:?}, encryption: {:?}, retransmitted: {:?}, message_number: {:?}, length: {:?} }}",
                    data.packet_sequence_number,
                    data.position,
                    data.order,
                    data.encryption,
                    data.retransmitted,
                    data.message_number,
                    data.content.len()
                );
                // println!(" => Payload: {:x?}", data.content);

                // let ack_n = conn.inc_ack();
                // let ack = make_full_ack(data, ack_n)?;
                // tracing::debug!("OUT: {ack:?}");
                // conn.send(&self.socket, ack)?;

                if conn.check_ack() {
                    conn.send(
                        &self.socket,
                        PacketContent::Control(ControlPacketInfo::Ack(Ack::Light {
                            last_ackd_packet_sequence_number: data.packet_sequence_number + 1,
                        })),
                    )?;
                }

                let mpeg_packet = &data.content[..];

                if let Some(callback) = &self.on_data {
                    callback(conn, mpeg_packet);
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let mut buf = [0; 10000];

            let (n, addr) = self.socket.recv_from(&mut buf)?;
            let data = &buf[..n];

            if let Some(conn) = self.connections.get(&addr) {
                let pack = Packet::from_raw(data)?;

                if matches!(
                    pack.content,
                    PacketContent::Control(ControlPacketInfo::Shutdown)
                ) {
                    if let Some(conn) = self.connections.remove(&addr)
                        && let Some(callback) = &self.on_disconnect
                    {
                        callback(&conn);
                    }

                    continue;
                }

                self.handle(conn, &pack)?;
            } else {
                let Ok(conn) = handshake_v5(&self.socket) else {
                    continue;
                };
                if let Some(callback) = &self.on_connect {
                    callback(&conn);
                }
                self.connections.insert(addr, conn);
            }
        }
    }
}
