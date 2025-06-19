//! Configuration constants and system parameters for Composer
//!
//! This module provides all configuration constants, system parameters, and operational
//! limits as defined in the Configuration & Constants Specification.

use serde::{Deserialize, Serialize};

/// Application metadata constants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationMetadata {
    pub version: &'static str,
    pub secret_key: &'static str,
    pub encryption_salt: &'static str,
    pub hash_salt: &'static str,
    pub hash_alphabet: &'static str,
    pub hash_min_length: u8,
}

/// Default application metadata
pub const APPLICATION: ApplicationMetadata = ApplicationMetadata {
    version: "2.35.2",
    secret_key: "MANATEE",
    encryption_salt: "M70XExr448",
    hash_salt: "XI0Y4UFrK6EPLnarrI4y",
    hash_alphabet: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_-",
    hash_min_length: 8,
};

/// Musical theory constants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MusicalConstants {
    pub scale_degrees: u8,
    pub chromatic_notes: u8,
    pub octave_range: u8,
    pub middle_c_midi: u8,
    pub default_octave: u8,
    pub chord_types: &'static [u8],
    pub max_inversions: u8,
    pub max_extensions: u8,
    pub ticks_per_beat: u8,
    pub beats_per_measure: u8,
    pub default_tempo: u16,
    pub min_tempo: u16,
    pub max_tempo: u16,
}

/// Default musical constants
pub const MUSICAL: MusicalConstants = MusicalConstants {
    scale_degrees: 7,
    chromatic_notes: 12,
    octave_range: 10,
    middle_c_midi: 60,
    default_octave: 4,
    chord_types: &[5, 7, 9, 11, 13],
    max_inversions: 4,
    max_extensions: 6,
    ticks_per_beat: 24,
    beats_per_measure: 4,
    default_tempo: 120,
    min_tempo: 60,
    max_tempo: 200,
};

/// Analysis parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisParameters {
    pub complexity_scale_max: f64,
    pub novelty_threshold: f64,
    pub tension_scale_max: f64,
    pub difficulty_percentile_max: u8,
}

/// Default analysis parameters
pub const ANALYSIS: AnalysisParameters = AnalysisParameters {
    complexity_scale_max: 10.0,
    novelty_threshold: 0.15,
    tension_scale_max: 100.0,
    difficulty_percentile_max: 99,
};

/// Performance threshold constants (in milliseconds)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub chord_lookup_max_ms: u32,
    pub chord_suggestion_max_ms: u32,
    pub music_analysis_max_ms: u32,
    pub asset_loading_max_ms: u32,
    pub ui_response_max_ms: u32,
}

/// Default performance thresholds
pub const PERFORMANCE: PerformanceThresholds = PerformanceThresholds {
    chord_lookup_max_ms: 1,
    chord_suggestion_max_ms: 50,
    music_analysis_max_ms: 200,
    asset_loading_max_ms: 30000,
    ui_response_max_ms: 16,
};

/// Memory limit constants (in MB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLimits {
    pub memory_usage_max_mb: u32,
    pub cache_size_max_entries: u32,
    pub object_pool_size: u32,
    pub trie_memory_max_mb: u32,
}

/// Default memory limits
pub const MEMORY: MemoryLimits = MemoryLimits {
    memory_usage_max_mb: 150,
    cache_size_max_entries: 10000,
    object_pool_size: 1000,
    trie_memory_max_mb: 100,
};

/// Processing limits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessingLimits {
    pub max_pattern_length: u8,
    pub max_suggestions: u8,
    pub max_batch_size: u8,
    pub max_concurrent_requests: u8,
}

/// Default processing limits
pub const PROCESSING: ProcessingLimits = ProcessingLimits {
    max_pattern_length: 20,
    max_suggestions: 100,
    max_batch_size: 50,
    max_concurrent_requests: 10,
};

/// Algorithm tuning constants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlgorithmConstants {
    pub context_bonus_forward: f64,
    pub statistical_strength_divisor: f64,
    pub complexity_penalty_factor: f64,
    pub max_pattern_length: u8,
    pub min_frequency_threshold: u32,
    pub cache_size_limit: u32,
    pub memory_warning_threshold: u32,
    pub performance_target_ms: u32,
}

