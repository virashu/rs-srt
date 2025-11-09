use anyhow::Result;

use crate::{
    constants::packet_ids::GROUP_CONTROL,
    pes::packet::PesPacket,
    psi::packet::ProgramSpecificInformation,
    transport::{adaptation_field::AdaptationField, header::Header},
};

#[derive(Debug)]
pub enum AdaptationFieldOption {
    None,
    Empty,
    Some(AdaptationField),
}

impl AdaptationFieldOption {
    pub fn size(&self) -> usize {
        match self {
            AdaptationFieldOption::None => 0,
            AdaptationFieldOption::Empty => 1,
            AdaptationFieldOption::Some(a_f) => a_f.size(),
        }
    }
}

#[derive(Debug)]
pub enum Payload {
    PES(PesPacket),
    PSI(ProgramSpecificInformation),
    Data(Vec<u8>),
}

#[derive(Debug)]
pub struct TransportPacket {
    pub header: Header,
    pub adaptation_field: AdaptationFieldOption,
    pub payload: Option<Payload>,
}

impl TransportPacket {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8], pmt_packet_ids: &[u16]) -> Result<Self> {
        let header = Header::from_raw(raw)?;

        let adaptation_field = if !header.adaptation_field_control.adaptation_field() {
            AdaptationFieldOption::None
        } else if raw[4] == 0 {
            AdaptationFieldOption::Empty
        } else {
            AdaptationFieldOption::Some(AdaptationField::from_raw(&raw[4..])?)
        };

        let payload = if header.adaptation_field_control.payload() {
            let payload_body = &raw[(4 + adaptation_field.size())..];
            if header.payload_unit_start {
                // Contains PES or PSI
                if GROUP_CONTROL.contains(&header.pid) || pmt_packet_ids.contains(&header.pid) {
                    Some(Payload::PSI(ProgramSpecificInformation::from_raw(
                        payload_body,
                    )?))
                } else {
                    Some(Payload::PES(PesPacket::from_raw(payload_body)?))
                }
            } else {
                Some(Payload::Data(Vec::from(payload_body)))
            }
        } else {
            None
        };

        Ok(Self {
            header,
            adaptation_field,
            payload,
        })
    }
}
