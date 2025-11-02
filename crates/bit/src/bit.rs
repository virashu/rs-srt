pub trait Bit {
    fn bit(&self, index: usize) -> bool;
}

impl Bit for &[u8] {
    fn bit(&self, index: usize) -> bool {
        let byte_offset = index / 8;
        let bit_offset = index % 8;
        (self[byte_offset] >> (7 - bit_offset)) & 1 != 0
    }
}

impl Bit for u8 {
    fn bit(&self, index: usize) -> bool {
        (self >> (7 - index)) & 1 != 0
    }
}
