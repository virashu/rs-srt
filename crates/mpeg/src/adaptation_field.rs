use bit::{Bit, Bits, from_bits};

use crate::adaptation_field_extension::AdaptationFieldExtension;

#[derive(Debug)]
pub struct AdaptationFieldContent {}

/// 8-16b+
#[derive(Debug)]
pub struct AdaptationField {
    /// 8b
    pub adaptation_field_length: u8,
    /// 1b
    pub discontinuity_indicator: bool,
    /// 1b
    pub random_access_indicator: bool,
    /// 1b
    pub elementary_stream_priority_indicator: bool,
    /// 1b + 48b?
    pub pcr: Option<u64>,
    /// 1b + 48b?
    pub opcr: Option<u64>,
    /// 1b + 8b?
    pub splice_countdown: Option<u8>,
    /// 1b + [1 + n]?
    pub transport_private_data: Option<Vec<u8>>,
    /// 1b + n?
    pub adaptation_field_extension: Option<AdaptationFieldExtension>,
}

impl AdaptationField {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let adaptation_field_length = raw[0];

        let discontinuity_indicator = raw[1].bit(0);
        let random_access_indicator = raw[1].bit(1);
        let elementary_stream_priority_indicator = raw[1].bit(3);
        let flags = raw[1] & 0b0001_1111;

        let mut offset: usize = 48;

        let pcr = flags.bit(3).then(|| 0).inspect(|_| offset += 6);

        let opcr = flags.bit(4).then(|| 0).inspect(|_| offset += 6);

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
            adaptation_field_length,
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
        self.adaptation_field_length as usize + 1
    }
}
