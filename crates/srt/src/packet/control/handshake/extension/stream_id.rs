#[derive(Clone, Debug)]
pub struct StreamIdExtension {
    pub r#type: u16,
    pub length: u16,
    pub stream_id: String,
}

impl StreamIdExtension {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let r#type = u16::from_be_bytes(raw[0..2].try_into()?);
        let length = u16::from_be_bytes(raw[2..4].try_into()?);

        let mut stream_id = String::new();

        for i in 0..length {
            let pos = 4 + i as usize * 4;
            let mut bytes = Vec::from(&raw[pos..(pos + 4)]);
            bytes.reverse();
            stream_id += String::from_utf8_lossy(&bytes).trim_matches(char::from(0));
        }

        Ok(Self {
            r#type,
            length,
            stream_id,
        })
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res
    }
}
