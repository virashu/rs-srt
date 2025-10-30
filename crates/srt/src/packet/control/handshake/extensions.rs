pub mod handshake_extension_flags {
    pub const HSREQ: u16 = 0x00_01;
    pub const KMREQ: u16 = 0x00_02;
    pub const CONFIG: u16 = 0x00_04;
}

pub mod handshake_extension_message_flags {
    pub const TSBPDSND: u32 = 0x00_00_00_01;
    pub const TSBPDRCV: u32 = 0x00_00_00_02;
    pub const CRYPT: u32 = 0x00_00_00_04;
    pub const TLPKTDROP: u32 = 0x00_00_00_08;
    pub const PERIODICNAK: u32 = 0x00_00_00_10;
    pub const REXMITFLG: u32 = 0x00_00_00_20;
    pub const STREAM: u32 = 0x00_00_00_40;
    pub const PACKET_FILTER: u32 = 0x00_00_00_80;
}

#[derive(Clone, Debug)]
pub struct HandshakeExtension {
    pub r#type: u16,
    pub length: u16,
    pub srt_version: u32,
    pub srt_flags: u32,
    pub receiver_delay: u16,
    pub sender_delay: u16,
}

impl HandshakeExtension {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let r#type = u16::from_be_bytes(raw[0..2].try_into()?);
        let length = u16::from_be_bytes(raw[2..4].try_into()?);

        let srt_version = u32::from_be_bytes(raw[4..8].try_into()?);
        let srt_flags = u32::from_be_bytes(raw[8..12].try_into()?);
        let receiver_delay = u16::from_be_bytes(raw[12..14].try_into()?);
        let sender_delay = u16::from_be_bytes(raw[14..16].try_into()?);

        Ok(Self {
            r#type,
            length,
            srt_version,
            srt_flags,
            receiver_delay,
            sender_delay,
        })
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend(self.r#type.to_be_bytes());
        res.extend(self.length.to_be_bytes());
        res.extend(self.srt_version.to_be_bytes());
        res.extend(self.srt_flags.to_be_bytes());
        res.extend(self.receiver_delay.to_be_bytes());
        res.extend(self.sender_delay.to_be_bytes());

        res
    }
}
