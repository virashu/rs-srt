use crate::constants::descriptor_tags::MPEG4_VIDEO_DESCRIPTOR;

#[derive(Debug)]
pub struct Mpeg4VideoDescriptor {
    pub descriptor_length: u8,
    pub mpeg4_visual_profile_and_level: u8,
}

impl Mpeg4VideoDescriptor {
    pub const DESCRIPTOR_TAG: u8 = MPEG4_VIDEO_DESCRIPTOR;

    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            descriptor_length: raw[1],
            mpeg4_visual_profile_and_level: raw[2],
        })
    }
}
