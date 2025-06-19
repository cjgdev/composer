//! Scale fingerprint and scale-related data structures

use crate::error::{ChordTheoryError, ChordTheoryResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Scale fingerprint representing semitone positions in an octave
///
/// A 12-element array where each position represents a semitone:
/// - 1 = note is present in the scale
/// - 0 = note is absent from the scale
///
/// Example: C major = [1,0,1,0,1,1,0,1,0,1,0,1] (C,D,E,F,G,A,B)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScaleFingerprint([u8; 12]);

impl ScaleFingerprint {
    /// Create a new scale fingerprint
    pub fn new(semitones: [u8; 12]) -> ChordTheoryResult<Self> {
        // Validate that all values are 0 or 1
        for (i, &value) in semitones.iter().enumerate() {
            if value > 1 {
                return Err(ChordTheoryError::InvalidScaleFingerprint {
                    reason: format!("Position {} has value {}, must be 0 or 1", i, value),
                });
            }
        }

        // Validate that at least one note is present
        if semitones.iter().all(|&x| x == 0) {
            return Err(ChordTheoryError::InvalidScaleFingerprint {
                reason: "Scale must contain at least one note".to_string(),
            });
        }

        Ok(ScaleFingerprint(semitones))
    }

    /// Create from slice with validation
    pub fn from_slice(slice: &[u8]) -> ChordTheoryResult<Self> {
        if slice.len() != 12 {
            return Err(ChordTheoryError::InvalidScaleFingerprint {
                reason: format!(
                    "Scale fingerprint must have exactly 12 elements, got {}",
                    slice.len()
                ),
            });
        }

        let mut array = [0u8; 12];
        array.copy_from_slice(slice);
        Self::new(array)
    }

    /// Get the raw semitone array
    pub fn semitones(&self) -> &[u8; 12] {
        &self.0
    }

    /// Check if a chromatic note (0-11) is in the scale
    pub fn contains_chromatic(&self, chromatic: u8) -> bool {
        if chromatic >= 12 {
            return false;
        }
        self.0[chromatic as usize] == 1
    }

    /// Get all chromatic notes present in the scale
    pub fn chromatic_notes(&self) -> Vec<u8> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, &present)| present == 1)
            .map(|(i, _)| i as u8)
            .collect()
    }

    /// Count the number of notes in the scale
    pub fn note_count(&self) -> usize {
        self.0.iter().filter(|&&x| x == 1).count()
    }

    /// Check if this is a valid diatonic scale (7 notes)
    pub fn is_diatonic(&self) -> bool {
        self.note_count() == 7
    }

    /// Get the scale degrees (1-based) from chromatic positions
    pub fn scale_degrees(&self) -> Vec<u8> {
        self.chromatic_notes()
            .into_iter()
            .enumerate()
            .map(|(i, _)| (i + 1) as u8)
            .collect()
    }

    /// Convert chromatic note to scale degree in this scale
    pub fn chromatic_to_scale_degree(&self, chromatic: u8) -> Option<u8> {
        if !self.contains_chromatic(chromatic) {
            return None;
        }

        let scale_notes = self.chromatic_notes();
        scale_notes
            .iter()
            .position(|&note| note == chromatic)
            .map(|pos| (pos + 1) as u8)
    }

    /// Convert scale degree to chromatic note in this scale
    pub fn scale_degree_to_chromatic(&self, degree: u8) -> Option<u8> {
        if degree == 0 || degree > 7 {
            return None;
        }

        let scale_notes = self.chromatic_notes();
        if (degree as usize) <= scale_notes.len() {
            Some(scale_notes[(degree - 1) as usize])
        } else {
            None
        }
    }
}

impl Default for ScaleFingerprint {
    /// Default to C major scale
    fn default() -> Self {
        Self::major_scale()
    }
}

impl fmt::Display for ScaleFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notes = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let present_notes: Vec<&str> = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, &present)| present == 1)
            .map(|(i, _)| notes[i])
            .collect();
        write!(f, "[{}]", present_notes.join(", "))
    }
}

impl ScaleFingerprint {
    /// C major scale: [1,0,1,0,1,1,0,1,0,1,0,1]
    pub fn major_scale() -> Self {
        ScaleFingerprint([1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1])
    }

    /// Natural minor scale: [1,0,1,1,0,1,0,1,1,0,1,0]
    pub fn minor_scale() -> Self {
        ScaleFingerprint([1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0])
    }

    /// Harmonic minor scale: [1,0,1,1,0,1,0,1,1,0,0,1]
    pub fn harmonic_minor_scale() -> Self {
        ScaleFingerprint([1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1])
    }

