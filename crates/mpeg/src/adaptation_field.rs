use bit::{Bit, Bits, from_bits};

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
    /// 1b + 42b?
    pub pcr: Option<u64>,
    /// 1b + 42b?
    pub opcr: Option<u64>,
    /// 1b + 8b?
    pub splice_countdown: Option<u8>,
    /// 1b + n?
    pub transport_private_data: Option<Vec<u8>>,
    /// 1b + n?
    pub adaptation_field_extension_length: Option<u8>,
}

impl AdaptationField {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let adaptation_field_length = raw[0];

        let discontinuity_indicator = raw[1].bit(0);
        let random_access_indicator = raw[1].bit(1);
        let elementary_stream_priority_indicator = raw[1].bit(3);
        let flags = raw[1] & 0b0001_1111;

        let mut bit_offset: usize = 48;

        let pcr = flags
            .bit(3)
            .then(|| raw.bits::<u64>(bit_offset, 42))
            .inspect(|_| bit_offset += 42);

        let opcr = flags
            .bit(4)
            .then(|| raw.bits::<u64>(bit_offset, 42))
            .inspect(|_| bit_offset += 42);

        let splice_countdown = flags
            .bit(5)
            .then(|| from_bits::<u8>(raw, bit_offset, 8))
            .inspect(|_| bit_offset += 8);

        let transport_private_data = flags.bit(6).then(|| {
            let len = from_bits::<u8>(raw, bit_offset, 8);
            bit_offset += 8;
            tracing::warn!("Not implemented yet");
            bit_offset += usize::from(len);
            Vec::new()
        });

        let adaptation_field_extension_length = flags.bit(7).then(|| {
            tracing::warn!("Not implemented yet");
            0
        });

        Ok(Self {
            adaptation_field_length,
            discontinuity_indicator,
            random_access_indicator,
            elementary_stream_priority_indicator,
            pcr,
            opcr,
            splice_countdown,
            transport_private_data,
            adaptation_field_extension_length,
        })
    }

    pub fn size(&self) -> usize {
        self.adaptation_field_length as usize + 1
    }
}
