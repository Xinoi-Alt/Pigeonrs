mod error;
pub use error::{ProtocolError, Result};

use serde::{Deserialize, Serialize};
///Aktuelle Protokollversion, wird mit Hello mitgeschickt
pub const PROTOCOL_VERSION: u8 = 1;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    ///Erste Nachricht nach dem Verbindungsaufbau, beide Seiten senden sie
    Hello { version: u8 },
    ///Sender bietet Datei an
    Offer { file_name: String, file_size: u64 },
    ///Empfänger nimmt Angebot an
    Accept,
    ///Empfänger lehnt bewusst ab
    Reject { reason: String },
    ///Ein Stück der Datei mit seiner Position
    Chunk {
        chunk_number: u32,
        chunk_data: Vec<u8>,
    },
    ///Chunk kam fehlerhaft an und soll neu gesendet werden
    Nack { chunk_number: u32 },
    ///Übertragung ist abgeschlossen
    Done,
    ///Lebenszeichen-Anfrage
    Ping,
    ///Antwort auf Ping
    Pong,
}

impl Message {
    ///Wandelt die Nachricht mit postcard in Bytes um
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        postcard::to_allocvec(self).map_err(|e| ProtocolError::InvalidMessage(e.to_string()))
    }
    ///Liest eine Nachricht aus Bytes ; Kaputte Byzes ergeben Fehler statt einer panic
    pub fn from_bytes(bytes: &[u8]) -> Result<Message> {
        postcard::from_bytes(bytes).map_err(|e| ProtocolError::InvalidMessage(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(msg: Message) {
        let bytes = msg.to_bytes().unwrap();
        let back = Message::from_bytes(&bytes).unwrap();
        assert_eq!(msg, back);
    }

    #[test]
    fn roundtrip_all_messages() {
        roundtrip(Message::Hello {
            version: PROTOCOL_VERSION,
        });
        roundtrip(Message::Offer {
            file_name: "urlaub.jpg".to_string(),
            file_size: 5_000_000_000,
        });
        roundtrip(Message::Accept);
        roundtrip(Message::Reject {
            reason: "Nicht genug Speicherplatz".to_string(),
        });
        roundtrip(Message::Chunk {
            chunk_number: 2,
            chunk_data: vec![0, 1, 2, 255],
        });
        roundtrip(Message::Nack { chunk_number: 2 });
        roundtrip(Message::Done);
        roundtrip(Message::Ping);
        roundtrip(Message::Pong);
    }
    #[test]
    fn invalid_bytes_give_error() {
        let result = Message::from_bytes(&[255, 255, 255]);
        assert!(result.is_err());
    }
}
