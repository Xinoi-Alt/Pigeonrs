//! Prototyp für die Discovery per UDP Broadcast (S1-05)
//! Format "PGRS" | tcp_port (u16 BE) | device_name (UTF-8)

pub const DISCOVERY_PORT: u16 = 52531;
pub const MAGIC: [u8; 4] = *b"PGRS";
pub const MAX_PACKET_LEN: usize = 516;
pub const MAX_NAME_LEN: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Announce {
    pub device_name: String,
    pub tcp_port: u16,
}

//Baut die Bytes für ein Discovery Paket
pub fn encode(announce: &Announce) -> Vec<u8> {
    let mut packet = Vec::with_capacity(MAGIC.len() + 2 + announce.device_name.len());
    packet.extend_from_slice(&MAGIC);
    packet.extend_from_slice(&announce.tcp_port.to_be_bytes());
    packet.extend_from_slice(announce.device_name.as_bytes());
    packet
}

//Prüft ein empfangenes Paket
pub fn decode(packet: &[u8]) -> Option<Announce> {
    if packet.len() < 7 || packet.len() > MAX_PACKET_LEN {
        return None;
    }
    if packet[0..4] != MAGIC {
        return None;
    }
    let tcp_port = u16::from_be_bytes([packet[4], packet[5]]);
    let name_bytes = &packet[6..];
    if name_bytes.len() > MAX_NAME_LEN {
        return None;
    }
    let device_name = match String::from_utf8(name_bytes.to_vec()) {
        Ok(name) => name,
        Err(_) => return None,
    };
    Some(Announce {
        device_name,
        tcp_port,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Announce {
        Announce {
            device_name: "Laptop".to_string(),
            tcp_port: 52530,
        }
    }
    #[test]
    fn roundtrip() {
        let packet = encode(&sample());
        assert_eq!(decode(&packet), Some(sample()));
    }
    #[test]
    fn rejects_wrong_magic() {
        let mut packet = encode(&sample());
        packet[0] = b'X';
        assert_eq!(decode(&packet), None);
    }
    #[test]
    fn rejects_to_short() {
        let packet = vec![1, 2, 3];
        assert_eq!(decode(&packet), None);
    }
    #[test]
    fn rejects_to_long_name() {
        let announce = Announce {
            device_name: "a".repeat(65),
            tcp_port: 52530,
        };
        let packet = encode(&announce);
        assert_eq!(decode(&packet), None);
    }
    #[test]
    fn rejects_invalid_text() {
        let mut packet = encode(&sample());
        packet[6] = 255;
        assert_eq!(decode(&packet), None);
    }
}
