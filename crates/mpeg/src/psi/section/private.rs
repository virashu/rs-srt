pub struct PrivateSection {
    pub table_id: u8,
    pub table_syntax_indicator: bool,
    pub private_indicator: bool,
    private_section_length: u16,
    pub transport_stream_id: u16,
    pub verion_number: u8,
    pub current_next_indicator: bool,
    pub section_number: u8,
    pub last_section_number: u8,

    crc_32: u32,
}
