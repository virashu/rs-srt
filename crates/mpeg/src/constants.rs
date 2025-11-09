pub const PACKET_SIZE: usize = 188;

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

pub mod steam_types {
    pub const ISO_13818_7_AUDIO: u8 = 0x0F;
    pub const ISO_14496_10_VIDEO: u8 = 0x1B;
}

pub mod descriptor_tags {
    pub const MPEG4_VIDEO_DESCRIPTOR: u8 = 27;
}

pub mod packet_ids {
    pub const PROGRAM_ASSOCIATION_TABLE: u16 = 0x0000;
    pub const CONDITIONAL_ACCESS_TABLE: u16 = 0x0001;
    pub const TRANSPORT_STREAM_DESCRIPTION_TABLE: u16 = 0x0002;
    pub const IPMP_CONTROL_INFORMATION_TABLE: u16 = 0x0003;

    pub const GROUP_CONTROL: &[u16] = &[
        PROGRAM_ASSOCIATION_TABLE,
        CONDITIONAL_ACCESS_TABLE,
        TRANSPORT_STREAM_DESCRIPTION_TABLE,
        IPMP_CONTROL_INFORMATION_TABLE,
    ];
}

pub mod table_ids {
    pub const PROGRAM_ASSOCIATION_SECTION: u8 = 0x00;
    pub const CONDITIONAL_ACCESS_SECTION: u8 = 0x01;
    pub const TS_PROGRAM_MAP_SECTION: u8 = 0x02;
    pub const TS_DESCRIPTION_SECTION: u8 = 0x03;
    pub const ISO_14496_SCENE_DESCRIPTION_SECTION: u8 = 0x04;
    pub const ISO_14496_OBJECT_DESCRIPTION_SECTION: u8 = 0x05;
    pub const METADATA_SECTION: u8 = 0x06;
    pub const IPMP_CONTROL_INFORMATION_SECTION: u8 = 0x07;
}
