//! Python bindings for the Composer music theory and AI library
//!
//! This module provides Python access to core Composer functionality including:
//! - Basic chord creation and manipulation
//! - Music theory analysis
//! - Binary chord serialization
//! - AI-powered chord suggestions and difficulty assessment

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

mod ai;
mod chord;
mod error;
mod scale;
mod serialization;
mod theory;

pub use ai::*;
pub use chord::*;
pub use error::*;
pub use scale::*;
pub use serialization::{
    chord_from_hex, chord_to_hex, deserialize_chord_from_binary, serialize_chord_to_binary, PyNote,
    PyTimeline, PyTokenLibrary, PyTrieNode,
};
pub use theory::{
    analyze_harmonic_function, chord_letter_to_lower_case, chord_letter_to_upper_case,
    get_chord_complexity, get_relative_chord_graphic, get_relative_scale_degrees,
    get_stable_scale_degrees, is_isotonal, is_valid_tri_sub, PyChordGraphic,
    PyRelativeScaleDegrees,
};

/// Get all configuration constants as a dictionary
#[pyfunction]
fn get_configuration_constants() -> HashMap<String, String> {
    let mut constants = HashMap::new();

    // Application metadata
    constants.insert(
        "APPLICATION_VERSION".to_string(),
        composer_config::APPLICATION.version.to_string(),
    );

    // Musical theory constants
    constants.insert(
        "SCALE_DEGREES".to_string(),
        composer_config::MUSICAL.scale_degrees.to_string(),
    );
    constants.insert(
        "CHROMATIC_NOTES".to_string(),
        composer_config::MUSICAL.chromatic_notes.to_string(),
    );
    constants.insert("OCTAVE_RANGE".to_string(), "10".to_string());
    constants.insert("MIDDLE_C_MIDI".to_string(), "60".to_string());
    constants.insert("DEFAULT_OCTAVE".to_string(), "4".to_string());
    constants.insert("CHORD_TYPES".to_string(), "[5, 7, 9, 11, 13]".to_string());
    constants.insert("MAX_INVERSIONS".to_string(), "4".to_string());
    constants.insert("MAX_EXTENSIONS".to_string(), "6".to_string());

    // Timing constants
    constants.insert(
        "TICKS_PER_BEAT".to_string(),
        composer_config::MUSICAL.ticks_per_beat.to_string(),
    );
    constants.insert("BEATS_PER_MEASURE".to_string(), "4".to_string());
    constants.insert("DEFAULT_TEMPO".to_string(), "120".to_string());
    constants.insert("MIN_TEMPO".to_string(), "60".to_string());
    constants.insert("MAX_TEMPO".to_string(), "200".to_string());

    // Analysis parameters
    constants.insert("COMPLEXITY_SCALE_MAX".to_string(), "10.0".to_string());
    constants.insert("NOVELTY_THRESHOLD".to_string(), "0.15".to_string());
    constants.insert("TENSION_SCALE_MAX".to_string(), "100.0".to_string());
    constants.insert("DIFFICULTY_PERCENTILE_MAX".to_string(), "99".to_string());

    // Performance thresholds
    constants.insert("CHORD_LOOKUP_MAX_MS".to_string(), "1".to_string());
    constants.insert("CHORD_SUGGESTION_MAX_MS".to_string(), "50".to_string());
    constants.insert("MUSIC_ANALYSIS_MAX_MS".to_string(), "200".to_string());
    constants.insert("ASSET_LOADING_MAX_MS".to_string(), "30000".to_string());
    constants.insert("UI_RESPONSE_MAX_MS".to_string(), "16".to_string());

    // Memory limits
    constants.insert("MEMORY_USAGE_MAX_MB".to_string(), "150".to_string());
    constants.insert("CACHE_SIZE_MAX_ENTRIES".to_string(), "10000".to_string());
    constants.insert("OBJECT_POOL_SIZE".to_string(), "1000".to_string());
    constants.insert("TRIE_MEMORY_MAX_MB".to_string(), "100".to_string());

    // Processing limits
    constants.insert("MAX_PATTERN_LENGTH".to_string(), "20".to_string());
    constants.insert("MAX_SUGGESTIONS".to_string(), "100".to_string());
    constants.insert("MAX_BATCH_SIZE".to_string(), "50".to_string());
    constants.insert("MAX_CONCURRENT_REQUESTS".to_string(), "10".to_string());

    // Error codes
    constants.insert(
        "ERROR_INVALID_CHORD_ROOT".to_string(),
        "invalid-chord-root".to_string(),
    );
    constants.insert(
        "ERROR_INVALID_CHORD_TYPE".to_string(),
        "invalid-chord-type".to_string(),
    );
    constants.insert(
        "ERROR_INVALID_INVERSION".to_string(),
        "invalid-inversion".to_string(),
    );
    constants.insert(
        "ERROR_INVALID_SCALE".to_string(),
        "invalid-scale-fingerprint".to_string(),
    );
    constants.insert(
        "ERROR_INCOMPATIBLE_ALTERATIONS".to_string(),
        "incompatible-alterations".to_string(),
    );
    constants.insert(
        "ERROR_MISSING_PROPERTY".to_string(),
        "missing-required-property".to_string(),
    );

    constants
}

