use anyhow::{Result, bail};
use bit::Bit;

use crate::transport::adaptation_field_control::AdaptationFieldControl;

#[derive(Debug)]
pub struct Header {
    pub packet_id: u16,

    pub transport_error: bool,
    pub payload_unit_start: bool,
    pub transport_priority: bool,

    pub transport_scrambling_control: u8,
    pub adaptation_field_control: AdaptationFieldControl,
    pub continuity_counter: u8,
}

impl Header {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        if raw[0] != 0x47 {
            bail!("Missing sync byte: {raw:?}");
        }

        // Raw numbers
        let transport_error_indicator = raw[1].bit(0);
        let payload_unit_start_indicator = raw[1].bit(1);
        let transport_priority = raw[1].bit(2);
        let packet_id = u16::from_be_bytes(raw[1..3].try_into()?) & 0b0001_1111_1111_1111;
        let transport_scrambling_control = (raw[3] & 0b1100_0000) >> 6;
        let adaptation_field_control = (raw[3] & 0b0011_0000) >> 4;
        let continuity_counter = raw[3] & 0b0000_1111;

        Ok(Self {
            packet_id,

            transport_error: transport_error_indicator,
            payload_unit_start: payload_unit_start_indicator,
            transport_priority,

            transport_scrambling_control,
            adaptation_field_control: AdaptationFieldControl::from_raw(adaptation_field_control)?,
            continuity_counter,
        })
    }

    pub const fn size(&self) -> usize {
        4
    }
}
