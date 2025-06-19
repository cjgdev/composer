//! Roman numeral notation and graphic representation

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::fmt;

/// Roman numeral graphic representation of a chord
///
/// Provides complete visual representation including Roman numeral symbol,
/// figured bass notation, quality symbols, and all harmonic context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RomanNumeralGraphic {
    /// Roman numeral symbol (I, ii, V7, etc.)
    pub symbol: String,

    /// Figured bass notation (6, 65, 42, etc.)
    pub figured_bass: String,

    /// Quality symbols (°, ø, +, maj, m)
    pub quality: String,

    /// Applied notation (/V, /vi, etc.)
    pub applied: String,

    /// Borrowed chord indication
    pub borrowed: String,

    /// Visible alterations
    pub alterations: SmallVec<[String; 4]>,

    /// Suspension notations
    pub suspensions: SmallVec<[String; 2]>,

    /// Add tone notations
    pub adds: SmallVec<[String; 3]>,

    /// Omit tone notations
    pub omits: SmallVec<[String; 2]>,
}

impl Default for RomanNumeralGraphic {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            figured_bass: String::new(),
            quality: String::new(),
            applied: String::new(),
            borrowed: String::new(),
            alterations: SmallVec::new(),
            suspensions: SmallVec::new(),
            adds: SmallVec::new(),
            omits: SmallVec::new(),
        }
    }
}

impl RomanNumeralGraphic {
    /// Create a new Roman numeral graphic
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }

    /// Set the figured bass notation
    pub fn with_figured_bass(mut self, figured_bass: String) -> Self {
        self.figured_bass = figured_bass;
        self
    }

    /// Set the quality symbol
    pub fn with_quality(mut self, quality: String) -> Self {
        self.quality = quality;
        self
    }

    /// Set the applied chord notation
    pub fn with_applied(mut self, applied: String) -> Self {
        self.applied = applied;
        self
    }

    /// Set the borrowed chord indication
    pub fn with_borrowed(mut self, borrowed: String) -> Self {
        self.borrowed = borrowed;
        self
    }

    /// Add an alteration notation
    pub fn with_alteration(mut self, alteration: String) -> Self {
        if !self.alterations.contains(&alteration) {
            self.alterations.push(alteration);
        }
        self
    }

    /// Add a suspension notation
    pub fn with_suspension(mut self, suspension: String) -> Self {
        if !self.suspensions.contains(&suspension) {
            self.suspensions.push(suspension);
        }
        self
    }

    /// Add an add tone notation
    pub fn with_add(mut self, add: String) -> Self {
        if !self.adds.contains(&add) {
            self.adds.push(add);
        }
        self
    }

    /// Add an omit tone notation
    pub fn with_omit(mut self, omit: String) -> Self {
        if !self.omits.contains(&omit) {
            self.omits.push(omit);
        }
        self
    }

    /// Check if this is a major chord (uppercase Roman numeral)
    pub fn is_major(&self) -> bool {
        if self.symbol.is_empty() {
            return false;
        }
        self.symbol.chars().next().unwrap().is_uppercase()
    }

    /// Check if this is a minor chord (lowercase Roman numeral)
    pub fn is_minor(&self) -> bool {
        if self.symbol.is_empty() {
            return false;
        }
        self.symbol.chars().next().unwrap().is_lowercase()
    }

    /// Check if this has any figured bass notation
    pub fn has_figured_bass(&self) -> bool {
        !self.figured_bass.is_empty()
    }

    /// Check if this is an applied chord
    pub fn is_applied(&self) -> bool {
        !self.applied.is_empty()
    }

    /// Check if this is a borrowed chord
    pub fn is_borrowed(&self) -> bool {
        !self.borrowed.is_empty()
    }

    /// Check if this has quality markings
    pub fn has_quality(&self) -> bool {
        !self.quality.is_empty()
    }

    /// Get the base Roman numeral without any modifiers
    pub fn base_numeral(&self) -> String {
        // Extract just the Roman numeral part, removing any numbers or symbols
        self.symbol
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect()
    }

    /// Get the numeric extensions (7, 9, 11, 13) from the symbol
    pub fn numeric_extensions(&self) -> Vec<u8> {
        self.symbol
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0) as u8)
            .filter(|&n| n > 0)
            .collect()
    }

    /// Build the complete chord symbol string
    pub fn full_symbol(&self) -> String {
        let mut result = String::new();

        // Start with the basic symbol
        result.push_str(&self.symbol);

        // Add quality
        if !self.quality.is_empty() {
            result.push_str(&self.quality);
        }

        // Add figured bass
        if !self.figured_bass.is_empty() {
            result.push_str(&self.figured_bass);
        }

        // Add alterations
        for alteration in &self.alterations {
            result.push_str(&format!("({})", alteration));
        }

        // Add suspensions
        for suspension in &self.suspensions {
            result.push_str(&format!("sus{}", suspension));
        }

        // Add add tones
        for add in &self.adds {
            result.push_str(&format!("add{}", add));
        }

        // Add omit tones
        for omit in &self.omits {
            result.push_str(&format!("omit{}", omit));
        }

        // Add applied notation
        if !self.applied.is_empty() {
            result.push_str(&self.applied);
        }

        // Add borrowed indication
        if !self.borrowed.is_empty() {
            result.push_str(&format!("[{}]", self.borrowed));
        }

        result
    }

    /// Create a simplified version with just essential information
    pub fn simplified(&self) -> Self {
        Self {
            symbol: self.symbol.clone(),
            figured_bass: self.figured_bass.clone(),
            quality: self.quality.clone(),
            applied: self.applied.clone(),
            borrowed: self.borrowed.clone(),
            ..Default::default()
        }
    }
}

