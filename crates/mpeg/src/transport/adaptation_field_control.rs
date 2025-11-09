use anyhow::{Result, anyhow};

#[derive(Debug)]
pub enum AdaptationFieldControl {
    AdaptationFieldOnly,
    PayloadOnly,
    AdaptationFieldAndPayload,
}

impl AdaptationFieldControl {
    /// # Errors
    /// Error while parsing raw data
    pub fn from_raw(raw: u8) -> Result<Self> {
        match raw {
            0b01 => Ok(Self::PayloadOnly),
            0b10 => Ok(Self::AdaptationFieldOnly),
            0b11 => Ok(Self::AdaptationFieldAndPayload),

            _ => Err(anyhow!("Invalid adaptation field control value: {raw}")),
        }
    }

    pub fn payload(&self) -> bool {
        !matches!(self, Self::AdaptationFieldOnly)
    }

    pub fn adaptation_field(&self) -> bool {
        !matches!(self, Self::PayloadOnly)
    }
}
