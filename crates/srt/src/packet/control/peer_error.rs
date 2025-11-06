use anyhow::Result;

use crate::packet::control::control_types;

#[derive(Clone, Debug)]
pub struct PeerError {
    pub error_code: u32,
}

impl PeerError {
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let error_code = u32::from_be_bytes(raw[4..8].try_into()?);

        Ok(Self { error_code })
    }

    pub fn raw_header(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend((control_types::PEER_ERROR | (1 << 15)).to_be_bytes()); // Control Flag + Control Type
        res.extend(0u16.to_be_bytes()); // Reserved
        res.extend(self.error_code.to_be_bytes());

        res
    }
}