impl fmt::Display for RomanNumeralGraphic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_symbol())
    }
}

/// Roman numeral case type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RomanCase {
    Upper,
    Lower,
}

impl RomanCase {
    /// Apply case to a Roman numeral string
    pub fn apply(&self, numeral: &str) -> String {
        match self {
            RomanCase::Upper => numeral.to_uppercase(),
            RomanCase::Lower => numeral.to_lowercase(),
        }
    }
}

/// Quality indicator for chord symbols
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityType {
    Major,
    Minor,
    Diminished,
    HalfDiminished,
    Augmented,
    MajorSeventh,
    Custom(String),
}

impl QualityType {
    /// Get the standard symbol for this quality
    pub fn symbol(&self) -> &str {
        match self {
            QualityType::Major => "",
            QualityType::Minor => "m",
            QualityType::Diminished => "°",
            QualityType::HalfDiminished => "ø",
            QualityType::Augmented => "+",
            QualityType::MajorSeventh => "maj",
            QualityType::Custom(s) => s,
        }
    }

    /// Get the Roman numeral case for this quality
    pub fn roman_case(&self) -> RomanCase {
        match self {
            QualityType::Major | QualityType::Augmented | QualityType::MajorSeventh => {
                RomanCase::Upper
            },
            QualityType::Minor | QualityType::Diminished | QualityType::HalfDiminished => {
                RomanCase::Lower
            },
            QualityType::Custom(_) => RomanCase::Upper, // Default to upper for custom
        }
    }
}

