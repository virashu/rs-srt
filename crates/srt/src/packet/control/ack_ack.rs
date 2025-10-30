use crate::packet::control::control_types;

#[derive(Clone, Debug)]
pub struct AckAck {
    pub ack_number: u32,
}

impl AckAck {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let ack_number = u32::from_be_bytes(raw[4..8].try_into()?);

        Ok(Self {
            ack_number,
        })
    }

    /// 8 BYTES
    pub fn raw_header(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend((control_types::ACKACK | (1 << 15)).to_be_bytes()); // Control Flag + Control Type
        res.extend(0u16.to_be_bytes()); // Reserved
        res.extend(self.ack_number.to_be_bytes()); // Acknowledgement Number

        res
    }
}
