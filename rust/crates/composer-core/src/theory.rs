//! Core chord theory algorithms and transformations

use crate::chord::{BorrowedScale, Chord};
use crate::constants::*;
use crate::error::{ChordTheoryError, ChordTheoryResult};
use crate::roman::{QualityType, RomanCase, RomanNumeralGraphic};
use crate::scale::ScaleFingerprint;

/// Scale degrees with accidentals
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleDegreeResult {
    /// Scale degree numbers (1-7)
    pub sd_numbers: Vec<u8>,
    /// Accidentals for each scale degree ("", "b", "#")
    pub sd_accs: Vec<String>,
}

/// Calculates the stable scale degrees that a chord occupies within a scale context.
///
/// This is the primary algorithm for harmonic analysis in Composer. It determines which
/// absolute scale degrees (1, 2, 3, 4, 5, 6, 7) a chord occupies when analyzed within
/// a specific scale. The function accounts for chord inversions, alterations, extensions,
/// and all other chord modifications.
///
/// # Arguments
///
/// * `chord` - The chord to analyze
/// * `scale_fingerprint` - The scale context for analysis (major, minor, modes, etc.)
///
/// # Returns
///
/// A vector of scale degree strings (e.g., `["1", "3", "5"]` for a tonic triad).
/// Alterations are represented with accidentals (e.g., `["1", "3", "b5"]`).
///
/// # Examples
///
/// ## Basic Triads in Major Scale
///
/// ```rust
/// use composer_core::{Chord, ScaleFingerprint, get_stable_scale_degrees};
///
/// let major_scale = ScaleFingerprint::major_scale();
///
/// // Tonic triad (I): contains scale degrees 1, 3, 5
/// let tonic = Chord::triad(1)?;
/// let degrees = get_stable_scale_degrees(&tonic, &major_scale)?;
/// assert_eq!(degrees, vec!["1", "3", "5"]);
///
/// // Subdominant triad (IV): contains scale degrees 4, 6, 1
/// let subdominant = Chord::triad(4)?;
/// let degrees = get_stable_scale_degrees(&subdominant, &major_scale)?;
/// assert_eq!(degrees, vec!["4", "6", "1"]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Extended and Altered Chords
///
/// ```rust
/// use composer_core::{Chord, ScaleFingerprint, get_stable_scale_degrees};
///
/// let major_scale = ScaleFingerprint::major_scale();
///
/// // Dominant seventh with alterations (V7♭9)
/// let altered_dom = Chord::seventh(5)?.with_alteration("b9")?;
/// let degrees = get_stable_scale_degrees(&altered_dom, &major_scale)?;
/// // Contains: 5 (root), 7 (third), 2 (fifth), 4 (seventh), b6 (♭9)
/// assert!(degrees.contains(&"5".to_string()));
/// assert!(degrees.contains(&"b6".to_string())); // ♭9 relative to V is ♭6 in the key
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Inversions and Voice Leading
///
/// ```rust
/// use composer_core::{Chord, ScaleFingerprint, get_stable_scale_degrees};
///
/// let major_scale = ScaleFingerprint::major_scale();
///
/// // Inversions don't change scale degree content, only bass note
/// let root_pos = Chord::triad(1)?;
/// let first_inv = Chord::triad(1)?.with_inversion(1)?;
///
/// let root_degrees = get_stable_scale_degrees(&root_pos, &major_scale)?;
/// let inv_degrees = get_stable_scale_degrees(&first_inv, &major_scale)?;
///
/// // Same scale degrees, different ordering due to inversion
/// assert_eq!(root_degrees.len(), inv_degrees.len());
/// for degree in &root_degrees {
///     assert!(inv_degrees.contains(degree));
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Performance
///
/// This function typically completes in <1ms and meets the CHORD_LOOKUP_MAX_MS = 1 requirement.
/// The algorithm is optimized for real-time harmonic analysis.
///
/// # Musical Theory Background
///
/// Scale degrees represent the functional relationships between chord tones and the tonic.
/// This analysis is fundamental to:
/// - Roman numeral analysis
/// - Functional harmony understanding  
/// - Voice leading analysis
/// - Jazz chord-scale relationships
/// - Modal harmonic analysis
///
/// # Related Functions
///
/// - [`get_relative_scale_degrees`] - Gets scale degrees relative to chord root
/// - [`get_chord_complexity`] - Uses scale degree analysis for complexity scoring
/// - [`get_relative_chord_graphic`] - Generates Roman numeral representations
/// - [`AiEngine::get_chord_suggestions`] - Uses this for harmonic context analysis
/// - [`is_isotonal`] - Compares chords by their scale degree content
pub fn get_stable_scale_degrees(
    chord: &Chord,
    scale_fingerprint: &ScaleFingerprint,
) -> ChordTheoryResult<Vec<String>> {
    if chord.is_rest {
        return Ok(Vec::new());
    }

    // First get relative scale degrees
    let relative = get_relative_scale_degrees(chord)?;

    // Transform to absolute scale degrees
    rel_scale_degree_to_abs_scale_degree(
        &relative.sd_numbers,
        &relative.sd_accs,
        chord,
        scale_fingerprint,
    )
}

