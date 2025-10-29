pub mod control;
pub mod data;

use crate::packet::{control::ControlInformation, data::DataPacket};

#[derive(Debug)]
pub enum PacketContent {
    Data(DataPacket),
    Control(ControlInformation),
}

impl PacketContent {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let packet_type = (raw[0] & 0b1000_0000) >> 7;

        Ok(match packet_type {
            1 => Self::Control(ControlInformation::from_raw(raw)?),
            0 => Self::Data(DataPacket::from_raw(raw)?),
            _ => unreachable!(),
        })
    }

    pub fn raw_header(&self) -> Vec<u8> {
        match self {
            Self::Control(p) => p.raw_header(),
            Self::Data(p) => p.raw_header(),
        }
    }

    pub fn raw_contnet(&self) -> Vec<u8> {
        match self {
            Self::Control(p) => p.raw_content(),
            Self::Data(p) => p.raw_content(),
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub timestamp: u32,
    pub dest_socket_id: u32,

    pub content: PacketContent,
}

impl Packet {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let timestamp = u32::from_be_bytes(raw[8..12].try_into()?);
        let dest_socket_id = u32::from_be_bytes(raw[12..16].try_into()?);
        let content = PacketContent::from_raw(raw)?;

        Ok(Self {
            timestamp,
            dest_socket_id,
            content,
        })
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend(self.content.raw_header());
        res.extend(self.timestamp.to_be_bytes());
        res.extend(self.dest_socket_id.to_be_bytes());
        res.extend(self.content.raw_contnet());

        res
    }
}
