use anyhow::Result;

use crate::constants::descriptor_tags::MPEG4_VIDEO_DESCRIPTOR;

#[derive(Debug)]
pub struct Mpeg4VideoDescriptor {
    pub mpeg4_visual_profile_and_level: u8,
}

impl Mpeg4VideoDescriptor {
    pub const DESCRIPTOR_LENGTH: u8 = 1;
    pub const DESCRIPTOR_TAG: u8 = MPEG4_VIDEO_DESCRIPTOR;

    /// # Errors
    /// Error while parsing raw bytes
    pub fn deserialize(raw: &[u8]) -> Result<Self> {
        Ok(Self {
            mpeg4_visual_profile_and_level: raw[2],
        })
    }

    pub fn size(&self) -> usize {
        Self::DESCRIPTOR_LENGTH as usize + 2
    }
}