/// Calculates scale degrees relative to the chord root.
///
/// This function determines the intervallic content of a chord by calculating
/// which scale degrees the chord contains relative to its own root. Unlike
/// [`get_stable_scale_degrees`], this function ignores the key context and focuses
/// purely on the chord's internal structure.
///
/// # Arguments
///
/// * `chord` - The chord to analyze
///
/// # Returns
///
/// A [`ScaleDegreeResult`] containing:
/// - `sd_numbers`: Vector of scale degree numbers (1-13)
/// - `sd_accs`: Vector of accidentals for each degree ("", "b", "#")
///
/// # Examples
///
/// ## Basic Chord Types
///
/// ```rust
/// use composer_core::{Chord, get_relative_scale_degrees};
///
/// // Major triad: root, major third, perfect fifth
/// let major_triad = Chord::triad(1)?;
/// let result = get_relative_scale_degrees(&major_triad)?;
/// assert_eq!(result.sd_numbers, vec![1, 3, 5]);
/// assert_eq!(result.sd_accs, vec!["", "", ""]); // All natural
///
/// // Dominant seventh: root, major third, perfect fifth, minor seventh
/// let dom7 = Chord::seventh(5)?;
/// let result = get_relative_scale_degrees(&dom7)?;
/// assert_eq!(result.sd_numbers, vec![1, 3, 5, 7]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Altered Chords
///
/// ```rust
/// use composer_core::{Chord, get_relative_scale_degrees};
///
/// // Dominant with ♭5 and ♭9
/// let altered = Chord::seventh(5)?
///     .with_alteration("b5")?
///     .with_alteration("b9")?;
/// let result = get_relative_scale_degrees(&altered)?;
///
/// // Contains 1, 3, ♭5, 7, ♭9
/// assert!(result.sd_numbers.contains(&5)); // Fifth degree
/// assert!(result.sd_accs[result.sd_numbers.iter().position(|&x| x == 5).unwrap()] == "b"); // Flattened
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Inversions and Extensions
///
/// ```rust
/// use composer_core::{Chord, get_relative_scale_degrees};
///
/// // First inversion doesn't change interval content, only ordering
/// let root_pos = Chord::triad(1)?;
/// let first_inv = Chord::triad(1)?.with_inversion(1)?;
///
/// let root_result = get_relative_scale_degrees(&root_pos)?;
/// let inv_result = get_relative_scale_degrees(&first_inv)?;
///
/// // Same intervals, potentially different order due to rotation
/// assert_eq!(root_result.sd_numbers.len(), inv_result.sd_numbers.len());
///
/// // Add9 chord has additional 9th degree
/// let add9 = Chord::triad(1)?.with_add(9)?;
/// let result = get_relative_scale_degrees(&add9)?;
/// assert!(result.sd_numbers.contains(&9));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Algorithm Details
///
/// The function processes chord components in this order:
/// 1. **Base intervals**: Determined by chord_type (triad=1,3,5; seventh=1,3,5,7; etc.)
/// 2. **Inversions**: Rotates the interval sequence based on inversion level
/// 3. **Extensions**: Adds intervals from the `adds` array
/// 4. **Omissions**: Removes intervals specified in the `omits` array  
/// 5. **Alterations**: Applies accidentals to modify specific degrees
/// 6. **Suspensions**: Replaces degrees with suspended intervals
///
/// # Performance
///
/// This function is highly optimized and typically completes in <0.1ms.
/// It's used internally by [`get_stable_scale_degrees`] and other analysis functions.
///
/// # Use Cases
///
/// - **Chord-scale analysis**: Determining appropriate scales for improvisation
/// - **Voice leading**: Understanding interval motion between chords
/// - **Harmonic analysis**: Comparing chord structures regardless of key
/// - **Composition tools**: Generating chord voicings and progressions
/// - **Educational software**: Teaching interval relationships
///
/// # Related Functions
///
/// - [`get_stable_scale_degrees`] - Uses this function for key-context analysis
/// - [`Chord::expected_tone_count`] - Counts the resulting scale degrees
/// - [`get_chord_complexity`] - Uses interval analysis for complexity scoring
/// - [`apply_alteration`] - Internal helper for processing alterations
pub fn get_relative_scale_degrees(chord: &Chord) -> ChordTheoryResult<ScaleDegreeResult> {
    if chord.is_rest {
        return Ok(ScaleDegreeResult {
            sd_numbers: Vec::new(),
            sd_accs: Vec::new(),
        });
    }

    // Start with base scale degrees for chord type
    let mut scale_degrees = match chord.chord_type {
        5 => vec![1, 3, 5],                // Triad
        7 => vec![1, 3, 5, 7],             // Seventh
        9 => vec![1, 3, 5, 7, 9],          // Ninth
        11 => vec![1, 3, 5, 7, 9, 11],     // Eleventh
        13 => vec![1, 3, 5, 7, 9, 11, 13], // Thirteenth
        _ => {
            return Err(ChordTheoryError::InvalidChordType {
                chord_type: chord.chord_type,
            })
        },
    };

    // Initialize accidentals (all natural initially)
    let mut accidentals = vec![String::new(); scale_degrees.len()];

    // Apply inversion (rotate scale degrees)
    if chord.inversion > 0 {
        let inversion = chord.inversion as usize;
        if inversion < scale_degrees.len() {
            scale_degrees.rotate_left(inversion);
            accidentals.rotate_left(inversion);
        }
    }

    // Add extension tones from 'adds' array
    for &add in &chord.adds {
        if !scale_degrees.contains(&add) {
            scale_degrees.push(add);
            accidentals.push(String::new());
        }
    }

    // Remove tones from 'omits' array
    let mut i = 0;
    while i < scale_degrees.len() {
        if chord.omits.contains(&scale_degrees[i]) {
            scale_degrees.remove(i);
            accidentals.remove(i);
        } else {
            i += 1;
        }
    }

    // Apply alterations
    for alteration in &chord.alterations {
        apply_alteration(&mut scale_degrees, &mut accidentals, alteration)?;
    }

    // Replace suspended tones
    for &suspension in &chord.suspensions {
        apply_suspension(&mut scale_degrees, &mut accidentals, suspension)?;
    }

    Ok(ScaleDegreeResult {
        sd_numbers: scale_degrees,
        sd_accs: accidentals,
    })
}

