use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
};

use anyhow::Result;

use crate::{
    connection::Connection,
    packet::{Packet, PacketContent, control::ControlPacketInfo},
};

const MAX_PACK_SIZE: usize = 1500;

type OnConnectHandler = dyn Fn(&Connection);
type OnDiscnnectHandler = dyn Fn(&Connection);
pub type OnDataHandler = dyn Fn(&Connection, &[u8]);

pub struct Server<'c> {
    socket: UdpSocket,
    connections: HashMap<SocketAddr, Connection<'c>>,

    on_connect: Option<Box<OnConnectHandler>>,
    on_disconnect: Option<Box<OnDiscnnectHandler>>,
    on_data: Option<Box<OnDataHandler>>,
}

impl<'c> Server<'c> {
    pub fn new<A>(addr: A) -> anyhow::Result<Self>
    where
        A: ToSocketAddrs,
    {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self {
            socket,
            connections: HashMap::new(),
            on_connect: None,
            on_disconnect: None,
            on_data: None,
        })
    }

    pub fn on_connect(&mut self, f: impl Fn(&Connection) + 'static) {
        self.on_connect = Some(Box::new(f));
    }

    pub fn on_disconnect(&mut self, f: impl Fn(&Connection) + 'static) {
        self.on_disconnect = Some(Box::new(f));
    }

    pub fn on_data(&mut self, f: impl Fn(&Connection, &[u8]) + 'static) {
        self.on_data = Some(Box::new(f));
    }

    fn recv(&self) -> anyhow::Result<(SocketAddr, Packet)> {
        let mut buf = [0; MAX_PACK_SIZE];

        let (n, addr) = self.socket.recv_from(&mut buf)?;
        let data = &buf[..n];
        let pack = Packet::from_raw(data)?;

        Ok((addr, pack))
    }

    pub fn run(&'c mut self) -> anyhow::Result<()> {
        loop {
            let (addr, pack) = self.recv()?;

            if let Some(conn) = self.connections.get(&addr) {
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

                conn.handle(&pack)?;
            } else {
                let Ok(conn): Result<Connection<'c>> =
                    Connection::establish_v5(&self.socket, self.on_data.as_deref())
                else {
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
