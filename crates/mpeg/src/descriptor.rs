use crate::descriptor::{
    mpeg4_video::Mpeg4VideoDescriptor, private_data_indicator::PrivateDataIndicatorDescriptor,
};

pub mod mpeg4_video;
pub mod private_data_indicator;

#[derive(Debug)]
pub enum Descriptor {
    // 15
    PrivateDataIndicator(PrivateDataIndicatorDescriptor),
    // 27
    Mpeg4Video(Mpeg4VideoDescriptor),
}
