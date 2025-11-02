use bit::{Bit, Bits};

/// 2B
#[derive(Debug)]
pub struct AdaptationFieldExtensionLtw {
    /// 1b
    pub ltw_valid_flag: bool,
    /// 15b
    pub ltw_offset: u16,
}

/// 5B
#[derive(Debug)]
pub struct AdaptationFieldExtensionSeamlessSplice {
    // 3b (reserved)
    /// 4b
    pub splice_type: u8,
    /// 33b
    pub dts_next_au: u64,
}

/// 2-12B+
#[derive(Debug)]
pub struct AdaptationFieldExtension {
    /// 1B
    pub adaptation_field_extension_length: u8,
    // 5b (flags)
    // Optional fields
    /// (1b) + 2B?
    pub ltw: Option<AdaptationFieldExtensionLtw>,
    /// (1b) + 3B?
    pub piecewise_rate: Option<u32>,
    /// (1b) + 5B?
    pub splice_type: Option<AdaptationFieldExtensionSeamlessSplice>,
}

impl AdaptationFieldExtension {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let adaptation_field_extension_length = raw[0];
        let flags = raw[1];

        let mut offset = 2;

        let ltw = flags
            .bit(0)
            .then(|| AdaptationFieldExtensionLtw {
                ltw_valid_flag: raw[offset..].bit(0),
                ltw_offset: raw[offset..].bits::<u16>(offset, 15),
            })
            .inspect(|_| offset += 2);

        let piecewise_rate = flags
            .bit(2)
            .then(|| raw[offset..].bits::<u32>(2, 22))
            .inspect(|_| offset += 5);

        let splice_type = flags
            .bit(2)
            .then(|| {
                let splice_type = raw[offset..].bits::<u8>(0, 4);

                let mut dts_next_au = 0;
                dts_next_au |= raw[offset..].bits::<u64>(4, 3);
                dts_next_au |= raw[offset..].bits::<u64>(8, 15);
                dts_next_au |= raw[offset..].bits::<u64>(16, 15);

                AdaptationFieldExtensionSeamlessSplice {
                    splice_type,
                    dts_next_au,
                }
            })
            .inspect(|_| offset += 5);

        Ok(Self {
            adaptation_field_extension_length,
            ltw,
            piecewise_rate,
            splice_type,
        })
    }
}
