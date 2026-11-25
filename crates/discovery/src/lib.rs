//! Prototyp für die Discovery per UDP Broadcast (S1-05)
//! Format "PGRS" | tcp_port (u16 BE) | device_name (UTF-8)
use std::io;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

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
pub fn encode(announce: &Announce) -> Option<Vec<u8>> {
    let name_len = announce.device_name.len();
    if name_len == 0 || name_len > MAX_NAME_LEN {
        return None;
    }

    let mut packet = Vec::with_capacity(MAGIC.len() + 2 + announce.device_name.len());
    packet.extend_from_slice(&MAGIC);
    packet.extend_from_slice(&announce.tcp_port.to_be_bytes());
    packet.extend_from_slice(announce.device_name.as_bytes());
    Some(packet)
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

//Schickt ein einzelnes Discovery-Paket an die Adresse 'target'
pub fn send_announce(socket: &UdpSocket, announce: &Announce, target: &str) -> io::Result<()> {
    let packet = match encode(announce) {
        Some(p) => p,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "device name must be 1 to 64 bytes",
            ));
        }
    };
    socket.send_to(&packet, target)?;
    Ok(())
}

//Schickt alle 2 Sekunden ein Discovery-Paket per Broadcast (läuft endlos)
pub fn announce_loop(announce: &Announce, port: u16) -> io::Result<()> {
    if encode(announce).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "device name must be 1 to 64 bytes",
        ));
    }

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let target = format!("255.255.255.255:{port}");
    loop {
        if let Err(e) = send_announce(&socket, announce, &target) {
            eprintln!("Senden fehlgeschlagen: {e}");
        }
        thread::sleep(Duration::from_secs(2));
    }
}

//Wartet auf ein gültiges Discovery Paket. Ungültige Pakete werden übersprungen.
//Gibt das Announce und die Adresse des Absenders zurück.
pub fn receive_announce(socket: &UdpSocket) -> io::Result<(Announce, SocketAddr)> {
    let mut buffer = [0u8; 65536];
    loop {
        let (length, sender) = socket.recv_from(&mut buffer)?;
        let received = &buffer[0..length];
        if let Some(announce) = decode(received) {
            return Ok((announce, sender));
        }
    }
}

pub fn listen_loop(port: u16) -> io::Result<()> {
    let address = format!("0.0.0.0:{port}");
    let socket = UdpSocket::bind(&address)?;
    println!("Lausche auf {address} ...");

    loop {
        let (announce, sender) = receive_announce(&socket)?;
        println!(
            "Gefunden: {} auf {}:{}",
            announce.device_name,
            sender.ip(),
            announce.tcp_port
        );
    }
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
        let packet = encode(&sample()).unwrap();
        assert_eq!(decode(&packet), Some(sample()));
    }
    #[test]
    fn rejects_wrong_magic() {
        let mut packet = encode(&sample()).unwrap();
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
        assert_eq!(encode(&announce), None);
    }
    #[test]
    fn rejects_invalid_text() {
        let mut packet = encode(&sample()).unwrap();
        packet[6] = 255;
        assert_eq!(decode(&packet), None);
    }
    #[test]
    fn send_announce_arrives() {
        let receiver = UdpSocket::bind("127.0.0.1:0").unwrap();
        receiver
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let target = receiver.local_addr().unwrap().to_string();

        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        send_announce(&sender, &sample(), &target).unwrap();

        let mut buffer = [0; 1024];
        let (length, _) = receiver.recv_from(&mut buffer).unwrap();
        let received = &buffer[0..length];
        assert_eq!(decode(received), Some(sample()));
    }

    #[test]
    fn receive_announce_skips_invalid_packets() {
        let receiver = UdpSocket::bind("127.0.0.1:0").unwrap();
        receiver
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let target = receiver.local_addr().unwrap().to_string();

        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        sender.send_to(&[1, 2, 3], &target).unwrap();
        send_announce(&sender, &sample(), &target).unwrap();

        let (announce, _) = receive_announce(&receiver).unwrap();
        assert_eq!(announce, sample());
    }
}
