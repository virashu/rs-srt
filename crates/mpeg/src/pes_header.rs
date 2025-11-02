use bit::{Bit, from_bits};

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
    /// (2b) + 33b?
    pub pts_dts: Option<u64>,
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

        let pes_header_data_length = raw[2];

        let mut bit_offset: usize = 24;

        let pts_dts = raw[1]
            .bit(0)
            .then(|| from_bits::<u64>(raw, bit_offset, 33))
            .inspect(|_| bit_offset += 33);

        let escr = raw[1]
            .bit(1)
            .then(|| from_bits::<u64>(raw, bit_offset, 42))
            .inspect(|_| bit_offset += 42);

        let es_rate = raw[1]
            .bit(2)
            .then(|| from_bits::<u32>(raw, bit_offset, 22))
            .inspect(|_| bit_offset += 22);

        let dsm_trick_mode = raw[1]
            .bit(3)
            .then(|| from_bits::<u8>(raw, bit_offset, 8))
            .inspect(|_| bit_offset += 8);

        let additional_copy_info = raw[1]
            .bit(4)
            .then(|| from_bits::<u8>(raw, bit_offset, 7))
            .inspect(|_| bit_offset += 7);

        let previous_pes_crc = raw[1]
            .bit(5)
            .then(|| from_bits::<u16>(raw, bit_offset, 16))
            .inspect(|_| bit_offset += 16);

        // let pes_extension = raw[1]
        //     .bit(6)
        //     .then(|| PesExtension::from_raw(&raw[bit_offset..]))
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
        let mut size = 2;

        self.pts_dts.inspect(|_| size += 33);
        self.escr.inspect(|_| size += 42);
        self.es_rate.inspect(|_| size += 22);
        self.dsm_trick_mode.inspect(|_| size += 8);
        self.additional_copy_info.inspect(|_| size += 7);
        self.previous_pes_crc.inspect(|_| size += 16);
        self.pes_extension.as_ref().inspect(|p| size += p.size());

        size
    }
}
