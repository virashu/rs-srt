use anyhow::Result;

#[derive(Clone, Debug)]
pub enum Nak {
    Single {
        lost_packet: u32,
    },
    Range {
        lost_packets_from: u32,
        lost_packets_to: u32,
    },
}

impl Nak {
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let is_range = raw[16] >> 7 == 1;

        if is_range {
            let lost_packets_from = u32::from_be_bytes(raw[16..20].try_into()?) & !(1 << 31);
            let lost_packets_to = u32::from_be_bytes(raw[20..24].try_into()?);
            Ok(Self::Range {
                lost_packets_from,
                lost_packets_to,
            })
        } else {
            let lost_packet = u32::from_be_bytes(raw[16..20].try_into()?);
            Ok(Self::Single { lost_packet })
        }
    }

    pub fn raw_content(&self) -> Vec<u8> {
        let mut res = Vec::new();

        match self {
            Self::Single { lost_packet } => {
                res.extend(lost_packet.to_be_bytes());
            }
            Self::Range {
                lost_packets_from,
                lost_packets_to,
            } => {
                res.extend((lost_packets_from | (1 << 31)).to_be_bytes());
                res.extend(lost_packets_to.to_be_bytes());
            }
        }

        res
    }
}
