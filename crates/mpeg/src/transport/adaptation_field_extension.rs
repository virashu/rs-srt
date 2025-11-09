use anyhow::Result;
use bit::{Bit, Bits};

#[derive(Debug)]
pub struct AdaptationFieldExtensionLtw {
    pub valid_flag: bool,
    pub offset: u16,
}

#[derive(Debug)]
pub struct AdaptationFieldExtensionSeamlessSplice {
    pub splice_type: u8,
    pub dts_next_au: u64,
}

#[derive(Debug)]
pub struct AdaptationFieldExtension {
    length: u8,

    // Optional fields
    pub ltw: Option<AdaptationFieldExtensionLtw>,
    pub piecewise_rate: Option<u32>,
    pub splice_type: Option<AdaptationFieldExtensionSeamlessSplice>,
}

impl AdaptationFieldExtension {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let length = raw[0];
        let flags = raw[1];

        let mut offset = 2;

        let ltw = flags
            .bit(0)
            .then(|| AdaptationFieldExtensionLtw {
                valid_flag: raw[offset..].bit(0),
                offset: raw[offset..].bits::<u16>(1, 15),
            })
            .inspect(|_| offset += 2);

        let piecewise_rate = flags
            .bit(1)
            .then(|| raw[offset..].bits::<u32>(2, 22))
            .inspect(|_| offset += 3);

        let splice_type = flags
            .bit(2)
            .then(|| {
                let splice_type = raw[offset..].bits::<u8>(0, 4);

                let mut dts_next_au = 0;
                dts_next_au |= raw[offset..].bits::<u64>(4, 3) << 29;
                dts_next_au |= raw[offset..].bits::<u64>(8, 15) << 15;
                dts_next_au |= raw[offset..].bits::<u64>(16, 15);

                AdaptationFieldExtensionSeamlessSplice {
                    splice_type,
                    dts_next_au,
                }
            })
            .inspect(|_| offset += 5);

        Ok(Self {
            length,
            ltw,
            piecewise_rate,
            splice_type,
        })
    }

    pub fn size(&self) -> usize {
        self.length as usize + 1
    }
}
