use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn parse_be<T>(raw: &[u8]) -> T
where
    T: From<u8> + std::ops::Shl<usize> + std::iter::Sum<<T as std::ops::Shl<usize>>::Output>,
{
    raw.iter()
        .rev()
        .enumerate()
        .map(|(i, n)| T::from(*n) << (i * 8))
        .sum()
}

// pub fn raw_u32_be(raw: &[u8]) -> u32 {
//     raw.iter()
//         .rev()
//         .enumerate()
//         .map(|(i, n)| u32::from(*n) << (i * 8))
//         .sum()
// }

// pub fn raw_u32_le(raw: &[u8]) -> u32 {
//     raw.iter()
//         .enumerate()
//         .map(|(i, n)| u32::from(*n) << (i * 8))
//         .sum()
// }

fn u32_to_system_time(timestamp: u32) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(u64::from(timestamp))
}