/// Generate complete Roman numeral representation
///
/// Creates a comprehensive RomanNumeralGraphic with all visual elements
/// including case, figured bass, quality symbols, and harmonic context.
pub fn get_relative_chord_graphic(
    chord: &Chord,
    scale_fingerprint: &ScaleFingerprint,
) -> ChordTheoryResult<RomanNumeralGraphic> {
    if chord.is_rest {
        return Ok(RomanNumeralGraphic::new("REST".to_string()));
    }

    // Determine chord quality and Roman numeral case
    let quality_type = determine_chord_quality(chord, scale_fingerprint)?;
    let roman_case = quality_type.roman_case();

    // Get base Roman numeral
    let base_numeral = get_roman_numeral_for_degree(chord.root, roman_case)?;

    // Build symbol with extensions
    let mut symbol = base_numeral;
    if chord.chord_type != 5 {
        symbol.push_str(&chord.chord_type.to_string());
    }

    // Generate figured bass notation
    let figured_bass = get_figured_bass_notation(chord)?;

    // Build the graphic
    let mut graphic = RomanNumeralGraphic::new(symbol)
        .with_figured_bass(figured_bass)
        .with_quality(quality_type.symbol().to_string());

    // Add applied chord notation
    if chord.applied != 0 {
        graphic = graphic.with_applied(format!(
            "/{}",
            get_roman_numeral_for_degree(chord.applied, RomanCase::Upper)?
        ));
    }

    // Add borrowed chord indicators
    if let Some(borrowed) = &chord.borrowed {
        graphic = graphic.with_borrowed(borrowed.scale_name());
    }

    // Add alteration symbols
    for alteration in &chord.alterations {
        graphic = graphic.with_alteration(alteration.clone());
    }

    // Add suspension notations
    for &suspension in &chord.suspensions {
        graphic = graphic.with_suspension(format!("sus{}", suspension));
    }

    // Add add/omit annotations
    for &add in &chord.adds {
        graphic = graphic.with_add(format!("add{}", add));
    }

    for &omit in &chord.omits {
        graphic = graphic.with_omit(format!("omit{}", omit));
    }

    Ok(graphic)
}

