#[derive(Debug)]
pub struct PrivateDataIndicatorDescriptor {
    pub descriptor_length: u8,
    pub private_data_indicator: u32,
}

impl PrivateDataIndicatorDescriptor {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            descriptor_length: raw[1],
            private_data_indicator: u32::from_be_bytes(raw[2..6].try_into()?),
        })
    }
}