    /// Chromatic scale: [1,1,1,1,1,1,1,1,1,1,1,1]
    pub fn chromatic_scale() -> Self {
        ScaleFingerprint([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    }

    /// Dorian mode: [1,0,1,1,0,1,0,1,0,1,1,0]
    pub fn dorian_scale() -> Self {
        ScaleFingerprint([1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0])
    }

    /// Mixolydian mode: [1,0,1,0,1,1,0,1,0,1,1,0]
    pub fn mixolydian_scale() -> Self {
        ScaleFingerprint([1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0])
    }
}

/// Named scale types for common scales
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScaleType {
    Major,
    Minor,
    HarmonicMinor,
    Dorian,
    Mixolydian,
    Chromatic,
    Custom(ScaleFingerprint),
}

impl ScaleType {
    /// Get the fingerprint for this scale type
    pub fn fingerprint(&self) -> ScaleFingerprint {
        match self {
            ScaleType::Major => ScaleFingerprint::major_scale(),
            ScaleType::Minor => ScaleFingerprint::minor_scale(),
            ScaleType::HarmonicMinor => ScaleFingerprint::harmonic_minor_scale(),
            ScaleType::Dorian => ScaleFingerprint::dorian_scale(),
            ScaleType::Mixolydian => ScaleFingerprint::mixolydian_scale(),
            ScaleType::Chromatic => ScaleFingerprint::chromatic_scale(),
            ScaleType::Custom(fp) => *fp,
        }
    }

    /// Parse scale type from string name
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "major" => Some(ScaleType::Major),
            "minor" => Some(ScaleType::Minor),
            "harmonic_minor" => Some(ScaleType::HarmonicMinor),
            "dorian" => Some(ScaleType::Dorian),
            "mixolydian" => Some(ScaleType::Mixolydian),
            "chromatic" => Some(ScaleType::Chromatic),
            _ => None,
        }
    }

    /// Get the string name of this scale type
    pub fn name(&self) -> &str {
        match self {
            ScaleType::Major => "major",
            ScaleType::Minor => "minor",
            ScaleType::HarmonicMinor => "harmonic_minor",
            ScaleType::Dorian => "dorian",
            ScaleType::Mixolydian => "mixolydian",
            ScaleType::Chromatic => "chromatic",
            ScaleType::Custom(_) => "custom",
        }
    }
}

impl fmt::Display for ScaleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_fingerprint_creation() {
        let major = ScaleFingerprint::major_scale();
        assert_eq!(major.semitones(), &[1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1]);
        assert!(major.is_diatonic());
        assert_eq!(major.note_count(), 7);
    }

    #[test]
    fn test_scale_fingerprint_validation() {
        // Invalid value > 1
        let invalid = ScaleFingerprint::new([2, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1]);
        assert!(invalid.is_err());

        // Empty scale
        let empty = ScaleFingerprint::new([0; 12]);
        assert!(empty.is_err());

        // Valid chromatic scale
        let chromatic = ScaleFingerprint::new([1; 12]);
        assert!(chromatic.is_ok());
    }

    #[test]
    fn test_from_slice() {
        let slice = &[1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1];
        let fingerprint = ScaleFingerprint::from_slice(slice).unwrap();
        assert_eq!(fingerprint, ScaleFingerprint::major_scale());

        // Wrong length
        let short_slice = &[1, 0, 1];
        assert!(ScaleFingerprint::from_slice(short_slice).is_err());
    }

    #[test]
    fn test_contains_chromatic() {
        let major = ScaleFingerprint::major_scale();
        assert!(major.contains_chromatic(0)); // C
        assert!(!major.contains_chromatic(1)); // C#
        assert!(major.contains_chromatic(2)); // D
        assert!(!major.contains_chromatic(3)); // D#
        assert!(major.contains_chromatic(4)); // E
    }

    #[test]
    fn test_chromatic_notes() {
        let major = ScaleFingerprint::major_scale();
        let notes = major.chromatic_notes();
        assert_eq!(notes, vec![0, 2, 4, 5, 7, 9, 11]); // C D E F G A B
    }

    #[test]
    fn test_scale_degree_conversion() {
        let major = ScaleFingerprint::major_scale();

        // Chromatic to scale degree
        assert_eq!(major.chromatic_to_scale_degree(0), Some(1)); // C = 1
        assert_eq!(major.chromatic_to_scale_degree(2), Some(2)); // D = 2
        assert_eq!(major.chromatic_to_scale_degree(1), None); // C# not in scale

        // Scale degree to chromatic
        assert_eq!(major.scale_degree_to_chromatic(1), Some(0)); // 1 = C
        assert_eq!(major.scale_degree_to_chromatic(2), Some(2)); // 2 = D
        assert_eq!(major.scale_degree_to_chromatic(8), None); // Invalid degree
    }

    #[test]
    fn test_scale_types() {
        assert_eq!(
            ScaleType::Major.fingerprint(),
            ScaleFingerprint::major_scale()
        );
        assert_eq!(
            ScaleType::Minor.fingerprint(),
            ScaleFingerprint::minor_scale()
        );

        assert_eq!(ScaleType::from_name("major"), Some(ScaleType::Major));
        assert_eq!(ScaleType::from_name("MAJOR"), Some(ScaleType::Major));
        assert_eq!(ScaleType::from_name("invalid"), None);

        assert_eq!(ScaleType::Major.name(), "major");
        assert_eq!(ScaleType::HarmonicMinor.name(), "harmonic_minor");
    }

    #[test]
    fn test_display() {
        let major = ScaleFingerprint::major_scale();
        let display = format!("{}", major);
        assert!(display.contains("C"));
        assert!(display.contains("G"));
        assert!(!display.contains("C#"));
    }

    #[test]
    fn test_different_scales() {
        let minor = ScaleFingerprint::minor_scale();
        assert_eq!(minor.semitones(), &[1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0]);
        assert!(minor.is_diatonic());

        let harmonic_minor = ScaleFingerprint::harmonic_minor_scale();
        assert_eq!(
            harmonic_minor.semitones(),
            &[1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1]
        );
        assert!(harmonic_minor.is_diatonic());

        let chromatic = ScaleFingerprint::chromatic_scale();
        assert_eq!(chromatic.note_count(), 12);
        assert!(!chromatic.is_diatonic());
    }
}
