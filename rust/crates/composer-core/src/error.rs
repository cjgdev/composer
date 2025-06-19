//! Error types for chord theory operations

use thiserror::Error;

/// Error types for chord theory operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ChordTheoryError {
    #[error("Invalid chord root degree: {root}. Must be 0-7")]
    InvalidChordRoot { root: u8 },

    #[error("Invalid chord type: {chord_type}. Must be one of [5, 7, 9, 11, 13]")]
    InvalidChordType { chord_type: u8 },

    #[error("Invalid inversion level: {inversion}. Must be 0-3")]
    InvalidInversion { inversion: u8 },

    #[error("Invalid applied degree: {applied}. Must be 0-7")]
    InvalidApplied { applied: u8 },

    #[error("Invalid scale fingerprint: {reason}")]
    InvalidScaleFingerprint { reason: String },

    #[error("Incompatible chord alterations: {alterations:?}")]
    IncompatibleAlterations { alterations: Vec<String> },

    #[error("Missing required chord property: {property}")]
    MissingRequiredProperty { property: String },

    #[error("Scale degree out of range: {degree}. Must be 1-7")]
    ScaleDegreeOutOfRange { degree: u8 },

    #[error("Invalid alteration: {alteration}. Must be one of [b5, #5, b9, #9, #11, b13]")]
    InvalidAlteration { alteration: String },

    #[error("Invalid suspension: {suspension}. Must be 2 or 4")]
    InvalidSuspension { suspension: u8 },

    #[error("Calculation overflow in {operation}")]
    CalculationOverflow { operation: String },
}

/// Result type for chord theory operations
pub type ChordTheoryResult<T> = Result<T, ChordTheoryError>;

impl ChordTheoryError {
    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            ChordTheoryError::InvalidChordRoot { .. }
            | ChordTheoryError::InvalidChordType { .. }
            | ChordTheoryError::InvalidInversion { .. }
            | ChordTheoryError::InvalidApplied { .. }
            | ChordTheoryError::ScaleDegreeOutOfRange { .. }
            | ChordTheoryError::InvalidAlteration { .. }
            | ChordTheoryError::InvalidSuspension { .. } => false,

            ChordTheoryError::InvalidScaleFingerprint { .. }
            | ChordTheoryError::IncompatibleAlterations { .. }
            | ChordTheoryError::MissingRequiredProperty { .. }
            | ChordTheoryError::CalculationOverflow { .. } => true,
        }
    }
}