/// Get the library version
#[pyfunction]
fn get_version() -> String {
    composer_config::APPLICATION.version.to_string()
}

/// The main Composer module for Python
#[pymodule]
fn composer(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core data structures
    m.add_class::<PyChord>()?;
    m.add_class::<PyScaleFingerprint>()?;
    m.add_class::<PyBorrowedScale>()?;

    // Roman numeral analysis classes
    m.add_class::<PyChordGraphic>()?;
    m.add_class::<PyRelativeScaleDegrees>()?;

    // Theory functions
    m.add_function(wrap_pyfunction!(theory::get_chord_complexity, m)?)?;
    m.add_function(wrap_pyfunction!(theory::get_relative_chord_graphic, m)?)?;
    m.add_function(wrap_pyfunction!(theory::get_stable_scale_degrees, m)?)?;
    m.add_function(wrap_pyfunction!(theory::get_relative_scale_degrees, m)?)?;
    m.add_function(wrap_pyfunction!(theory::is_valid_tri_sub, m)?)?;
    m.add_function(wrap_pyfunction!(theory::is_isotonal, m)?)?;
    m.add_function(wrap_pyfunction!(theory::analyze_harmonic_function, m)?)?;
    m.add_function(wrap_pyfunction!(theory::chord_letter_to_lower_case, m)?)?;
    m.add_function(wrap_pyfunction!(theory::chord_letter_to_upper_case, m)?)?;

    // Serialization classes
    m.add_class::<PyNote>()?;
    m.add_class::<PyTokenLibrary>()?;
    m.add_class::<PyTimeline>()?;
    m.add_class::<PyTrieNode>()?;

    // Basic serialization functions
    m.add_function(wrap_pyfunction!(
        serialization::serialize_chord_to_binary,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        serialization::deserialize_chord_from_binary,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(serialization::chord_to_hex, m)?)?;
    m.add_function(wrap_pyfunction!(serialization::chord_from_hex, m)?)?;

    // Tokenization functions
    m.add_function(wrap_pyfunction!(serialization::py_tokenize_duration, m)?)?;
    m.add_function(wrap_pyfunction!(serialization::py_parse_duration_token, m)?)?;
    m.add_function(wrap_pyfunction!(
        serialization::py_tokenize_chord_as_raw,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(serialization::py_detokenize_cluster, m)?)?;
    m.add_function(wrap_pyfunction!(serialization::py_detokenize_midi_like, m)?)?;

    // Hash and compression functions
    m.add_function(wrap_pyfunction!(serialization::py_fast_hash, m)?)?;
    m.add_function(wrap_pyfunction!(serialization::py_fold_hash, m)?)?;
    m.add_function(wrap_pyfunction!(serialization::py_scale40_encode, m)?)?;
    m.add_function(wrap_pyfunction!(serialization::py_scale40_decode, m)?)?;

    // Trie serialization functions
    m.add_function(wrap_pyfunction!(serialization::py_serialize_trie, m)?)?;
    m.add_function(wrap_pyfunction!(serialization::py_deserialize_trie, m)?)?;
    m.add_function(wrap_pyfunction!(
        serialization::py_validate_binary_format,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(serialization::py_reduce_chord_vocab, m)?)?;
    m.add_function(wrap_pyfunction!(
        serialization::py_augment_with_repeated,
        m
    )?)?;

    // Token validation functions
    m.add_function(wrap_pyfunction!(serialization::py_validate_token, m)?)?;
    m.add_function(wrap_pyfunction!(
        serialization::py_validate_duration_token,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        serialization::py_validate_raw_note_token,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        serialization::py_validate_octave_token,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        serialization::py_validate_chord_cluster_token,
        m
    )?)?;

    // Configuration and constants functions
    m.add_function(wrap_pyfunction!(get_configuration_constants, m)?)?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(
        serialization::get_serialization_constants,
        m
    )?)?;

    // AI classes and functions
    m.add_class::<PyAiEngine>()?;
    m.add_class::<PySuggestionContext>()?;
    m.add_class::<PySuggestionConfig>()?;
    m.add_class::<PyChordSuggestion>()?;
    m.add_class::<PyDifficultyAssessment>()?;
    m.add_class::<PyBassHarmonization>()?;

    // Constants
    let constants = PyDict::new(_py);
    constants.set_item("VERSION", composer_config::APPLICATION.version)?;
    constants.set_item("TICKS_PER_BEAT", composer_config::MUSICAL.ticks_per_beat)?;
    constants.set_item("CHROMATIC_NOTES", composer_config::MUSICAL.chromatic_notes)?;
    constants.set_item("SCALE_DEGREES", composer_config::MUSICAL.scale_degrees)?;
    constants.set_item("MAX_PATTERN_LENGTH", 20)?;
    constants.set_item("COMPLEXITY_SCALE_MAX", 10.0)?;
    m.add("constants", constants)?;

    // Error types
    m.add("ComposerError", _py.get_type::<PyComposerError>())?;

    // Add version as module attribute
    m.add("__version__", composer_config::APPLICATION.version)?;

    Ok(())
}
