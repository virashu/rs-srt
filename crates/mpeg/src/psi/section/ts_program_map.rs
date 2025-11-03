use bit::{Bit, Bits};

use crate::descriptor::{Descriptor, mpeg4_video::Mpeg4VideoDescriptor};

#[derive(Debug)]
pub struct ProgramDefinition {
    pub stream_type: u8,
    pub elementary_pid: u16,
    pub es_info_length: u16,

    pub descriptors: Vec<Descriptor>,
}

impl ProgramDefinition {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let stream_type = raw[0];
        let elementary_pid = u16::from_be_bytes(raw[1..3].try_into()?) & !(0b111 << 13);
        let es_info_length = u16::from_be_bytes(raw[3..5].try_into()?) & !(0b1111 << 12);

        let mut descriptors = Vec::new();

        Ok(Self {
            stream_type,
            elementary_pid,
            es_info_length,
            descriptors,
        })
    }
}

#[derive(Debug)]
pub struct TsProgramMapSection {
    pub table_id: u8,
    pub section_syntax_indicator: bool,
    pub section_length: u16,
    pub program_number: u16,
    pub version_number: u8,
    pub current_next_indicator: bool,
    pub section_number: u8,
    pub last_section_number: u8,

    pub pcr_pid: u16,
    pub program_info_length: u16,

    pub program_info: Vec<Mpeg4VideoDescriptor>,
    pub program_definitions: Vec<ProgramDefinition>,

    pub crc_32: u32,
}

impl TsProgramMapSection {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_MPEG_2);

        let table_id = raw[0];
        let section_syntax_indicator = raw[1].bit(0);
        let section_length = u16::from_be_bytes(raw[1..3].try_into()?) & !(0b11_11_11 << 10);
        let program_number = u16::from_be_bytes(raw[3..5].try_into()?);
        let version_number = raw[5..].bits::<u8>(2, 5);
        let current_next_indicator = raw[5..].bit(7);
        let section_number = raw[6];
        let last_section_number = raw[7];

        let pcr_pid = u16::from_be_bytes(raw[8..10].try_into()?) & !(0b111 << 13);

        // Program info (descriptors)
        let program_info_length = u16::from_be_bytes(raw[10..12].try_into()?) & !(0b1111 << 12);
        let descriptors_count = program_info_length / 3;
        let mut program_info = Vec::new();
        for i in 0..descriptors_count {
            let offset = 12 + (i * 3) as usize;
            let program = Mpeg4VideoDescriptor::from_raw(&raw[offset..])?;
            program_info.push(program);
        }

        let mut offset = 12 + program_info_length as usize;

        let mut program_definitions = Vec::new();
        while offset < (section_length as usize - 1) {
            let def = ProgramDefinition::from_raw(&raw[offset..(section_length as usize - 1)])?;
            offset += def.es_info_length as usize + 5;
            program_definitions.push(def);
        }

        // Check CRC
        let chksum_provided = raw[(section_length as usize - 1)..].bits::<u32>(0, 32);
        let chksum_calculated = CRC.checksum(&raw[0..(section_length as usize - 1)]);
        if chksum_calculated != chksum_provided {
            return Err(anyhow::anyhow!(
                "Checksum does not match: {chksum_calculated} != {chksum_provided}"
            ));
        }

        Ok(Self {
            table_id,
            section_syntax_indicator,
            section_length,
            program_number,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
            pcr_pid,
            program_info_length,
            program_info,
            program_definitions,
            crc_32: chksum_provided,
        })
    }
}
