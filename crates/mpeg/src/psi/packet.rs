use anyhow::Result;

use crate::{
    constants::table_ids,
    psi::section::{
        program_association::ProgramAssociationSection, ts_program_map::TsProgramMapSection,
    },
};

#[derive(Debug)]
pub enum Section {
    PAS(ProgramAssociationSection),
    PMS(TsProgramMapSection),
}

#[derive(Debug)]
pub struct ProgramSpecificInformation {
    pointer: u8,

    pub section: Section,
}

impl ProgramSpecificInformation {
    /// # Errors
    /// Error while parsing raw bytes
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let pointer = raw[0] + 1;
        let table_id = raw[pointer as usize];

        let section = match table_id {
            table_ids::PROGRAM_ASSOCIATION_SECTION => Section::PAS(
                ProgramAssociationSection::from_raw(&raw[pointer as usize..])?,
            ),
            table_ids::TS_PROGRAM_MAP_SECTION => {
                Section::PMS(TsProgramMapSection::from_raw(&raw[pointer as usize..])?)
            }
            _ => todo!("{table_id}"),
        };

        Ok(Self { pointer, section })
    }
}
