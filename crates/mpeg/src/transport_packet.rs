use crate::{adaptation_field::AdaptationField, header::Header, payload::Payload};

#[derive(Debug)]
pub enum AdaptationFieldOption {
    None,
    Empty,
    Some(AdaptationField),
}

#[derive(Debug)]
pub struct TransportPacket {
    pub header: Header,
    /// 16b+
    pub adaptation_field: AdaptationFieldOption,
    /// n
    pub payload: Option<Payload>,
}

impl TransportPacket {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let header = Header::from_raw(raw)?;

        let adaptation_field = if header.adaptation_field_control & 0b10 == 0 {
            AdaptationFieldOption::None
        } else if raw[4] == 0 {
            AdaptationFieldOption::Empty
        } else {
            AdaptationFieldOption::Some(AdaptationField::from_raw(&raw[4..])?)
        };

        let payload = if header.adaptation_field_control & 0b01 != 0 {
            let offset = if let AdaptationFieldOption::Some(a) = &adaptation_field {
                a.size()
            } else if let AdaptationFieldOption::Empty = &adaptation_field {
                1
            } else {
                0
            };

            let payload_body = &raw[(4 + offset)..];
            if header.payload_unit_start_indicator {
                // Contains PES or PSI
                Some(Payload::pes_from_raw(payload_body)?)
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