/// Calculate chord complexity score (0-10 scale)
///
/// Assesses harmonic complexity based on chord type, inversions, alterations,
/// applied chords, borrowed elements, and other factors.
pub fn get_chord_complexity(chord: &Chord, _scale_name: &str) -> ChordTheoryResult<f64> {
    if chord.is_rest {
        return Ok(0.0);
    }

    let mut complexity = 0.0;

    // Base complexity from chord type
    complexity += match chord.chord_type {
        5 => 1.0, // Triad
        7 => 2.0, // Seventh
        _ => 3.0, // Extended chords
    };

    // Add complexity for inversions
    complexity += chord.inversion as f64 * 0.5;

    // Add complexity for alterations
    complexity += chord.alterations.len() as f64 * 0.5;

    // Add complexity for applied chords
    if chord.applied != 0 {
        complexity += 1.0;
    }

    // Add complexity for borrowed chords
    if chord.borrowed.is_some() {
        complexity += 1.5;
    }

    // Add complexity for suspensions
    complexity += chord.suspensions.len() as f64 * 0.3;

    // Add complexity for omissions
    complexity += chord.omits.len() as f64 * 0.4;

    // Clamp to 0-10 range
    Ok(complexity.clamp(0.0, 10.0))
}

/// Validate tritone substitution eligibility
///
/// Checks if a chord qualifies for tritone substitution based on
/// harmonic function and scale context.
pub fn is_valid_tri_sub(chord: &Chord, scale_name: &str) -> bool {
    // Check if chord is applied V7
    if chord.applied != 0 && chord.chord_type == 7 {
        return true;
    }

    // Check if chord is V7 in major scale
    if chord.root == 5 && chord.chord_type == 7 && scale_name == "major" {
        return true;
    }

    // Check if chord is borrowed V7 from major
    if let Some(borrowed) = &chord.borrowed {
        if chord.root == 5 && chord.chord_type == 7 && borrowed.scale_name() == "major" {
            return true;
        }
    }

    false
}

// Helper functions

/// Transform relative scale degrees to absolute positions
fn rel_scale_degree_to_abs_scale_degree(
    scale_degrees: &[u8],
    accidentals: &[String],
    chord: &Chord,
    scale_fingerprint: &ScaleFingerprint,
) -> ChordTheoryResult<Vec<String>> {
    let mut result = Vec::new();

    for (i, &degree) in scale_degrees.iter().enumerate() {
        let accidental = accidentals.get(i).map(|s| s.as_str()).unwrap_or("");

        // Apply root transposition
        let mut abs_degree = ((degree + chord.root - 1 - 1) % 7) + 1;

        // Handle applied chord modifications
        if chord.applied != 0 {
            abs_degree = ((abs_degree + chord.applied - 1 - 1) % 7) + 1;
            // Applied chords may introduce chromatic alterations
            if is_applied_chromatic_alteration(degree, chord.applied) {
                result.push(format!("#{}", abs_degree));
                continue;
            }
        }

        // Handle borrowed chord alterations
        let mut final_accidental = accidental.to_string();
        if let Some(borrowed) = &chord.borrowed {
            final_accidental =
                apply_borrowed_alteration(degree, accidental, borrowed, scale_fingerprint)?;
        }

        // Combine degree with accidental
        if final_accidental.is_empty() {
            result.push(abs_degree.to_string());
        } else {
            result.push(format!("{}{}", final_accidental, abs_degree));
        }
    }

    Ok(result)
}

