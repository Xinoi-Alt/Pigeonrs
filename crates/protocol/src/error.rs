use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("unsupported protocol version: expected {expected}, found {found}" )]
        UnsupportedVersion { expected:u8, found:u8 },

    #[error("invalid message: {0}")]
    InvalidMessage(String), 
}

pub type Result<T> = std::result::Result<T, ProtocolError>;

