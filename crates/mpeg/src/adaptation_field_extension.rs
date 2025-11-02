/// 0-77b
#[derive(Debug)]
pub struct AdaptationFieldExtension {
    // Optional fields
    /// 1b?
    pub ltw_valid_flag: bool,
    /// 15b?
    pub ltw_offset: u16,
    // 2b empty
    /// 22b?
    pub piecewise_rate: u32,
    /// 4b?
    pub splice_type: u8,
    /// 33b?
    pub dts_next_au: u64,
}
