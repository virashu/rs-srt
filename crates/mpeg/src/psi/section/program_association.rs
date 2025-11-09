use anyhow::{Result, anyhow};
use bit::{Bit, Bits};

#[derive(Debug)]
pub struct ProgramAssociation {
    pub program_number: u16,
    pub program_id: u16,
}

#[derive(Debug)]
pub struct ProgramAssociationSection {
    pub table_id: u8,
    pub section_syntax_indicator: bool,
    section_length: u16,
    pub transport_stream_id: u16,
    pub version_number: u8,
    pub current_next_indicator: bool,
    pub section_number: u8,
    pub last_section_number: u8,

    pub programs: Vec<ProgramAssociation>,

    crc_32: u32,
}

impl ProgramAssociationSection {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_MPEG_2);

        let table_id = raw[0];
        let section_syntax_indicator = raw[1].bit(0);
        let section_length = u16::from_be_bytes(raw[1..3].try_into()?) & !(0b11_11_11 << 10);
        let transport_stream_id = u16::from_be_bytes(raw[3..5].try_into()?);
        let version_number = raw[5..].bits::<u8>(2, 5);
        let current_next_indicator = raw[5..].bit(7);
        let section_number = raw[6];
        let last_section_number = raw[7];

        let programs_count = (section_length as usize - 9) / 4;

        let mut programs = Vec::new();
        for i in 0..programs_count {
            let offset = 8 + (i * 4);
            let program = ProgramAssociation {
                program_number: u16::from_be_bytes(raw[offset..(offset + 2)].try_into()?),
                program_id: u16::from_be_bytes(raw[(offset + 2)..(offset + 4)].try_into()?)
                    & !(0b111 << 13),
            };
            programs.push(program);
        }

        // Check CRC
        let chksum_provided = raw[(section_length as usize - 1)..].bits::<u32>(0, 32);
        let chksum_calculated = CRC.checksum(&raw[0..(section_length as usize - 1)]);
        if chksum_calculated != chksum_provided {
            return Err(anyhow!(
                "Checksum does not match: {chksum_calculated} != {chksum_provided}"
            ));
        }

        Ok(Self {
            table_id,
            section_syntax_indicator,
            section_length,
            transport_stream_id,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
            programs,
            crc_32: chksum_provided,
        })
    }
}
