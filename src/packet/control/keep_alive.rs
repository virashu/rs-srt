use crate::serial::Serial;

#[derive(Clone, Debug)]
pub struct KeepAlive {}

impl Serial for KeepAlive {
    fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        todo!()
    }

    fn to_raw(&self) -> Vec<u8> {
        todo!()
    }
}
