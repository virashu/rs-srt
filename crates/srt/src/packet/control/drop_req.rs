use anyhow::Result;

#[derive(Clone, Debug)]
pub struct DropReq {
    pub message_number: u32,
    pub first_packet_sequence_number: u32,
    pub last_packet_sequence_number: u32,
}

impl DropReq {
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let message_number = u32::from_be_bytes(raw[4..8].try_into()?);

        let first_packet_sequence_number = u32::from_be_bytes(raw[16..20].try_into()?);
        let last_packet_sequence_number = u32::from_be_bytes(raw[20..24].try_into()?);

        Ok(Self {
            message_number,
            first_packet_sequence_number,
            last_packet_sequence_number,
        })
    }

    pub fn raw_header(&self) -> Vec<u8> {
        todo!()
    }
}
