use anyhow::Result;
use bit::{Bit, Bits};

use crate::transport::adaptation_field_extension::AdaptationFieldExtension;

#[derive(Debug)]
pub struct AdaptationFieldContent {}

#[derive(Debug)]
pub struct AdaptationField {
    length: u8,

    pub discontinuity_indicator: bool,
    pub random_access_indicator: bool,
    pub elementary_stream_priority_indicator: bool,

    // Optional fields
    pub pcr: Option<u64>,
    pub opcr: Option<u64>,
    pub splice_countdown: Option<u8>,
    pub transport_private_data: Option<Vec<u8>>,
    pub adaptation_field_extension: Option<AdaptationFieldExtension>,
}

impl AdaptationField {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let length = raw[0];

        let discontinuity_indicator = raw[1].bit(0);
        let random_access_indicator = raw[1].bit(1);
        let elementary_stream_priority_indicator = raw[1].bit(3);
        let flags = raw[1] & 0b0001_1111;

        let mut offset: usize = 2;

        let pcr = flags
            .bit(3)
            .then(|| {
                let base = raw[offset..].bits::<u64>(0, 33);
                let ext = raw[offset..].bits::<u64>(39, 9);
                base * 300 + ext
            })
            .inspect(|_| offset += 6);

        let opcr = flags
            .bit(4)
            .then(|| {
                let base = raw[offset..].bits::<u64>(0, 33);
                let ext = raw[offset..].bits::<u64>(39, 9);
                base * 300 + ext
            })
            .inspect(|_| offset += 6);

        let splice_countdown = flags.bit(5).then(|| raw[offset]).inspect(|_| offset += 1);

        let transport_private_data = flags.bit(6).then(|| {
            let len = raw[offset] as usize;
            offset += 1;
            let data = Vec::from(&raw[offset..(offset + len)]);
            offset += len;
            data
        });

        let adaptation_field_extension = flags
            .bit(7)
            .then(|| AdaptationFieldExtension::from_raw(&raw[offset..]))
            .transpose()?;

        Ok(Self {
            length,
            discontinuity_indicator,
            random_access_indicator,
            elementary_stream_priority_indicator,
            pcr,
            opcr,
            splice_countdown,
            transport_private_data,
            adaptation_field_extension,
        })
    }

    pub fn size(&self) -> usize {
        self.length as usize + 1
    }
}