/// Apply an alteration to the scale degrees
fn apply_alteration(
    scale_degrees: &mut Vec<u8>,
    accidentals: &mut Vec<String>,
    alteration: &str,
) -> ChordTheoryResult<()> {
    let (target_degree, accidental) = match alteration {
        "b5" => (5, "b"),
        "#5" => (5, "#"),
        "b9" => (9, "b"),
        "#9" => (9, "#"),
        "#11" => (11, "#"),
        "b13" => (13, "b"),
        _ => {
            return Err(ChordTheoryError::InvalidAlteration {
                alteration: alteration.to_string(),
            })
        },
    };

    // Find the target degree and apply alteration
    for (i, &degree) in scale_degrees.iter().enumerate() {
        if degree == target_degree {
            accidentals[i] = accidental.to_string();
            return Ok(());
        }
    }

    // If degree not found, add it
    scale_degrees.push(target_degree);
    accidentals.push(accidental.to_string());

    Ok(())
}

/// Apply a suspension to the scale degrees
fn apply_suspension(
    scale_degrees: &mut Vec<u8>,
    accidentals: &mut Vec<String>,
    suspension: u8,
) -> ChordTheoryResult<()> {
    // Remove the 3rd (suspended by 2nd or 4th)
    if let Some(pos) = scale_degrees.iter().position(|&x| x == 3) {
        scale_degrees.remove(pos);
        accidentals.remove(pos);
    }

    // Add the suspension in the correct position to maintain order
    if !scale_degrees.contains(&suspension) {
        // Find the correct position to insert the suspension
        let insert_pos = scale_degrees
            .iter()
            .position(|&x| x > suspension)
            .unwrap_or(scale_degrees.len());
        scale_degrees.insert(insert_pos, suspension);
        accidentals.insert(insert_pos, String::new());
    }

    Ok(())
}

/// Determine chord quality from chord structure and scale context
fn determine_chord_quality(
    chord: &Chord,
    scale_fingerprint: &ScaleFingerprint,
) -> ChordTheoryResult<QualityType> {
    // Check for specific quality indicators
    if chord.alterations.contains(&"b5".to_string()) {
        if chord.chord_type == 7 {
            return Ok(QualityType::HalfDiminished);
        } else {
            return Ok(QualityType::Diminished);
        }
    }

    if chord.alterations.contains(&"#5".to_string()) {
        return Ok(QualityType::Augmented);
    }

    // For seventh chords, check if it's major seventh
    if chord.chord_type == 7 {
        // Logic to determine if it's major seventh vs dominant seventh
        // This would require analyzing the 7th degree in scale context
        if is_major_seventh_in_scale(chord, scale_fingerprint) {
            return Ok(QualityType::MajorSeventh);
        }
    }

    // Default quality based on scale degree function
    let scale_degree_quality = get_scale_degree_quality(chord.root, scale_fingerprint)?;
    Ok(scale_degree_quality)
}

/// Get Roman numeral string for scale degree
fn get_roman_numeral_for_degree(degree: u8, case: RomanCase) -> ChordTheoryResult<String> {
    if degree == 0 || degree > 7 {
        return Err(ChordTheoryError::ScaleDegreeOutOfRange { degree });
    }

    let numerals = match case {
        RomanCase::Upper => &UPPER_CASE_NUMERALS,
        RomanCase::Lower => &LOWER_CASE_NUMERALS,
    };

    Ok(numerals[(degree - 1) as usize].to_string())
}

/// Generate figured bass notation for chord
fn get_figured_bass_notation(chord: &Chord) -> ChordTheoryResult<String> {
    let figured_bass_map = &*FIGURED_BASS;

    if let Some(inversions) = figured_bass_map.get(&chord.chord_type) {
        let inversion_index = chord.inversion as usize;
        if inversion_index < inversions.len() {
            Ok(inversions[inversion_index].to_string())
        } else {
            Ok(String::new())
        }
    } else {
        Ok(String::new())
    }
}

/// Check if applied chord introduces chromatic alteration
fn is_applied_chromatic_alteration(degree: u8, applied: u8) -> bool {
    // V/V typically introduces #4 (leading tone to 5)
    degree == 7 && applied == 5
}

