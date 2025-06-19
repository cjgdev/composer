//! Musical constants for serialization and tokenization

/// Beat subdivision resolution - use config instead
pub const TICKS_PER_BEAT: u32 = composer_config::MUSICAL.ticks_per_beat as u32;

/// Beat to tick conversion multiplier
pub const BEAT_TO_TICK_MULTIPLIER: u32 = 24;

/// Maximum representable beat value in 16-bit space
pub const MAX_BEAT_VALUE: u32 = 65536 / 24; // floor(65536 / 24) = 2730

/// Minimum octave range
pub const OCTAVE_RANGE_MIN: u8 = 2;

/// Maximum octave range  
pub const OCTAVE_RANGE_MAX: u8 = 7;

/// Semitones per octave
pub const CHROMATIC_RANGE: u8 = 12;

/// Chord type mapping for binary serialization - use config instead
pub use composer_config::MUSICAL;

/// Default alteration names for binary encoding/decoding
pub const ALTERATION_NAMES: [&str; 6] = ["b5", "#5", "b9", "#9", "#11", "b13"];

/// Token prefixes
pub const DURATION_TOKEN_PREFIX: &str = "D_";
pub const RAW_NOTE_TOKEN_PREFIX: &str = "R_";
pub const OCTAVE_TOKEN_PREFIX: &str = "O_";
pub const REST_NOTE_TOKEN: &str = "NOTE-REST";
pub const REST_CHORD_TOKEN: &str = "CHORD-REST";

/// Token validation patterns
pub const DURATION_PATTERN: &str = r"^D_[0-9a-f]+$";
pub const RAW_NOTE_PATTERN: &str = r"^R_[0-9a-b]$";
pub const OCTAVE_PATTERN: &str = r"^O_[2-7]$";
pub const CHORD_CLUSTER_PATTERN: &str = r"^(R_[0-9a-b]-?)+$";

/// Error messages
pub const ERROR_INVALID_CHORD_DATA: &str = "Invalid chord data structure";
pub const ERROR_UNSUPPORTED_VERSION: &str = "Unsupported serialization version";
pub const ERROR_CORRUPTED_BINARY: &str = "Corrupted binary data";
pub const ERROR_TOKEN_LIBRARY_MISSING: &str = "Token library not initialized";
pub const ERROR_INVALID_TOKEN_FORMAT: &str = "Invalid token format";
