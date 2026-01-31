use anyhow::{Result, bail};
use bit::{Bit, Bits};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct PesExtension {
    // Optional fields
    pub pes_private_data: Option<u128>,
    pub pack_header_field: Option<u8>,
    pub program_packet_seq_cntr: Option<u8>,
    pub pstd_buffer: Option<u16>,
    pub pes_extension_field_data: Option<Vec<u8>>,
}

impl PesExtension {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
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
    pub fn pts_from_raw(raw: [u8; 5]) -> Self {
        let pts_check = raw.bits::<u8>(0, 4);
        assert_eq!(pts_check, 0b0010);

        let mut pts = 0;
        pts |= raw.bits::<u64>(4, 3) << 29;
        pts |= raw.bits::<u64>(8, 15) << 15;
        pts |= raw.bits::<u64>(16, 15);

        Self::Pts { pts }
    }

    pub fn pts_dts_from_raw(raw: [u8; 10]) -> Self {
        let pts_check = raw.bits::<u8>(0, 4);
        assert_eq!(pts_check, 0b0011);

        let mut pts = 0;
        pts |= raw.bits::<u64>(4, 3) << 29;
        pts |= raw.bits::<u64>(8, 15) << 15;
        pts |= raw.bits::<u64>(24, 15);

        let pts_check = raw.bits::<u8>(40, 4);
        assert_eq!(pts_check, 0b0001);

        let mut dts = 0;
        dts |= raw.bits::<u64>(44, 3) << 29;
        dts |= raw.bits::<u64>(48, 15) << 15;
        dts |= raw.bits::<u64>(64, 15);

        Self::PtsDts { pts, dts }
    }

    pub fn pts(&self) -> u64 {
        match self {
            PtsDts::Pts { pts } | PtsDts::PtsDts { pts, .. } => *pts,
        }
    }

    pub fn dts(&self) -> Option<u64> {
        match self {
            PtsDts::Pts { .. } => None,
            PtsDts::PtsDts { dts, .. } => Some(*dts),
        }
    }
}

#[derive(Debug)]
pub struct PesHeader {
    pub pes_scrambling_control: u8,
    pub pes_priority: bool,
    pub data_alignment_indicator: bool,
    pub copyright: bool,
    pub original_or_copy: OriginalOrCopy,
    pub pes_header_data_length: u8,

    // Optional fields
    pub pts_dts: Option<PtsDts>,
    pub escr: Option<u64>,
    pub es_rate: Option<u32>,
    pub dsm_trick_mode: Option<u8>,
    pub additional_copy_info: Option<u8>,
    pub previous_pes_crc: Option<u16>,
    pub pes_extension: Option<PesExtension>,
}

impl PesHeader {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn deserialize(raw: &[u8]) -> Result<Self> {
        let mut iter = raw.iter().copied();

        // Octet 0
        let octet_0 = iter.next().unwrap();
        // Bits [1..=2] = '10'
        let pes_scrambling_control = (octet_0 & 0b0011_0000) >> 4;
        let pes_priority = octet_0.bit(4);
        let data_alignment_indicator = octet_0.bit(5);
        let copyright = octet_0.bit(6);
        let original_or_copy = octet_0.bit(7).into();

        // Octet 1
        let octet_1 = iter.next().unwrap();
        let flags_pts_dts = (octet_1 & 0b1100_0000) >> 6;
        let flag_escr = octet_1.bit(2);
        let flag_es_rate = octet_1.bit(3);
        let flag_dsm_trick_mode = octet_1.bit(4);
        let flag_additional_copy_info = octet_1.bit(5);
        let flag_pes_crc = octet_1.bit(6);
        let flag_pes_extension = octet_1.bit(7);

        // Octet 2
        let pes_header_data_length = iter.next().unwrap();

        let pts_dts = match flags_pts_dts {
            0b10 => Some(PtsDts::pts_from_raw(iter.next_array::<5>().unwrap())),
            0b11 => Some(PtsDts::pts_dts_from_raw(iter.next_array::<10>().unwrap())),
            0b01 => bail!("Illegal"),
            _ => None,
        };

        let escr = flag_escr.then(|| {
            let raw_num = iter.next_array::<6>().unwrap();
            let mut res = 0;
            res |= raw_num.bits::<u64>(2, 3) << 29;
            res |= raw_num.bits::<u64>(6, 15) << 15;
            res |= raw_num.bits::<u64>(22, 15);
            // TODO: ESCR_extension
            res
        });

        let es_rate = flag_es_rate.then(|| iter.next_array::<3>().unwrap().bits::<u32>(1, 22));

        let dsm_trick_mode = flag_dsm_trick_mode.then(|| iter.next().unwrap());

        let additional_copy_info = flag_additional_copy_info.then(|| iter.next().unwrap());

        let previous_pes_crc =
            flag_pes_crc.then(|| iter.next_array::<2>().unwrap().bits::<u16>(0, 16));

        let pes_extension = flag_pes_extension
            .then(|| PesExtension::from_raw(&iter.collect::<Vec<_>>()))
            .transpose()?;

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
            pes_extension,
        })
    }

    /// Get size of raw content in bytes
    pub fn size(&self) -> usize {
        let mut size = 3;

        size += match self.pts_dts {
            Some(PtsDts::Pts { .. }) => 5,
            Some(PtsDts::PtsDts { .. }) => 10,
            None => 0,
        };

        self.escr.inspect(|_| size += 6);
        self.es_rate.inspect(|_| size += 3);
        self.dsm_trick_mode.inspect(|_| size += 1);
        self.additional_copy_info.inspect(|_| size += 1);
        self.previous_pes_crc.inspect(|_| size += 2);
        self.pes_extension.as_ref().inspect(|p| size += p.size());

        size
    }
}
