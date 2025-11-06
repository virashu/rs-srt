use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Nak {
    pub lost_packet: u32,
}

impl Nak {
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        todo!()
    }

    pub fn raw_content(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend(self.lost_packet.to_be_bytes());

        res
    }
}
