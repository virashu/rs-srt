pub fn get_bit(buf: &[u8], index: usize) -> u8 {
    let byte_offset = index / 8;
    let bit_offset = index % 8;

    (buf[byte_offset] >> (7 - bit_offset)) & 1
}

pub fn from_bits<T>(buf: &[u8], offset: usize, size: usize) -> T
where
    T: From<u8> + std::ops::Shl<usize> + std::ops::BitOrAssign<<T as std::ops::Shl<usize>>::Output>,
{
    let type_size = size_of::<T>() * 8;

    let mut res = T::from(0u8);
    let zeroes = type_size - size;

    for (n, pos) in (offset..(offset + size)).enumerate() {
        let abs_n = n + zeroes;
        let bit = get_bit(buf, pos);
        let mask = T::from(bit) << (type_size - 1 - abs_n);
        res |= mask;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        let buf: [u8; _] = [0b0000_0001, 0b1000_0000];

        assert_eq!(get_bit(&buf, 6), 0);
        assert_eq!(get_bit(&buf, 7), 1);
        assert_eq!(get_bit(&buf, 8), 1);
        assert_eq!(get_bit(&buf, 9), 0);
    }

    #[test]
    fn test_slice_full() {
        let buf: [u8; _] = [
            0b0000_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_0000,
        ];

        let x = from_bits::<u32>(&buf, 4, 32);
        assert_eq!(x, u32::MAX);
    }

    #[test]
    fn test_slice_part() {
        let buf: [u8; _] = [
            0b0000_0000,
            0b0000_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_0000,
        ];

        let x = from_bits::<u32>(&buf, 12, 24);
        assert_eq!(x, 0x00_FF_FF_FF);
    }
}
