//! Error types for AI-powered features

use thiserror::Error;

/// Error types for AI operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum AiError {
    #[error("Engine not initialized")]
    EngineNotInitialized,

    #[error("Data corruption detected: {details}")]
    DataCorruption { details: String },

    #[error("Memory exhausted: {limit_mb}MB limit exceeded")]
    MemoryExhausted { limit_mb: u32 },

    #[error("Performance degradation: {operation} took {ms}ms (limit: {limit_ms}ms)")]
    PerformanceDegradation {
        operation: String,
        ms: u64,
        limit_ms: u64,
    },

    #[error("Invalid pattern: {reason}")]
    InvalidPattern { reason: String },

    #[error("Suggestion generation failed: {reason}")]
    SuggestionFailed { reason: String },

    #[error("Model not found: {model_name}")]
    ModelNotFound { model_name: String },

    #[error("Invalid model version: {version}")]
    InvalidModelVersion { version: String },

    #[error("Cache full: {cache_name} at capacity")]
    CacheFull { cache_name: String },

    #[error("Analysis failed: {reason}")]
    AnalysisFailed { reason: String },
}

/// Result type for AI operations
pub type AiResult<T> = Result<T, AiError>;

impl AiError {
    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            AiError::EngineNotInitialized
            | AiError::InvalidPattern { .. }
            | AiError::ModelNotFound { .. }
            | AiError::InvalidModelVersion { .. } => false,

            AiError::DataCorruption { .. }
            | AiError::MemoryExhausted { .. }
            | AiError::PerformanceDegradation { .. }
            | AiError::SuggestionFailed { .. }
            | AiError::CacheFull { .. }
            | AiError::AnalysisFailed { .. } => true,
        }
    }

    /// Get severity level
    pub fn severity(&self) -> Severity {
        match self {
            AiError::EngineNotInitialized => Severity::Critical,
            AiError::DataCorruption { .. } => Severity::Critical,
            AiError::MemoryExhausted { .. } => Severity::High,
            AiError::PerformanceDegradation { .. } => Severity::Medium,
            AiError::InvalidPattern { .. } => Severity::Low,
            AiError::SuggestionFailed { .. } => Severity::Medium,
            AiError::ModelNotFound { .. } => Severity::High,
            AiError::InvalidModelVersion { .. } => Severity::High,
            AiError::CacheFull { .. } => Severity::Low,
            AiError::AnalysisFailed { .. } => Severity::Medium,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}
