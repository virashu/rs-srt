use bit::{Bit, Bits};

#[derive(Clone, Debug)]
pub struct PesExtension {
    // Optional fields
    /// 128b?
    pub pes_private_data: Option<u128>,
    /// 8b?
    pub pack_header_field: Option<u8>,
    /// 8b?
    pub program_packet_seq_cntr: Option<u8>,
    /// 16b?
    pub pstd_buffer: Option<u16>,
    /// n?
    pub pes_extension_field_data: Option<Vec<u8>>,
}

impl PesExtension {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }
}

#[derive(Debug)]
pub enum OriginalOrCopy {
    Original,
    Copy,
}

impl From<bool> for OriginalOrCopy {
    fn from(value: bool) -> Self {
        if value { Self::Original } else { Self::Copy }
    }
}

#[derive(Debug)]
pub enum PtsDts {
    Pts { pts: u64 },
    PtsDts { pts: u64, dts: u64 },
}

impl PtsDts {
    pub fn pts_from_raw(raw: &[u8]) -> Self {
        let mut pts = 0;

        pts |= raw.bits::<u64>(4, 3) << 29;
        pts |= raw.bits::<u64>(8, 15) << 15;
        pts |= raw.bits::<u64>(16, 15);

        Self::Pts { pts }
    }

    pub fn pts_dts_from_raw(raw: &[u8]) -> Self {
        let mut pts = 0;

        pts |= raw.bits::<u64>(4, 3) << 29;
        pts |= raw.bits::<u64>(8, 15) << 15;
        pts |= raw.bits::<u64>(16, 15);

        let mut dts = 0;

        dts |= raw.bits::<u64>(44, 3) << 29;
        dts |= raw.bits::<u64>(48, 15) << 15;
        dts |= raw.bits::<u64>(66, 15);

        Self::PtsDts { pts, dts }
    }

    pub fn pts(&self) -> u64 {
        match self {
            PtsDts::Pts { pts } | PtsDts::PtsDts { pts, .. } => *pts,
        }
    }
}

#[derive(Debug)]
pub struct PesHeader {
    // 2b const
    /// 2b
    pub pes_scrambling_control: u8,
    /// 1b
    pub pes_priority: bool,
    /// 1b
    pub data_alignment_indicator: bool,
    /// 1b
    pub copyright: bool,
    /// 1b
    pub original_or_copy: OriginalOrCopy,
    // (8b flags)
    /// 8b
    pub pes_header_data_length: u8,

    // Optional fields
    /// (2b) + {0,40,80}b
    pub pts_dts: Option<PtsDts>,
    /// (1b) + 42b?
    pub escr: Option<u64>,
    /// (1b) + 22b?
    pub es_rate: Option<u32>,
    /// (1b) + 8b?
    pub dsm_trick_mode: Option<u8>,
    /// (1b) + 7b?
    pub additional_copy_info: Option<u8>,
    /// (1b) + 16b?
    pub previous_pes_crc: Option<u16>,
    /// (1b) + n?
    pub pes_extension: Option<PesExtension>,
}

impl PesHeader {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let pes_scrambling_control = raw[0] & 0b0011_0000 >> 4;
        let pes_priority = raw.bit(4);
        let data_alignment_indicator = raw.bit(5);
        let copyright = raw.bit(6);
        let original_or_copy = raw.bit(7).into();
        let flags = raw[1];
        let pes_header_data_length = raw[2];

        let mut offset: usize = 3;

        let pts_dts = flags
            .bit(1)
            .then(|| {
                if flags.bit(0) {
                    PtsDts::pts_from_raw(&raw[offset..])
                } else {
                    PtsDts::pts_dts_from_raw(&raw[offset..])
                }
            })
            .inspect(|x| match x {
                PtsDts::Pts { .. } => offset += 5,
                PtsDts::PtsDts { .. } => offset += 10,
            });

        let escr = raw[1]
            .bit(2)
            .then(|| {
                let mut res = 0;
                res |= raw[offset..].bits::<u64>(4, 3);
                res |= raw[offset..].bits::<u64>(8, 15);
                res |= raw[offset..].bits::<u64>(16, 15);
                res
            })
            .inspect(|_| offset += 6);

        let es_rate = raw[1]
            .bit(3)
            .then(|| raw[offset..].bits::<u32>(1, 22))
            .inspect(|_| offset += 3);

        let dsm_trick_mode = raw[1].bit(4).then(|| raw[offset]).inspect(|_| offset += 1);

        let additional_copy_info = raw[1].bit(5).then(|| raw[offset]).inspect(|_| offset += 1);

        let previous_pes_crc = raw[1]
            .bit(6)
            .then(|| raw[offset..].bits::<u16>(0, 16))
            .inspect(|_| offset += 2);

        // let pes_extension = raw[1]
        //     .bit(7)
        //     .then(|| PesExtension::from_raw(&raw[offset..]))
        //     .transpose()?;

        Ok(Self {
            pes_scrambling_control,
            pes_priority,
            data_alignment_indicator,
            copyright,
            original_or_copy,
            pes_header_data_length,
            pts_dts,
            escr,
            es_rate,
            dsm_trick_mode,
            additional_copy_info,
            previous_pes_crc,
            pes_extension: None,
        })
    }

    /// Bytes
    pub fn size(&self) -> usize {
        let mut size = 3;

        size += match self.pts_dts {
            Some(PtsDts::Pts { .. }) => 5,
            Some(PtsDts::PtsDts { .. }) => 10,
            None => 0,
        };

        self.escr.inspect(|_| size += 48);
        self.es_rate.inspect(|_| size += 24);
        self.dsm_trick_mode.inspect(|_| size += 1);
        self.additional_copy_info.inspect(|_| size += 1);
        self.previous_pes_crc.inspect(|_| size += 2);
        self.pes_extension.as_ref().inspect(|p| size += p.size());

        size
    }
}
