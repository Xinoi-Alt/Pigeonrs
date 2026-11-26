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