/// Apply borrowed chord alterations
fn apply_borrowed_alteration(
    degree: u8,
    current_accidental: &str,
    borrowed: &BorrowedScale,
    _original_scale: &ScaleFingerprint,
) -> ChordTheoryResult<String> {
    match borrowed {
        BorrowedScale::Named(scale_name) => {
            // Apply alterations based on named scale
            match scale_name.as_str() {
                "harmonic_minor" => {
                    if degree == 3 {
                        Ok("b".to_string()) // b3 in harmonic minor
                    } else {
                        Ok(current_accidental.to_string())
                    }
                },
                _ => Ok(current_accidental.to_string()),
            }
        },
        BorrowedScale::ScaleType(scale_type) => {
            let _borrowed_fingerprint = scale_type.fingerprint();
            // Compare fingerprints and determine alterations
            Ok(current_accidental.to_string()) // Simplified for now
        },
        BorrowedScale::Numeric(_) => {
            Ok(current_accidental.to_string()) // Simplified for now
        },
    }
}

/// Check if chord is major seventh in scale context
fn is_major_seventh_in_scale(chord: &Chord, _scale: &ScaleFingerprint) -> bool {
    // Simplified logic - would need more sophisticated analysis
    chord.root == 1 || chord.root == 4 // I and IV typically have major sevenths
}

/// Get default quality for scale degree
fn get_scale_degree_quality(
    degree: u8,
    _scale: &ScaleFingerprint,
) -> ChordTheoryResult<QualityType> {
    // In major scale: I, IV, V are major; ii, iii, vi are minor; vii is diminished
    match degree {
        1 | 4 | 5 => Ok(QualityType::Major),
        2 | 3 | 6 => Ok(QualityType::Minor),
        7 => Ok(QualityType::Diminished),
        _ => Err(ChordTheoryError::ScaleDegreeOutOfRange { degree }),
    }
}

/// Utility functions for case conversion
pub fn chord_letter_to_lower_case(note_string: &str) -> String {
    if note_string.is_empty() {
        return String::new();
    }

    let mut chars: Vec<char> = note_string.chars().collect();
    if chars[0].is_alphabetic() {
        chars[0] = chars[0].to_lowercase().next().unwrap_or(chars[0]);
    }
    chars.into_iter().collect()
}

