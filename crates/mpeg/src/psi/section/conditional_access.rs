pub struct ConditionalAccessSection {
    pub table_id: u8,
    pub section_syntax_indicator: bool,
    section_length: u16,
    pub version_number: u8,
    pub current_next_indicator: bool,
    pub section_number: u8,
    pub last_section_number: u8,

    crc_32: u32,
}
