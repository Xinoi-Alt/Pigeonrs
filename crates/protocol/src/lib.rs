mod error;
pub use error::{ProtocolError, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Hello { version: u8 },
    Offer { file_name: String, file_size: u64 },
}