impl fmt::Display for QualityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_numeral_creation() {
        let rn = RomanNumeralGraphic::new("I".to_string());
        assert_eq!(rn.symbol, "I");
        assert!(rn.figured_bass.is_empty());
        assert!(rn.is_major());
        assert!(!rn.is_minor());
    }

    #[test]
    fn test_builder_pattern() {
        let rn = RomanNumeralGraphic::new("V".to_string())
            .with_figured_bass("7".to_string())
            .with_applied("/V".to_string())
            .with_alteration("b9".to_string());

        assert_eq!(rn.symbol, "V");
        assert_eq!(rn.figured_bass, "7");
        assert_eq!(rn.applied, "/V");
        assert!(rn.alterations.contains(&"b9".to_string()));
    }

    #[test]
    fn test_major_minor_detection() {
        let major = RomanNumeralGraphic::new("I".to_string());
        assert!(major.is_major());
        assert!(!major.is_minor());

        let minor = RomanNumeralGraphic::new("ii".to_string());
        assert!(!minor.is_major());
        assert!(minor.is_minor());
    }

    #[test]
    fn test_queries() {
        let rn = RomanNumeralGraphic::new("V".to_string())
            .with_figured_bass("7".to_string())
            .with_applied("/V".to_string())
            .with_borrowed("harmonic_minor".to_string())
            .with_quality("°".to_string());

        assert!(rn.has_figured_bass());
        assert!(rn.is_applied());
        assert!(rn.is_borrowed());
        assert!(rn.has_quality());
    }

    #[test]
    fn test_base_numeral() {
        let rn = RomanNumeralGraphic::new("V7".to_string());
        assert_eq!(rn.base_numeral(), "V");

        let complex = RomanNumeralGraphic::new("vii°7".to_string());
        assert_eq!(complex.base_numeral(), "vii");
    }

    #[test]
    fn test_numeric_extensions() {
        let seventh = RomanNumeralGraphic::new("V7".to_string());
        assert_eq!(seventh.numeric_extensions(), vec![7]);

        let ninth = RomanNumeralGraphic::new("ii9".to_string());
        assert_eq!(ninth.numeric_extensions(), vec![9]);

        let complex = RomanNumeralGraphic::new("V713".to_string());
        assert_eq!(complex.numeric_extensions(), vec![7, 1, 3]); // Individual digits
    }

    #[test]
    fn test_full_symbol() {
        let simple = RomanNumeralGraphic::new("I".to_string());
        assert_eq!(simple.full_symbol(), "I");

        let complex = RomanNumeralGraphic::new("V".to_string())
            .with_figured_bass("7".to_string())
            .with_alteration("b9".to_string())
            .with_applied("/V".to_string());

        let full = complex.full_symbol();
        assert!(full.contains("V"));
        assert!(full.contains("7"));
        assert!(full.contains("b9"));
        assert!(full.contains("/V"));
    }

    #[test]
    fn test_simplified() {
        let complex = RomanNumeralGraphic::new("V".to_string())
            .with_figured_bass("7".to_string())
            .with_alteration("b9".to_string())
            .with_suspension("4".to_string())
            .with_applied("/V".to_string());

        let simplified = complex.simplified();
        assert_eq!(simplified.symbol, "V");
        assert_eq!(simplified.figured_bass, "7");
        assert_eq!(simplified.applied, "/V");
        assert!(simplified.alterations.is_empty());
        assert!(simplified.suspensions.is_empty());
    }

    #[test]
    fn test_display() {
        let rn = RomanNumeralGraphic::new("V7".to_string());
        assert_eq!(format!("{}", rn), "V7");

        let complex = RomanNumeralGraphic::new("ii".to_string())
            .with_figured_bass("65".to_string())
            .with_quality("°".to_string());
        let display = format!("{}", complex);
        assert!(display.contains("ii"));
        assert!(display.contains("65"));
        assert!(display.contains("°"));
    }

    #[test]
    fn test_roman_case() {
        assert_eq!(RomanCase::Upper.apply("i"), "I");
        assert_eq!(RomanCase::Lower.apply("V"), "v");
    }

    #[test]
    fn test_quality_type() {
        assert_eq!(QualityType::Major.symbol(), "");
        assert_eq!(QualityType::Minor.symbol(), "m");
        assert_eq!(QualityType::Diminished.symbol(), "°");
        assert_eq!(QualityType::HalfDiminished.symbol(), "ø");
        assert_eq!(QualityType::Augmented.symbol(), "+");

        assert_eq!(QualityType::Major.roman_case(), RomanCase::Upper);
        assert_eq!(QualityType::Minor.roman_case(), RomanCase::Lower);
        assert_eq!(QualityType::Diminished.roman_case(), RomanCase::Lower);
    }

    #[test]
    fn test_custom_quality() {
        let custom = QualityType::Custom("sus".to_string());
        assert_eq!(custom.symbol(), "sus");
        assert_eq!(custom.roman_case(), RomanCase::Upper);
    }

    #[test]
    fn test_deduplication() {
        let mut rn = RomanNumeralGraphic::new("I".to_string());
        rn = rn.with_alteration("b5".to_string());
        rn = rn.with_alteration("b5".to_string()); // Duplicate
        assert_eq!(rn.alterations.len(), 1);

        rn = rn.with_suspension("4".to_string());
        rn = rn.with_suspension("4".to_string()); // Duplicate
        assert_eq!(rn.suspensions.len(), 1);
    }
}
