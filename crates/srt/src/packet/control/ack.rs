//! <https://datatracker.ietf.org/doc/html/draft-sharabayko-srt#section-3.2.4>

use crate::packet::control::control_types;

#[derive(Clone, Debug)]
pub struct Ack {
    // Header
    pub ack_number: u32,

    // CIF
    pub last_ackd_packet_sequence_number: u32,
    pub rtt: u32,
    pub rtt_variance: u32,
    pub available_buffer_size: u32,
    pub packets_receiving_rate: u32,
    pub estimated_link_capacity: u32,
    pub receiving_rate: u32,
}

impl Ack {
    /// 44 BYTES
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let ack_number = u32::from_be_bytes(raw[4..8].try_into()?);

        let last_ackd_packet_sequence_number = u32::from_be_bytes(raw[16..20].try_into()?);
        let rtt = u32::from_be_bytes(raw[20..24].try_into()?);
        let rtt_variance = u32::from_be_bytes(raw[24..28].try_into()?);
        let available_buffer_size = u32::from_be_bytes(raw[28..32].try_into()?);
        let packets_receiving_rate = u32::from_be_bytes(raw[32..36].try_into()?);
        let estimated_link_capacity = u32::from_be_bytes(raw[36..40].try_into()?);
        let receiving_rate = u32::from_be_bytes(raw[40..44].try_into()?);

        Ok(Self {
            ack_number,
            last_ackd_packet_sequence_number,
            rtt,
            rtt_variance,
            available_buffer_size,
            packets_receiving_rate,
            estimated_link_capacity,
            receiving_rate,
        })
    }

    /// 8 BYTES
    pub fn raw_header(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend((control_types::ACK | (1 << 15)).to_be_bytes()); // Control Flag + Control Type
        res.extend(0u16.to_be_bytes()); // Reserved
        res.extend(self.ack_number.to_be_bytes()); // Acknowledgement Number

        res
    }

    /// BYTES (full)
    pub fn raw_content(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend(self.last_ackd_packet_sequence_number.to_be_bytes());
        res.extend(self.rtt.to_be_bytes());
        res.extend(self.rtt_variance.to_be_bytes());
        res.extend(self.available_buffer_size.to_be_bytes());
        res.extend(self.packets_receiving_rate.to_be_bytes());
        res.extend(self.estimated_link_capacity.to_be_bytes());
        res.extend(self.receiving_rate.to_be_bytes());

        res
    }
}