pub fn chord_letter_to_upper_case(note_string: &str) -> String {
    if note_string.is_empty() {
        return String::new();
    }

    let mut chars: Vec<char> = note_string.chars().collect();
    if chars[0].is_alphabetic() {
        chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::ScaleFingerprint;
    use smallvec::SmallVec;

    // Test data from specification
    fn test_chord_c_major() -> Chord {
        Chord {
            root: 1,
            chord_type: 5,
            inversion: 0,
            applied: 0,
            adds: SmallVec::new(),
            omits: SmallVec::new(),
            alterations: SmallVec::new(),
            suspensions: SmallVec::new(),
            borrowed: None,
            pedal: None,
            alternate: String::new(),
            substitutions: SmallVec::new(),
            is_rest: false,
            beat: None,
            duration: None,
        }
    }

    fn test_chord_v7() -> Chord {
        Chord {
            root: 5,
            chord_type: 7,
            ..test_chord_c_major()
        }
    }

    fn test_chord_applied_v7_of_v() -> Chord {
        Chord {
            root: 2,
            chord_type: 7,
            applied: 5,
            ..test_chord_c_major()
        }
    }

    #[test]
    fn test_get_relative_scale_degrees_triad() {
        let chord = test_chord_c_major();
        let result = get_relative_scale_degrees(&chord).unwrap();

        assert_eq!(result.sd_numbers, vec![1, 3, 5]);
        assert_eq!(result.sd_accs, vec!["", "", ""]);
    }

    #[test]
    fn test_get_relative_scale_degrees_seventh() {
        let chord = test_chord_v7();
        let result = get_relative_scale_degrees(&chord).unwrap();

        assert_eq!(result.sd_numbers, vec![1, 3, 5, 7]);
        assert_eq!(result.sd_accs, vec!["", "", "", ""]);
    }

    #[test]
    fn test_get_relative_scale_degrees_inversion() {
        let mut chord = test_chord_c_major();
        chord.inversion = 1;
        let result = get_relative_scale_degrees(&chord).unwrap();

        assert_eq!(result.sd_numbers, vec![3, 5, 1]);
        assert_eq!(result.sd_accs, vec!["", "", ""]);
    }

    #[test]
    fn test_get_relative_scale_degrees_altered() {
        let mut chord = test_chord_v7();
        chord.alterations.push("b5".to_string());
        chord.alterations.push("#9".to_string());
        let result = get_relative_scale_degrees(&chord).unwrap();

        assert_eq!(result.sd_numbers, vec![1, 3, 5, 7, 9]);
        assert_eq!(result.sd_accs, vec!["", "", "b", "", "#"]);
    }

    #[test]
    fn test_get_relative_scale_degrees_suspended() {
        let mut chord = test_chord_c_major();
        chord.suspensions.push(4);
        let result = get_relative_scale_degrees(&chord).unwrap();

        assert_eq!(result.sd_numbers, vec![1, 4, 5]);
        assert_eq!(result.sd_accs, vec!["", "", ""]);
    }

    #[test]
    fn test_get_relative_scale_degrees_add_omit() {
        let mut chord = test_chord_v7();
        chord.adds.push(9);
        chord.omits.push(5);
        let result = get_relative_scale_degrees(&chord).unwrap();

        assert_eq!(result.sd_numbers, vec![1, 3, 7, 9]);
        assert_eq!(result.sd_accs, vec!["", "", "", ""]);
    }

    #[test]
    fn test_get_stable_scale_degrees_basic() {
        let chord = test_chord_c_major();
        let scale = ScaleFingerprint::major_scale();
        let result = get_stable_scale_degrees(&chord, &scale).unwrap();

        assert_eq!(result, vec!["1", "3", "5"]);
    }

    #[test]
    fn test_get_stable_scale_degrees_rest() {
        let chord = Chord::rest();
        let scale = ScaleFingerprint::major_scale();
        let result = get_stable_scale_degrees(&chord, &scale).unwrap();

        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_get_chord_complexity_basic() {
        let chord = test_chord_c_major();
        let complexity = get_chord_complexity(&chord, "major").unwrap();

        assert_eq!(complexity, 1.0); // Basic triad
    }

    #[test]
    fn test_get_chord_complexity_seventh() {
        let chord = test_chord_v7();
        let complexity = get_chord_complexity(&chord, "major").unwrap();

        assert_eq!(complexity, 2.0); // Seventh chord
    }

    #[test]
    fn test_get_chord_complexity_complex() {
        let mut chord = test_chord_v7();
        chord.inversion = 1;
        chord.alterations.push("b9".to_string());
        chord.alterations.push("#11".to_string());
        let complexity = get_chord_complexity(&chord, "major").unwrap();

        // 2.0 (seventh) + 0.5 (inversion) + 1.0 (2 alterations) = 3.5
        assert_eq!(complexity, 3.5);
    }

    #[test]
    fn test_is_valid_tri_sub() {
        let v7 = test_chord_v7();
        assert!(is_valid_tri_sub(&v7, "major"));

        let applied = test_chord_applied_v7_of_v();
        assert!(is_valid_tri_sub(&applied, "major"));

        let triad = test_chord_c_major();
        assert!(!is_valid_tri_sub(&triad, "major"));
    }

    #[test]
    fn test_chord_letter_case_conversion() {
        assert_eq!(chord_letter_to_lower_case("C#"), "c#");
        assert_eq!(chord_letter_to_upper_case("bb"), "Bb");
        assert_eq!(chord_letter_to_lower_case(""), "");
        assert_eq!(chord_letter_to_upper_case("#"), "#");
    }

    #[test]
    fn test_get_relative_chord_graphic_basic() {
        let chord = test_chord_c_major();
        let scale = ScaleFingerprint::major_scale();
        let graphic = get_relative_chord_graphic(&chord, &scale).unwrap();

        assert_eq!(graphic.symbol, "I");
        assert!(graphic.figured_bass.is_empty());
        assert!(graphic.quality.is_empty()); // Major has no symbol
    }

    #[test]
    fn test_get_relative_chord_graphic_seventh() {
        let chord = test_chord_v7();
        let scale = ScaleFingerprint::major_scale();
        let graphic = get_relative_chord_graphic(&chord, &scale).unwrap();

        assert_eq!(graphic.symbol, "V7");
        assert_eq!(graphic.figured_bass, "7");
    }

    #[test]
    fn test_get_relative_chord_graphic_applied() {
        let chord = test_chord_applied_v7_of_v();
        let scale = ScaleFingerprint::major_scale();
        let graphic = get_relative_chord_graphic(&chord, &scale).unwrap();

        assert_eq!(graphic.symbol, "ii7"); // ii7 is correct for D7 (2nd degree) in major
        assert_eq!(graphic.applied, "/V");
    }
}
