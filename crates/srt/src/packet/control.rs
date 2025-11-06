use crate::packet::control::{
    ack::Ack, ack_ack::AckAck, drop_req::DropReq, handshake::Handshake, nak::Nak,
    peer_error::PeerError,
};

// Control Information Field of different Types
pub mod ack;
pub mod ack_ack;
pub mod drop_req;
pub mod handshake;
pub mod nak;
pub mod peer_error;

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

/// Contains `Type`, `Subtype`, `Type-specific Information`, `CIF`
#[derive(Clone, Debug)]
pub enum ControlPacketInfo {
    Handshake(Handshake),
    KeepAlive,
    Ack(Ack),
    Nak(Nak),
    CongestionWarning,
    Shutdown,
    AckAck(AckAck),
    DropReq(DropReq),
    PeerError(PeerError),
    Other,
}

impl ControlPacketInfo {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let control_type = u16::from_be_bytes(raw[0..2].try_into()?) & !(1 << 15);
        let _subtype = u16::from_be_bytes(raw[2..4].try_into()?);
        let _type_specific = &raw[4..8];

        Ok(match control_type {
            control_types::HANDSHAKE => Self::Handshake(Handshake::from_raw_cif(&raw[16..])?),
            control_types::KEEPALIVE => Self::KeepAlive,
            control_types::ACK => Self::Ack(Ack::from_raw(raw)?),
            control_types::NAK => todo!("Nak"),
            control_types::CONGESTION_WARNING => todo!("CongestionWarning"),
            control_types::SHUTDOWN => Self::Shutdown,
            control_types::ACKACK => Self::AckAck(AckAck::from_raw(raw)?),
            control_types::DROPREQ => Self::DropReq(DropReq::from_raw(raw)?),
            control_types::PEERERROR => todo!("PeerError"),
            control_types::OTHER => todo!("Other"),

            _ => unreachable!(),
        })
    }

    /// Get control header (`Type`, `Subtype` and `Type-specific Information`) with `Subtype` and `Type-specific Information` fields equal to 0.
    /// (For types, that don't have info in those fields)
    fn empty_header_with_type(r#type: u16) -> Vec<u8> {
        [(r#type | (1 << 15)).to_be_bytes(), [0, 0], [0, 0], [0, 0]].concat()
    }

    /// Get `Type`, `Subtype` and `Type-specific Information`
    pub fn raw_header(&self) -> Vec<u8> {
        match self {
            Self::Handshake(_) => Self::empty_header_with_type(control_types::HANDSHAKE),
            Self::KeepAlive => Self::empty_header_with_type(control_types::KEEPALIVE),
            Self::Ack(ack) => ack.raw_header(),
            Self::Nak(_) => Self::empty_header_with_type(control_types::NAK),
            Self::CongestionWarning => {
                Self::empty_header_with_type(control_types::CONGESTION_WARNING)
            }
            Self::Shutdown => Self::empty_header_with_type(control_types::SHUTDOWN),
            Self::AckAck(ack_ack) => ack_ack.raw_header(),
            Self::DropReq(drop_req) => drop_req.raw_header(),
            Self::PeerError(peer_error) => peer_error.raw_header(),
            Self::Other => Self::empty_header_with_type(control_types::OTHER),
        }
    }

    /// Get `Control Information Field (CIF)`
    pub fn raw_content(&self) -> Vec<u8> {
        match self {
            Self::Handshake(h) => h.raw_content(),
            Self::Ack(ack) => ack.raw_content(),
            Self::Nak(nak) => nak.raw_content(),
            Self::DropReq(_) => todo!(),
            Self::Other => todo!(),

            // Other types don't have CIF
            _ => Vec::new(),
        }
    }
}
