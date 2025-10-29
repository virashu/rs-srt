use crate::packet::control::handshake::Handshake;

pub mod handshake;

pub mod control_types {
    pub const HANDSHAKE: u16 = 0x0000;
    pub const KEEPALIVE: u16 = 0x0001;
    pub const ACK: u16 = 0x0002;
    pub const NAK: u16 = 0x0003;
    pub const CONGESTION_WARNING: u16 = 0x0004;
    pub const SHUTDOWN: u16 = 0x0005;
    pub const ACKACK: u16 = 0x0006;
    pub const DROPREQ: u16 = 0x0007;
    pub const PEERERROR: u16 = 0x0008;
    pub const OTHER: u16 = 0x7FFF;
}

#[derive(Clone, Debug)]
pub enum ControlInformation {
    Handshake(Handshake),
}

impl ControlInformation {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let control_type = u16::from_be_bytes(raw[0..2].try_into()?) & !(1 << 15);
        let _subtype = u16::from_be_bytes(raw[2..4].try_into()?);

        // Data after package header
        let content = &raw[16..];

        Ok(match control_type {
            control_types::HANDSHAKE => Self::Handshake(Handshake::from_raw(content)?),
            _ => todo!(),
        })
    }

    pub fn raw_header(&self) -> Vec<u8> {
        match self {
            Self::Handshake(_) => [
                (control_types::HANDSHAKE | (1 << 15)).to_be_bytes(),
                [0, 0],
                [0, 0],
                [0, 0],
            ]
            .concat(),
        }
    }

    pub fn raw_content(&self) -> Vec<u8> {
        match self {
            Self::Handshake(h) => h.to_raw(),
        }
    }
}
