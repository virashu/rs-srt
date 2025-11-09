use anyhow::Result;

#[derive(Debug)]
pub struct PrivateDataIndicatorDescriptor {
    pub private_data_indicator: u32,
}

impl PrivateDataIndicatorDescriptor {
    pub const DESCRIPTOR_LENGTH: u8 = 4;

    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        Ok(Self {
            private_data_indicator: u32::from_be_bytes(raw[2..6].try_into()?),
        })
    }

    pub const fn size(&self) -> usize {
        Self::DESCRIPTOR_LENGTH as usize + 2
    }
}
