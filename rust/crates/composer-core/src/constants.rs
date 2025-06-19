//! Musical theory constants and mappings

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Roman numeral labels (uppercase for major, lowercase for minor)
pub const UPPER_CASE_NUMERALS: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
pub const LOWER_CASE_NUMERALS: [&str; 7] = ["i", "ii", "iii", "iv", "v", "vi", "vii"];

/// Figured bass notation for different chord types and inversions
pub static FIGURED_BASS: Lazy<HashMap<u8, Vec<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(5, vec!["", "6", "64"]); // Triad inversions
    map.insert(7, vec!["7", "65", "43", "42"]); // Seventh chord inversions
    map.insert(9, vec!["9"]); // Ninth chords
    map.insert(11, vec!["11"]); // Eleventh chords
    map.insert(13, vec!["13"]); // Thirteenth chords
    map
});

/// Chord quality symbols for triads
pub static QUALITY_SYMBOLS_TRIADS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("minor_flat5", "°"); // Diminished
    map.insert("minor", "m"); // Minor
    map.insert("major", ""); // Major (no symbol)
    map.insert("augmented", "+"); // Augmented
    map
});

/// Chord quality symbols for seventh chords
pub static QUALITY_SYMBOLS_SEVENTHS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("diminished7", "°7"); // Fully diminished
    map.insert("half_diminished7", "ø7"); // Half diminished
    map.insert("minor7", "m7"); // Minor seventh
    map.insert("dominant7", "7"); // Dominant seventh
    map.insert("major7", "maj7"); // Major seventh
    map.insert("augmented7", "+7"); // Augmented seventh
    map
});

/// Alteration mapping to semitone offsets
pub static ALTERATION_MAP: Lazy<HashMap<&'static str, i8>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("#5", 1); // Sharp fifth
    map.insert("b5", -1); // Flat fifth
    map.insert("#9", 3); // Sharp ninth
    map.insert("b9", -3); // Flat ninth
    map.insert("#11", 4); // Sharp eleventh
    map.insert("b11", -4); // Flat eleventh
    map.insert("#13", 5); // Sharp thirteenth
    map.insert("b13", -5); // Flat thirteenth
    map
});

/// Voice leading penalty weights
pub static VOICE_LEADING_PENALTIES: Lazy<HashMap<&'static str, f64>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("parallel_fifths", 10.0);
    map.insert("parallel_octaves", 8.0);
    map.insert("large_leaps", 2.0);
    map.insert("voice_crossing", 3.0);
    map.insert("doubled_leading_tone", 5.0);
    map
});

/// Harmonic function classification by scale degree
pub static HARMONIC_FUNCTIONS: Lazy<HashMap<&'static str, Vec<u8>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("tonic", vec![1, 6, 3]); // I, vi, iii
    map.insert("predominant", vec![4, 2]); // IV, ii
    map.insert("dominant", vec![5, 7]); // V, vii°
    map
});

/// Scale degree function names
pub static SCALE_DEGREE_FUNCTIONS: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(1, "tonic");
    map.insert(2, "supertonic");
    map.insert(3, "mediant");
    map.insert(4, "subdominant");
    map.insert(5, "dominant");
    map.insert(6, "submediant");
    map.insert(7, "leading_tone");
    map
});

/// Valid chord types (number of notes) - imported from config
pub use composer_config::MUSICAL;

/// Valid alteration strings
pub const VALID_ALTERATIONS: [&str; 6] = ["b5", "#5", "b9", "#9", "#11", "b13"];

/// Valid suspension intervals
pub const VALID_SUSPENSIONS: [u8; 2] = [2, 4];

/// Valid add tone intervals  
pub const VALID_ADD_TONES: [u8; 3] = [4, 6, 9];

/// Valid omit tone intervals
pub const VALID_OMIT_TONES: [u8; 2] = [3, 5];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_numerals() {
        assert_eq!(UPPER_CASE_NUMERALS.len(), 7);
        assert_eq!(LOWER_CASE_NUMERALS.len(), 7);
        assert_eq!(UPPER_CASE_NUMERALS[0], "I");
        assert_eq!(LOWER_CASE_NUMERALS[0], "i");
    }

    #[test]
    fn test_figured_bass() {
        let triads = FIGURED_BASS.get(&5).unwrap();
        assert_eq!(triads, &vec!["", "6", "64"]);

        let sevenths = FIGURED_BASS.get(&7).unwrap();
        assert_eq!(sevenths, &vec!["7", "65", "43", "42"]);
    }

    #[test]
    fn test_quality_symbols() {
        assert_eq!(QUALITY_SYMBOLS_TRIADS.get("minor_flat5"), Some(&"°"));
        assert_eq!(QUALITY_SYMBOLS_SEVENTHS.get("major7"), Some(&"maj7"));
    }

    #[test]
    fn test_alteration_map() {
        assert_eq!(ALTERATION_MAP.get("#5"), Some(&1));
        assert_eq!(ALTERATION_MAP.get("b9"), Some(&-3));
    }

    #[test]
    fn test_harmonic_functions() {
        let tonic = HARMONIC_FUNCTIONS.get("tonic").unwrap();
        assert_eq!(tonic, &vec![1, 6, 3]);
    }

    #[test]
    fn test_scale_degree_functions() {
        assert_eq!(SCALE_DEGREE_FUNCTIONS.get(&1), Some(&"tonic"));
        assert_eq!(SCALE_DEGREE_FUNCTIONS.get(&5), Some(&"dominant"));
    }

    #[test]
    fn test_chord_types() {
        assert_eq!(crate::constants::MUSICAL.chord_types, &[5, 7, 9, 11, 13]);
    }

    #[test]
    fn test_valid_constants() {
        assert_eq!(VALID_ALTERATIONS.len(), 6);
        assert_eq!(VALID_SUSPENSIONS, [2, 4]);
        assert_eq!(VALID_ADD_TONES, [4, 6, 9]);
        assert_eq!(VALID_OMIT_TONES, [3, 5]);
    }
}
