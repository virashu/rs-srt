//! <https://datatracker.ietf.org/doc/html/draft-sharabayko-srt#section-3.2.1>

use crate::macros::auto_try_from;

auto_try_from! {
    #[repr(u16)]
    #[derive(Clone, Copy, Debug)]
    pub enum HandshakeEncryption {
        NoEncryption = 0,
        AES128 = 2,
        AES192 = 3,
        AES256 = 4,
    }
}

auto_try_from! {
    #[repr(u32)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum HandshakeType {
        Done = 0xFF_FF_FF_FD,
        Agreement = 0xFF_FF_FF_FE,
        Conclusion = 0xFF_FF_FF_FF,
        WaveHand = 0x00_00_00_00,
        Induction = 0x00_00_00_01,
    }
}

#[derive(Clone, Debug)]
pub struct HandshakeExtension {
    pub r#type: u16,
    pub length: u16,
    pub content: Vec<u8>,
}

/// [ 48 BYTES (+ Extension) ]
#[derive(Clone, Debug)]
pub struct Handshake {
    pub version: u32,
    pub encryption: HandshakeEncryption,
    pub extension_field: u16,
    pub initial_packet_sequence_number: u32,
    pub maximum_transmission_unit_size: u32,
    pub maximum_flow_window_size: u32,
    pub handshake_type: HandshakeType,
    pub srt_socket_id: u32,
    pub syn_cookie: u32,
    pub peer_ip_address: (u32, u32, u32, u32),
    pub extension: Option<HandshakeExtension>,
}

impl Handshake {
    pub fn from_raw(raw: &[u8]) -> anyhow::Result<Self> {
        let version = u32::from_be_bytes(raw[0..4].try_into()?);

        let encryption = u16::from_be_bytes(raw[4..6].try_into()?).try_into()?;
        let extension_field = u16::from_be_bytes(raw[6..8].try_into()?);

        let initial_packet_sequence_number = u32::from_be_bytes(raw[8..12].try_into()?);
        let maximum_transmission_unit_size = u32::from_be_bytes(raw[12..16].try_into()?);
        let maximum_flow_window_size = u32::from_be_bytes(raw[16..20].try_into()?);
        let handshake_type = u32::from_be_bytes(raw[20..24].try_into()?).try_into()?;
        let srt_socket_id = u32::from_be_bytes(raw[24..28].try_into()?);
        let syn_cookie = u32::from_be_bytes(raw[28..32].try_into()?);

        let peer_ip_address = (
            u32::from_be_bytes(raw[32..36].try_into()?),
            u32::from_be_bytes(raw[36..40].try_into()?),
            u32::from_be_bytes(raw[40..44].try_into()?),
            u32::from_be_bytes(raw[44..48].try_into()?),
        );

        let extension = if raw.len() > 48 {
            let r#type = u16::from_be_bytes(raw[48..50].try_into()?);
            let length = u16::from_be_bytes(raw[50..52].try_into()?);
            let content = Vec::from(&raw[52..(52 + length as usize)]);

            Some(HandshakeExtension {
                r#type,
                length,
                content,
            })
        } else {
            None
        };

        Ok(Self {
            version,
            encryption,
            extension_field,
            initial_packet_sequence_number,
            maximum_transmission_unit_size,
            maximum_flow_window_size,
            handshake_type,
            srt_socket_id,
            syn_cookie,
            peer_ip_address,
            extension,
        })
    }

    pub fn to_raw(&self) -> Vec<u8> {
        let mut res = Vec::new();

        res.extend(self.version.to_be_bytes());
        res.extend((self.encryption as u16).to_be_bytes());
        res.extend(self.extension_field.to_be_bytes());
        res.extend(self.initial_packet_sequence_number.to_be_bytes());
        res.extend(self.maximum_transmission_unit_size.to_be_bytes());
        res.extend(self.maximum_flow_window_size.to_be_bytes());
        res.extend((self.handshake_type as u32).to_be_bytes());
        res.extend(self.srt_socket_id.to_be_bytes());
        res.extend(self.syn_cookie.to_be_bytes());

        res.extend(self.peer_ip_address.0.to_be_bytes());
        res.extend(self.peer_ip_address.1.to_be_bytes());
        res.extend(self.peer_ip_address.2.to_be_bytes());
        res.extend(self.peer_ip_address.3.to_be_bytes());

        if let Some(ext) = &self.extension {
            res.extend(ext.r#type.to_be_bytes());
            res.extend(ext.length.to_be_bytes());
            res.extend(ext.content.clone());
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_raw() {
        let raw: [u8; _] = [
            0, 0, 0, 4, 0, 0, 0, 2, 82, 85, 247, 107, 0, 0, 5, 220, 0, 0, 32, 0, 0, 0, 0, 1, 16,
            10, 162, 106, 0, 0, 0, 0, 1, 0, 0, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let h = Handshake::from_raw(&raw).unwrap();

        assert_eq!(raw, h.to_raw().as_slice());
    }
}
