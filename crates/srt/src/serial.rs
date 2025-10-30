pub trait Serial: Sized {
    fn from_raw(raw: &[u8]) -> anyhow::Result<Self>;
    fn to_raw(&self) -> Vec<u8>;
}
