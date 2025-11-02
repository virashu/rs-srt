use crate::pes_header::PesHeader;

pub mod stream_ids {
    pub const PROGRAM_STREAM_MAP: u8 = 0b1011_1100;
    pub const PRIVATE_STREAM_1: u8 = 0b1011_1101;
    pub const PADDING_STREAM: u8 = 0b1011_1110;
    pub const PRIVATE_STREAM_2: u8 = 0b1011_1111;

    pub const ECM_STREAM: u8 = 0b1111_0000;
    pub const EMM_STREAM: u8 = 0b1111_0001;

    pub const PROGRAM_STREAM_DIRECTORY: u8 = 0b1111_1111;
    pub const DSMCC_STREAM: u8 = 0b1111_0010;

    pub const ITU_T_REC_H2221_TYPE_E_STREAM: u8 = 0b1111_1000;

    pub const GROUP_NO_HEADER: &[u8] = &[
        PROGRAM_STREAM_MAP,
        PRIVATE_STREAM_1,
        PADDING_STREAM,
        PRIVATE_STREAM_2,
        ECM_STREAM,
        EMM_STREAM,
        PROGRAM_STREAM_DIRECTORY,
        DSMCC_STREAM,
        ITU_T_REC_H2221_TYPE_E_STREAM,
    ];
}

/// 24b+
#[derive(Debug)]
pub struct PesPacket {
    // 24b prefix
    /// 8b
    pub stream_id: u8,
    /// 16b
    pub pes_packet_length: u16,
    /// n?
    pub pes_header: Option<PesHeader>,
    /// n
    pub pes_data: Vec<u8>,
}

impl PesPacket {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let stream_id = raw[3];
        let pes_packet_length = u16::from_be_bytes(raw[4..6].try_into()?);

        let pes_header = (!stream_ids::GROUP_NO_HEADER.contains(&stream_id))
            .then(|| PesHeader::from_raw(&raw[6..]))
            .transpose()?;

        let data_start = 6 + pes_header.as_ref().map_or(0, |h| h.size().div_ceil(8));
        let pes_data = Vec::from(&raw[data_start..]);

        Ok(Self {
            stream_id,
            pes_packet_length,
            pes_header,
            pes_data,
        })
    }
}
