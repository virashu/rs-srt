use anyhow::Result;

use crate::{constants::stream_ids::GROUP_NO_HEADER, pes::header::PesHeader};

#[derive(Debug)]
pub struct PesPacket {
    pub stream_id: u8,
    pes_packet_length: u16,
    pub pes_header: Option<PesHeader>,
    pub pes_data: Vec<u8>,
}

impl PesPacket {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        // Octets [0..=2] = Pack start code prefix (0x000001)
        let stream_id = raw[3];
        let pes_packet_length = u16::from_be_bytes(raw[4..6].try_into()?);

        let pes_header = (!GROUP_NO_HEADER.contains(&stream_id))
            .then(|| PesHeader::deserialize(&raw[6..]))
            .transpose()?;

        let data_start = 6 + pes_header.as_ref().map_or(0, PesHeader::size);
        let pes_data = Vec::from(&raw[data_start..]);

        Ok(Self {
            stream_id,
            pes_packet_length,
            pes_header,
            pes_data,
        })
    }
}
