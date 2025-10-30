#[derive(Debug)]
pub enum PacketPosition {
    Middle,
    First,
    Last,
    Single,
}

#[derive(Debug)]
pub enum EncryptionFlag {
    NoEncryption,
    EvenKey,
    OddKey,
}

#[derive(Debug)]
pub struct DataPacketInfo {
    pub packet_sequence_number: u32,
    pub position: PacketPosition,
    pub order: bool,
    pub encryption: EncryptionFlag,
    pub retransmitted: bool,
    pub message_number: u32,
    pub content: Vec<u8>,
}

impl DataPacketInfo {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let packet_sequence_number = u32::from_be_bytes(raw[0..4].try_into()?) & !(1 << 31);

        let fb = raw[4];
        let position = match (fb & 0b1100_0000) >> 6 {
            0b00 => PacketPosition::Middle,
            0b01 => PacketPosition::Last,
            0b10 => PacketPosition::First,
            0b11 => PacketPosition::Single,
            _ => unreachable!(),
        };
        let order = match (fb & 0b0010_0000) >> 5 {
            0 => false,
            1 => true,
            _ => unreachable!(),
        };
        let encryption = match (fb & 0b0001_1000) >> 3 {
            0b00 => EncryptionFlag::NoEncryption,
            0b01 => EncryptionFlag::EvenKey,
            0b10 => EncryptionFlag::OddKey,
            _ => unreachable!(),
        };
        let retransmitted = match (fb & 0b0000_0100) >> 2 {
            0 => false,
            1 => true,
            _ => unreachable!(),
        };

        let message_number = u32::from_be_bytes(raw[4..8].try_into()?) & !(0b11_11_11 << 26);
        let content = Vec::from(&raw[8..]);

        Ok(Self {
            packet_sequence_number,
            position,
            order,
            encryption,
            retransmitted,
            message_number,
            content,
        })
    }

    pub fn raw_header(&self) -> Vec<u8> {
        todo!()
    }

    pub fn raw_content(&self) -> Vec<u8> {
        todo!()
    }
}
