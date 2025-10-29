pub trait Serial {
    fn from_raw(raw: &[u8]) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized;

    fn to_raw(&self) -> Vec<u8>;
}
