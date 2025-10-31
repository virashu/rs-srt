#[derive(Clone, Debug)]
pub enum KeyBasedEncryption {
    // None,
    EvenKey,
    OddKey,
    Both,
}

#[derive(Clone, Debug)]
pub struct KeyMaterialExtension {
    pub r#type: u16,
    pub length: u16,
    pub packet_type: u8,
    pub key_based_encryption: KeyBasedEncryption,
}

impl KeyMaterialExtension {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let r#type = u16::from_be_bytes(raw[0..2].try_into()?);
        let length = u16::from_be_bytes(raw[2..4].try_into()?);

        let packet_type = raw[4] & 0b0000_1111;
        // let sign = u16::from_be_bytes(raw[5..7].try_into()?); // = 0x2029
        let key_based_encryption = match raw[7] & 0x0000_0011 {
            0b00 => return Err(anyhow::anyhow!("Invalid extension format")),
            0b01 => KeyBasedEncryption::EvenKey,
            0b10 => KeyBasedEncryption::OddKey,
            0b11 => KeyBasedEncryption::Both,
            _ => unreachable!(),
        };
        // let keki = u32::from_be_bytes(raw[8..12].try_into()?); // = 0
        let cipher = u8::from_be_bytes(raw[12..13].try_into()?);
        // let auth = u8::from_be_bytes(raw[13..14].try_into()?); // = 0
        let stream_encapsulation = u8::from_be_bytes(raw[14..15].try_into()?);

        Ok(Self {
            r#type,
            length,
            packet_type,
            key_based_encryption,
        })
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res
    }
}