/// Default algorithm constants  
pub const ALGORITHM: AlgorithmConstants = AlgorithmConstants {
    context_bonus_forward: 1.7,
    statistical_strength_divisor: 10000.0,
    complexity_penalty_factor: 0.05,
    max_pattern_length: 20,
    min_frequency_threshold: 5,
    cache_size_limit: 10000,
    memory_warning_threshold: 100,
    performance_target_ms: 50,
};

/// Quality thresholds for suggestions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityThresholds {
    pub min_suggestion_weight: f64,
    pub max_suggestions_per_query: u8,
    pub diversity_threshold: f64,
    pub confidence_threshold: f64,
}

/// Default quality thresholds
pub const QUALITY: QualityThresholds = QualityThresholds {
    min_suggestion_weight: 0.01,
    max_suggestions_per_query: 100,
    diversity_threshold: 0.8,
    confidence_threshold: 0.3,
};

/// Error type for configuration validation
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid configuration value: {field} = {value}")]
    InvalidValue { field: String, value: String },

    #[error("Configuration value out of range: {field} must be between {min} and {max}")]
    OutOfRange {
        field: String,
        min: String,
        max: String,
    },

    #[error("Missing required configuration: {field}")]
    MissingRequired { field: String },
}

/// Validation trait for configuration structures
pub trait Validate {
    /// Validate the configuration values
    fn validate(&self) -> Result<(), ConfigError>;
}

impl Validate for MusicalConstants {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.scale_degrees == 0 {
            return Err(ConfigError::InvalidValue {
                field: "scale_degrees".to_string(),
                value: "0".to_string(),
            });
        }

        if self.chromatic_notes != 12 {
            return Err(ConfigError::InvalidValue {
                field: "chromatic_notes".to_string(),
                value: self.chromatic_notes.to_string(),
            });
        }

        if self.min_tempo >= self.max_tempo {
            return Err(ConfigError::OutOfRange {
                field: "tempo_range".to_string(),
                min: self.min_tempo.to_string(),
                max: self.max_tempo.to_string(),
            });
        }

        Ok(())
    }
}

impl Validate for PerformanceThresholds {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.chord_lookup_max_ms == 0 {
            return Err(ConfigError::InvalidValue {
                field: "chord_lookup_max_ms".to_string(),
                value: "0".to_string(),
            });
        }

        if self.ui_response_max_ms > 32 {
            return Err(ConfigError::OutOfRange {
                field: "ui_response_max_ms".to_string(),
                min: "1".to_string(),
                max: "32".to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_metadata() {
        assert_eq!(APPLICATION.version, "2.35.2");
        assert_eq!(APPLICATION.hash_min_length, 8);
        assert!(!APPLICATION.hash_alphabet.is_empty());
    }

    #[test]
    fn test_musical_constants() {
        assert_eq!(MUSICAL.scale_degrees, 7);
        assert_eq!(MUSICAL.chromatic_notes, 12);
        assert_eq!(MUSICAL.chord_types, &[5, 7, 9, 11, 13]);
        assert!(MUSICAL.validate().is_ok());
    }

    #[test]
    fn test_performance_thresholds() {
        assert_eq!(PERFORMANCE.chord_lookup_max_ms, 1);
        assert_eq!(PERFORMANCE.chord_suggestion_max_ms, 50);
        assert!(PERFORMANCE.validate().is_ok());
    }

    #[test]
    fn test_algorithm_constants() {
        assert_eq!(ALGORITHM.context_bonus_forward, 1.7);
        assert_eq!(ALGORITHM.statistical_strength_divisor, 10000.0);
        assert_eq!(ALGORITHM.max_pattern_length, 20);
    }

    #[test]
    fn test_invalid_musical_constants() {
        let invalid = MusicalConstants {
            scale_degrees: 0,
            ..MUSICAL
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_invalid_performance_thresholds() {
        let invalid = PerformanceThresholds {
            chord_lookup_max_ms: 0,
            ..PERFORMANCE
        };
        assert!(invalid.validate().is_err());
    }
}
