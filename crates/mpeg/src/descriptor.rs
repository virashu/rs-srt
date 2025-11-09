use anyhow::Result;

use crate::{
    constants::descriptor_tags,
    descriptor::{
        mpeg4_video::Mpeg4VideoDescriptor, private_data_indicator::PrivateDataIndicatorDescriptor,
    },
};

pub mod mpeg4_video;
pub mod private_data_indicator;

#[derive(Debug)]
pub enum Descriptor {
    PrivateDataIndicator(PrivateDataIndicatorDescriptor),
    Mpeg4Video(Mpeg4VideoDescriptor),
}

impl Descriptor {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let descriptor_tag = raw[0];

        Ok(match descriptor_tag {
            descriptor_tags::MPEG4_VIDEO_DESCRIPTOR => {
                Self::Mpeg4Video(Mpeg4VideoDescriptor::from_raw(raw)?)
            }

            _ => todo!("Descriptor tag '{descriptor_tag}'"),
        })
    }

    pub fn size(&self) -> usize {
        match self {
            Descriptor::PrivateDataIndicator(d) => d.size(),
            Descriptor::Mpeg4Video(d) => d.size(),
        }
    }
}
