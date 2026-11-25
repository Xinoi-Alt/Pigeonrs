use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("failed to bind to {address}")]
    Bind {
        address: String,
        source: std::io::Error,
    },

    #[error("connection failed")]
    Accept { source: std::io::Error },

    #[error("peer {address} unreachable")]
    Connect {
        address: String,
        source: std::io::Error,
    },

    #[error("reading or writing error")]
    Io {
        #[from]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, TransportError>;
