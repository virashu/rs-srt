/// (bytes)
pub const MAX_PACKET_SIZE: usize = 1500;

pub const HANDSHAKE_MAGIC_CODE: u16 = 0x4A17;

/// (micros)
pub const RTT_INIT: u32 = 100_000;
/// (micros)
pub const RTT_VAR_INIT: u32 = 50_000;

/// (micros)
pub const FULL_ACK_INTERVAL: u32 = 10_000;
