//! Error types for serialization operations

use thiserror::Error;

/// Error types for serialization operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum SerializationError {
    #[error("Invalid binary format: {reason}")]
    InvalidBinaryFormat { reason: String },

    #[error("Unsupported serialization version: {version}")]
    UnsupportedVersion { version: String },

    #[error("Corrupted binary data: {details}")]
    CorruptedBinary { details: String },

    #[error("Token library not initialized")]
    TokenLibraryMissing,

    #[error("Invalid token format: {token}")]
    InvalidTokenFormat { token: String },

    #[error("Serialization buffer overflow")]
    BufferOverflow,

    #[error("Deserialization failed: {reason}")]
    DeserializationFailed { reason: String },

    #[error("Invalid chord data structure: {field}")]
    InvalidChordData { field: String },

    #[error("Trie structure validation failed: {reason}")]
    InvalidTrieStructure { reason: String },

    #[error("IO error during serialization: {details}")]
    IoError { details: String },

    #[error("Invalid format: {message}")]
    InvalidFormat { message: String },

    #[error("Unexpected end of file")]
    UnexpectedEof,
}

/// Result type for serialization operations
pub type SerializationResult<T> = Result<T, SerializationError>;

impl SerializationError {
    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            SerializationError::InvalidBinaryFormat { .. }
            | SerializationError::UnsupportedVersion { .. }
            | SerializationError::CorruptedBinary { .. }
            | SerializationError::BufferOverflow => false,

            SerializationError::TokenLibraryMissing
            | SerializationError::InvalidTokenFormat { .. }
            | SerializationError::DeserializationFailed { .. }
            | SerializationError::InvalidChordData { .. }
            | SerializationError::InvalidTrieStructure { .. }
            | SerializationError::IoError { .. }
            | SerializationError::InvalidFormat { .. }
            | SerializationError::UnexpectedEof => true,
        }
    }
}
