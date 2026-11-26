mod error;
pub use error::{ProtocolError, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Hello {
        version: u8,
    },
    Offer {
        file_name: String,
        file_size: u64,
    },
    Accept,
    Reject {
        reason: String,
    },
    Chunk {
        chunk_number: u32,
        chunk_data: Vec<u8>,
    },
}
