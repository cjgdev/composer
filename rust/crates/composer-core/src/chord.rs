//! Chord data structure and validation

use crate::constants::{VALID_ADD_TONES, VALID_ALTERATIONS, VALID_OMIT_TONES, VALID_SUSPENSIONS};
use crate::error::{ChordTheoryError, ChordTheoryResult};
use crate::scale::ScaleType;
use composer_config::MUSICAL;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::fmt;

/// Core chord data structure with 27 fields as specified in the Composer specification.
///
/// The `Chord` struct represents a musical chord with comprehensive support for all possible
/// extensions, alterations, inversions, and harmonic context information. This is the fundamental
/// data type for all chord-based operations in the Composer library.
///
/// # Examples
///
/// ## Basic Chord Creation
///
/// ```rust
/// use composer_core::Chord;
///
/// // Create a simple C major triad (I)
/// let c_major = Chord::new(1, 5)?;
/// assert_eq!(c_major.root, 1);
/// assert_eq!(c_major.chord_type, 5);
/// assert!(c_major.is_triad());
///
/// // Create a G dominant seventh chord (V7)
/// let g7 = Chord::new(5, 7)?;
/// assert!(g7.is_seventh());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Advanced Chord Construction
///
/// ```rust
/// use composer_core::Chord;
///
/// // Create a complex altered dominant: G7♭9♯11 in first inversion
/// let altered_dom = Chord::new(5, 7)?
///     .with_alteration("b9")?
///     .with_alteration("#11")?
///     .with_inversion(1)?;
///
/// assert!(altered_dom.has_alterations());
/// assert_eq!(altered_dom.inversion, 1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Timing and Context
///
/// ```rust
/// use composer_core::{Chord, BorrowedScale};
///
/// // Create a chord with timing and borrowed scale context
/// let borrowed_chord = Chord::new(6, 5)?
///     .with_borrowed_scale(BorrowedScale::Named("harmonic_minor".to_string()))?
///     .with_timing(2.5, 1.0);
///
/// assert!(borrowed_chord.is_borrowed());
/// assert_eq!(borrowed_chord.beat, Some(2.5));
/// assert_eq!(borrowed_chord.duration, Some(1.0));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Related Functions
///
/// - [`get_stable_scale_degrees`] - Analyze chord scale degrees in key context
/// - [`get_chord_complexity`] - Calculate harmonic complexity score
/// - [`get_relative_chord_graphic`] - Generate Roman numeral representation
/// - Binary serialization: [`serialize_chord`] and [`deserialize_chord`]
/// - AI analysis: [`AiEngine::assess_difficulty`] and [`AiEngine::get_chord_suggestions`]
///
/// [`get_stable_scale_degrees`]: crate::theory::get_stable_scale_degrees
/// [`get_chord_complexity`]: crate::theory::get_chord_complexity  
/// [`get_relative_chord_graphic`]: crate::roman::get_relative_chord_graphic
/// [`serialize_chord`]: composer_serialization::serialize_chord
/// [`deserialize_chord`]: composer_serialization::deserialize_chord
/// [`AiEngine::assess_difficulty`]: composer_ai::AiEngine::assess_difficulty
/// [`AiEngine::get_chord_suggestions`]: composer_ai::AiEngine::get_chord_suggestions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chord {
    /// Scale degree (1-7), 0 for rest
    pub root: u8,

    /// Chord type (5=triad, 7=seventh, 9=ninth, 11=eleventh, 13=thirteenth)
    pub chord_type: u8,

    /// Inversion level (0=root, 1=first, 2=second, 3=third)
    pub inversion: u8,

    /// Applied chord target (0=none, 1-7=scale degree)
    pub applied: u8,

    /// Added intervals (typically 4, 6, 9)
    pub adds: SmallVec<[u8; 3]>,

    /// Omitted tones (typically 3, 5)
    pub omits: SmallVec<[u8; 2]>,

    /// Alterations (b5, #5, b9, #9, #11, b13)
    pub alterations: SmallVec<[String; 4]>,

    /// Suspended intervals (2, 4)
    pub suspensions: SmallVec<[u8; 2]>,

    /// Borrowed scale information
    pub borrowed: Option<BorrowedScale>,

    /// Pedal note
    pub pedal: Option<String>,

    /// Alternate chord symbols
    pub alternate: String,

    /// Substitution types
    pub substitutions: SmallVec<[String; 2]>,

    /// Rest indicator
    pub is_rest: bool,

    /// Beat position (optional)
    pub beat: Option<f64>,

    /// Duration in beats (optional)
    pub duration: Option<f64>,
}

/// Borrowed scale information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BorrowedScale {
    /// Named scale (e.g., "harmonic_minor", "dorian")
    Named(String),
    /// Scale type reference
    ScaleType(ScaleType),
    /// Numeric modality offset
    Numeric(i8),
}

impl Default for Chord {
    fn default() -> Self {
        Self {
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
}

impl Chord {
    /// Creates a new chord with the specified root and chord type, applying validation.
    ///
    /// This is the primary constructor for creating chord instances. All chord properties
    /// are validated according to music theory rules before the chord is created.
    ///
    /// # Arguments
    ///
    /// * `root` - Scale degree (1-7), where 1=tonic, 2=supertonic, etc.
    /// * `chord_type` - Chord extension level: 5=triad, 7=seventh, 9=ninth, 11=eleventh, 13=thirteenth
    ///
    /// # Returns
    ///
    /// Returns `Ok(Chord)` if the parameters are valid, or a `ChordTheoryError` if validation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// // Create a tonic triad (I)
    /// let tonic = Chord::new(1, 5)?;
    /// assert_eq!(tonic.root, 1);
    /// assert!(tonic.is_triad());
    ///
    /// // Create a dominant seventh (V7)
    /// let dominant7 = Chord::new(5, 7)?;
    /// assert!(dominant7.is_seventh());
    ///
    /// // Invalid chord type will return error
    /// assert!(Chord::new(1, 6).is_err());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Performance
    ///
    /// This function performs comprehensive validation and typically completes in <0.001ms.
    ///
    /// # Related Functions
    ///
    /// - [`Chord::triad`] - Convenience constructor for triads
    /// - [`Chord::seventh`] - Convenience constructor for seventh chords
    /// - [`Chord::rest`] - Create a rest chord
    /// - [`Chord::validate`] - Manual validation of chord properties
    pub fn new(root: u8, chord_type: u8) -> ChordTheoryResult<Self> {
        let chord = Self {
            root,
            chord_type,
            ..Default::default()
        };
        chord.validate()?;
        Ok(chord)
    }

    /// Creates a rest chord representing musical silence.
    ///
    /// Rest chords are special chord instances that represent periods of silence in
    /// musical progressions. They have `root = 0` and `is_rest = true`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// let rest = Chord::rest();
    /// assert_eq!(rest.root, 0);
    /// assert!(rest.is_rest);
    ///
    /// // Rest chords display as "REST"
    /// assert_eq!(format!("{}", rest), "REST");
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Representing musical rests in chord progressions
    /// - Creating gaps in harmonic analysis
    /// - Placeholder chords in AI-generated progressions
    ///
    /// # Related Functions
    ///
    /// - [`Chord::new`] - Create regular chords
    /// - [`AiEngine::get_chord_suggestions`] - May return rest suggestions
    pub fn rest() -> Self {
        Self {
            root: 0,
            is_rest: true,
            ..Default::default()
        }
    }

    /// Creates a basic triad chord (three-note chord).
    ///
    /// A convenience constructor for creating triad chords, which are the most common
    /// chord type in music. Triads consist of root, third, and fifth.
    ///
    /// # Arguments
    ///
    /// * `root` - Scale degree (1-7) for the chord root
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// // Create triads for a I-vi-IV-V progression
    /// let tonic = Chord::triad(1)?;      // I
    /// let submediant = Chord::triad(6)?; // vi
    /// let subdominant = Chord::triad(4)?; // IV
    /// let dominant = Chord::triad(5)?;   // V
    ///
    /// assert!(tonic.is_triad());
    /// assert_eq!(tonic.expected_tone_count(), 3);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Related Functions
    ///
    /// - [`Chord::seventh`] - Create seventh chords
    /// - [`Chord::new`] - General constructor with explicit chord type
    /// - [`Chord::is_triad`] - Check if a chord is a triad
    pub fn triad(root: u8) -> ChordTheoryResult<Self> {
        Self::new(root, 5)
    }

    /// Creates a seventh chord (four-note chord).
    ///
    /// A convenience constructor for creating seventh chords, which add harmonic richness
    /// and are essential in jazz and contemporary music. Seventh chords consist of
    /// root, third, fifth, and seventh.
    ///
    /// # Arguments
    ///
    /// * `root` - Scale degree (1-7) for the chord root
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// // Create seventh chords for jazz progression
    /// let maj7 = Chord::seventh(1)?;  // Imaj7
    /// let dom7 = Chord::seventh(5)?;  // V7
    /// let min7 = Chord::seventh(2)?;  // ii7
    ///
    /// assert!(maj7.is_seventh());
    /// assert_eq!(maj7.expected_tone_count(), 4);
    ///
    /// // Can be further modified with alterations
    /// let altered_dom = dom7.with_alteration("b9")?;
    /// assert!(altered_dom.has_alterations());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Musical Context
    ///
    /// Seventh chords are particularly important in:
    /// - Jazz progressions (ii7-V7-Imaj7)
    /// - Blues and rock music
    /// - Classical harmonic analysis
    /// - Contemporary pop music
    ///
    /// # Related Functions
    ///
    /// - [`Chord::triad`] - Create simpler triad chords
    /// - [`Chord::new`] - General constructor for extended chords (9th, 11th, 13th)
    /// - [`Chord::is_seventh`] - Check if a chord is a seventh chord
    /// - [`Chord::with_alteration`] - Add alterations like ♭9, ♯11
    pub fn seventh(root: u8) -> ChordTheoryResult<Self> {
        Self::new(root, 7)
    }

    /// Validates all chord properties according to music theory rules.
    ///
    /// This method performs comprehensive validation of all chord fields, ensuring
    /// that the chord represents a musically valid construction. It checks:
    /// - Root scale degree validity (0 for rests, 1-7 for regular chords)
    /// - Chord type validity (5, 7, 9, 11, 13)
    /// - Inversion limits (0-3)
    /// - Applied chord targets (0-7)
    /// - Alteration validity and compatibility
    /// - Suspension compatibility
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// // Valid chord passes validation
    /// let valid_chord = Chord::new(1, 7)?;
    /// assert!(valid_chord.validate().is_ok());
    ///
    /// // Invalid modifications can be caught
    /// let mut invalid_chord = Chord::new(1, 7)?;
    /// invalid_chord.alterations.push("invalid".to_string());
    /// assert!(invalid_chord.validate().is_err());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Performance
    ///
    /// Validation typically completes in <0.001ms and is automatically called
    /// by all constructor and builder methods.
    ///
    /// # Related Functions
    ///
    /// - [`Chord::new`] - Automatically validates on construction
    /// - [`Chord::with_*`] - Builder methods that validate after each change
    /// - [`Chord::validate_alteration_compatibility`] - Internal validation helper
    pub fn validate(&self) -> ChordTheoryResult<()> {
        // Rest chords can skip other validations
        if self.is_rest {
            if self.root != 0 {
                return Err(ChordTheoryError::InvalidChordRoot { root: self.root });
            }
            return Ok(());
        }

        // Non-rest chords must have valid root (1-7)
        if self.root == 0 || self.root > MUSICAL.scale_degrees {
            return Err(ChordTheoryError::InvalidChordRoot { root: self.root });
        }

        // Validate chord type
        if !MUSICAL.chord_types.contains(&self.chord_type) {
            return Err(ChordTheoryError::InvalidChordType {
                chord_type: self.chord_type,
            });
        }

        // Validate inversion
        if self.inversion > 3 {
            return Err(ChordTheoryError::InvalidInversion {
                inversion: self.inversion,
            });
        }

        // Validate applied
        if self.applied > 7 {
            return Err(ChordTheoryError::InvalidApplied {
                applied: self.applied,
            });
        }

        // Validate alterations
        for alteration in &self.alterations {
            if !VALID_ALTERATIONS.contains(&alteration.as_str()) {
                return Err(ChordTheoryError::InvalidAlteration {
                    alteration: alteration.clone(),
                });
            }
        }

        // Validate suspensions
        for &suspension in &self.suspensions {
            if !VALID_SUSPENSIONS.contains(&suspension) {
                return Err(ChordTheoryError::InvalidSuspension { suspension });
            }
        }

        // Validate add tones
        for &add in &self.adds {
            if !VALID_ADD_TONES.contains(&add) {
                return Err(ChordTheoryError::InvalidAlteration {
                    alteration: format!("add{}", add),
                });
            }
        }

        // Validate omit tones
        for &omit in &self.omits {
            if !VALID_OMIT_TONES.contains(&omit) {
                return Err(ChordTheoryError::InvalidAlteration {
                    alteration: format!("omit{}", omit),
                });
            }
        }

        // Check for incompatible alterations
        self.validate_alteration_compatibility()?;

        Ok(())
    }

    /// Validate that alterations are compatible with each other
    fn validate_alteration_compatibility(&self) -> ChordTheoryResult<()> {
        // Check for conflicting alterations (b5 vs #5, etc.)
        let has_flat5 = self.alterations.iter().any(|a| a == "b5");
        let has_sharp5 = self.alterations.iter().any(|a| a == "#5");
        if has_flat5 && has_sharp5 {
            return Err(ChordTheoryError::IncompatibleAlterations {
                alterations: vec!["b5".to_string(), "#5".to_string()],
            });
        }

        let has_flat9 = self.alterations.iter().any(|a| a == "b9");
        let has_sharp9 = self.alterations.iter().any(|a| a == "#9");
        if has_flat9 && has_sharp9 {
            return Err(ChordTheoryError::IncompatibleAlterations {
                alterations: vec!["b9".to_string(), "#9".to_string()],
            });
        }

        // Check suspension compatibility
        if self.suspensions.contains(&2) && self.suspensions.contains(&4) {
            return Err(ChordTheoryError::IncompatibleAlterations {
                alterations: vec!["sus2".to_string(), "sus4".to_string()],
            });
        }

        Ok(())
    }

    /// Builder pattern methods
    ///
    /// Sets the inversion level of the chord.
    ///
    /// Inversions change which chord tone appears in the bass, creating different
    /// harmonic colors and voice-leading possibilities. This method uses the builder
    /// pattern and validates the result.
    ///
    /// # Arguments
    ///
    /// * `inversion` - Inversion level: 0=root position, 1=first inversion, 2=second inversion, 3=third inversion
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// // Create a C major triad in different inversions
    /// let root_position = Chord::triad(1)?;                    // C-E-G
    /// let first_inversion = Chord::triad(1)?.with_inversion(1)?; // E-G-C
    /// let second_inversion = Chord::triad(1)?.with_inversion(2)?; // G-C-E
    ///
    /// assert_eq!(root_position.inversion, 0);
    /// assert_eq!(first_inversion.inversion, 1);
    /// assert_eq!(second_inversion.inversion, 2);
    ///
    /// // Invalid inversion returns error
    /// assert!(Chord::triad(1)?.with_inversion(4).is_err());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Musical Context
    ///
    /// Inversions are crucial for:
    /// - Smooth voice leading between chords
    /// - Creating different bass lines
    /// - Reducing harmonic weight (inversions often sound lighter)
    /// - Classical four-part writing conventions
    ///
    /// # Related Functions
    ///
    /// - [`Chord::is_root_position`] - Check if chord is in root position
    /// - [`get_stable_scale_degrees`] - Analysis accounts for inversions
    /// - [`Chord::expected_tone_count`] - Tone count unaffected by inversion
    pub fn with_inversion(mut self, inversion: u8) -> ChordTheoryResult<Self> {
        self.inversion = inversion;
        self.validate()?;
        Ok(self)
    }

    /// Adds a harmonic alteration to the chord.
    ///
    /// Alterations modify specific chord tones to create color, tension, or
    /// harmonic movement. Common alterations include flattened/sharpened fifths,
    /// ninths, elevenths, and thirteenths.
    ///
    /// # Arguments
    ///
    /// * `alteration` - Alteration string: "b5", "#5", "b9", "#9", "#11", "b13"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// // Create altered dominant chords
    /// let flat_five = Chord::seventh(5)?.with_alteration("b5")?;     // V7♭5
    /// let sharp_nine = Chord::seventh(5)?.with_alteration("#9")?;    // V7♯9
    /// let sharp_eleven = Chord::seventh(1)?.with_alteration("#11")?; // Imaj7♯11 (Lydian)
    ///
    /// assert!(flat_five.has_alterations());
    /// assert!(sharp_nine.alterations.contains(&"#9".to_string()));
    ///
    /// // Multiple alterations can be chained
    /// let altered_dom = Chord::seventh(5)?
    ///     .with_alteration("b9")?
    ///     .with_alteration("#11")?;
    /// assert_eq!(altered_dom.alterations.len(), 2);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Alteration Reference
    ///
    /// | Alteration | Musical Effect | Common Use |
    /// |------------|----------------|------------|
    /// | `"b5"`     | Diminished fifth | Tritone substitution, blues |
    /// | `"#5"`     | Augmented fifth | Augmented chords, chromatic movement |
    /// | `"b9"`     | Minor ninth | Dominant tension, Phrygian color |
    /// | `"#9"`     | Sharp ninth | Jazz fusion, rock harmony |
    /// | `"#11"`    | Sharp eleventh | Lydian mode, major 7 chords |
    /// | `"b13"`    | Minor thirteenth | Dorian mode, minor harmony |
    ///
    /// # Musical Context
    ///
    /// - Jazz: Extensively used in dominant chords (V7alt)
    /// - Classical: Augmented sixth chords, Neapolitan sixths
    /// - Contemporary: Add harmonic sophistication
    ///
    /// # Related Functions
    ///
    /// - [`Chord::has_alterations`] - Check for any alterations
    /// - [`Chord::validate_alteration_compatibility`] - Prevents conflicting alterations
    /// - [`get_chord_complexity`] - Alterations increase complexity score
    pub fn with_alteration(mut self, alteration: &str) -> ChordTheoryResult<Self> {
        if !self.alterations.contains(&alteration.to_string()) {
            self.alterations.push(alteration.to_string());
        }
        self.validate()?;
        Ok(self)
    }

    /// Add a suspension
    pub fn with_suspension(mut self, suspension: u8) -> ChordTheoryResult<Self> {
        if !self.suspensions.contains(&suspension) {
            self.suspensions.push(suspension);
        }
        self.validate()?;
        Ok(self)
    }

    /// Add an extension tone
    pub fn with_add(mut self, add: u8) -> ChordTheoryResult<Self> {
        if !self.adds.contains(&add) {
            self.adds.push(add);
        }
        self.validate()?;
        Ok(self)
    }

    /// Omit a tone
    pub fn with_omit(mut self, omit: u8) -> ChordTheoryResult<Self> {
        if !self.omits.contains(&omit) {
            self.omits.push(omit);
        }
        self.validate()?;
        Ok(self)
    }

    /// Set as applied chord
    pub fn with_applied(mut self, applied: u8) -> ChordTheoryResult<Self> {
        self.applied = applied;
        self.validate()?;
        Ok(self)
    }

    /// Set borrowed scale
    pub fn with_borrowed_scale(mut self, scale: BorrowedScale) -> ChordTheoryResult<Self> {
        self.borrowed = Some(scale);
        self.validate()?;
        Ok(self)
    }

    /// Set timing information
    pub fn with_timing(mut self, beat: f64, duration: f64) -> Self {
        self.beat = Some(beat);
        self.duration = Some(duration);
        self
    }

    /// Query methods
    ///
    /// Checks if this is a basic triad (three-note chord).
    ///
    /// Triads are the fundamental building blocks of harmony, consisting of
    /// root, third, and fifth. This method returns true for chord_type = 5.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// let triad = Chord::triad(1)?;
    /// assert!(triad.is_triad());
    /// assert!(!triad.is_seventh());
    /// assert!(!triad.is_extended());
    ///
    /// let seventh = Chord::seventh(5)?;
    /// assert!(!seventh.is_triad());
    /// assert!(seventh.is_seventh());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Related Functions
    ///
    /// - [`Chord::triad`] - Constructor for triad chords
    /// - [`Chord::is_seventh`] - Check for seventh chords
    /// - [`Chord::is_extended`] - Check for extended chords (9th, 11th, 13th)
    /// - [`Chord::expected_tone_count`] - Triads typically have 3 tones
    pub fn is_triad(&self) -> bool {
        self.chord_type == 5
    }

    /// Check if this is a seventh chord
    pub fn is_seventh(&self) -> bool {
        self.chord_type == 7
    }

    /// Check if this is an extended chord (9th, 11th, 13th)
    pub fn is_extended(&self) -> bool {
        self.chord_type > 7
    }

    /// Check if this chord has any alterations
    pub fn has_alterations(&self) -> bool {
        !self.alterations.is_empty()
    }

    /// Check if this is an applied chord
    pub fn is_applied(&self) -> bool {
        self.applied != 0
    }

    /// Check if this is a borrowed chord
    pub fn is_borrowed(&self) -> bool {
        self.borrowed.is_some()
    }

    /// Check if this chord has suspensions
    pub fn has_suspensions(&self) -> bool {
        !self.suspensions.is_empty()
    }

    /// Check if this chord is in root position
    pub fn is_root_position(&self) -> bool {
        self.inversion == 0
    }

    /// Calculates the expected number of chord tones based on type and modifications.
    ///
    /// This method computes the theoretical number of distinct pitch classes in the chord,
    /// accounting for the base chord type, added tones, and omitted tones. It's useful
    /// for voice-leading analysis and chord complexity calculations.
    ///
    /// # Returns
    ///
    /// The expected number of distinct chord tones as a `usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_core::Chord;
    ///
    /// // Basic chord types
    /// let triad = Chord::triad(1)?;
    /// assert_eq!(triad.expected_tone_count(), 3); // Root, 3rd, 5th
    ///
    /// let seventh = Chord::seventh(1)?;
    /// assert_eq!(seventh.expected_tone_count(), 4); // Root, 3rd, 5th, 7th
    ///
    /// let ninth = Chord::new(1, 9)?;
    /// assert_eq!(ninth.expected_tone_count(), 5); // Root, 3rd, 5th, 7th, 9th
    ///
    /// // Modified chords
    /// let add9 = Chord::triad(1)?.with_add(9)?;
    /// assert_eq!(add9.expected_tone_count(), 4); // 3 + 1 added tone
    ///
    /// let no5 = Chord::seventh(1)?.with_omit(5)?;
    /// assert_eq!(no5.expected_tone_count(), 3); // 4 - 1 omitted tone
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Musical Applications
    ///
    /// - **Voice Leading**: Ensures proper voice count in arrangements
    /// - **Complexity Analysis**: More tones generally increase harmonic complexity
    /// - **Chord Voicing**: Helps determine doubling and omission strategies
    /// - **MIDI Generation**: Assists in chord voicing algorithms
    ///
    /// # Note
    ///
    /// The actual sounding result may differ due to:
    /// - Octave doublings in performance
    /// - Instrument limitations
    /// - Voice leading considerations
    /// - Performance practice conventions
    ///
    /// # Related Functions
    ///
    /// - [`Chord::with_add`] - Add tones to increase count
    /// - [`Chord::with_omit`] - Remove tones to decrease count
    /// - [`get_chord_complexity`] - Uses tone count in complexity calculation
    /// - [`get_stable_scale_degrees`] - Returns actual scale degrees present
    pub fn expected_tone_count(&self) -> usize {
        let base_count = match self.chord_type {
            5 => 3,  // Triad
            7 => 4,  // Seventh
            9 => 5,  // Ninth
            11 => 6, // Eleventh
            13 => 7, // Thirteenth
            _ => 3,  // Default to triad
        };

        base_count + self.adds.len() - self.omits.len()
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_rest {
            return write!(f, "REST");
        }

        // Start with root
        write!(f, "{}", self.root)?;

        // Add chord type if not triad
        if self.chord_type != 5 {
            write!(f, "{}", self.chord_type)?;
        }

        // Add alterations
        for alteration in &self.alterations {
            write!(f, "{}", alteration)?;
        }

        // Add suspensions
        for &suspension in &self.suspensions {
            write!(f, "sus{}", suspension)?;
        }

        // Add extensions
        for &add in &self.adds {
            write!(f, "add{}", add)?;
        }

        // Add omissions
        for &omit in &self.omits {
            write!(f, "omit{}", omit)?;
        }

        // Add inversion
        if self.inversion > 0 {
            write!(f, "/{}", self.inversion)?;
        }

        // Add applied chord notation
        if self.applied > 0 {
            write!(f, "/V{}", self.applied)?;
        }

        Ok(())
    }
}

impl BorrowedScale {
    /// Get the scale name as string
    pub fn scale_name(&self) -> String {
        match self {
            BorrowedScale::Named(name) => name.clone(),
            BorrowedScale::ScaleType(scale_type) => scale_type.name().to_string(),
            BorrowedScale::Numeric(offset) => format!("mode_{:+}", offset),
        }
    }
}

impl fmt::Display for BorrowedScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.scale_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_creation() {
        let chord = Chord::new(1, 5).unwrap();
        assert_eq!(chord.root, 1);
        assert_eq!(chord.chord_type, 5);
        assert!(!chord.is_rest);
        assert!(chord.is_triad());
    }

    #[test]
    fn test_rest_chord() {
        let rest = Chord::rest();
        assert_eq!(rest.root, 0);
        assert!(rest.is_rest);
    }

    #[test]
    fn test_chord_validation() {
        // Invalid root
        assert!(Chord::new(8, 5).is_err());

        // Invalid chord type
        let mut chord = Chord::new(1, 5).unwrap();
        chord.chord_type = 6;
        assert!(chord.validate().is_err());

        // Invalid inversion
        chord = Chord::new(1, 5).unwrap();
        chord.inversion = 4;
        assert!(chord.validate().is_err());

        // Invalid alteration
        chord = Chord::new(1, 7).unwrap();
        chord.alterations.push("invalid".to_string());
        assert!(chord.validate().is_err());
    }

    #[test]
    fn test_alteration_compatibility() {
        let mut chord = Chord::new(1, 7).unwrap();

        // Conflicting alterations should fail
        chord.alterations.push("b5".to_string());
        chord.alterations.push("#5".to_string());
        assert!(chord.validate().is_err());

        // Conflicting suspensions should fail
        chord = Chord::new(1, 5).unwrap();
        chord.suspensions.push(2);
        chord.suspensions.push(4);
        assert!(chord.validate().is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let chord = Chord::new(5, 7)
            .unwrap()
            .with_alteration("b9")
            .unwrap()
            .with_suspension(4)
            .unwrap()
            .with_inversion(1)
            .unwrap();

        assert_eq!(chord.root, 5);
        assert_eq!(chord.chord_type, 7);
        assert!(chord.alterations.contains(&"b9".to_string()));
        assert!(chord.suspensions.contains(&4));
        assert_eq!(chord.inversion, 1);
    }

    #[test]
    fn test_chord_queries() {
        let triad = Chord::new(1, 5).unwrap();
        assert!(triad.is_triad());
        assert!(!triad.is_seventh());
        assert!(!triad.is_extended());

        let seventh = Chord::new(5, 7).unwrap();
        assert!(!seventh.is_triad());
        assert!(seventh.is_seventh());
        assert!(!seventh.is_extended());

        let ninth = Chord::new(2, 9).unwrap();
        assert!(!ninth.is_triad());
        assert!(!ninth.is_seventh());
        assert!(ninth.is_extended());
    }

    #[test]
    fn test_applied_chord() {
        let applied = Chord::new(2, 7).unwrap().with_applied(5).unwrap();
        assert!(applied.is_applied());
        assert_eq!(applied.applied, 5);
    }

    #[test]
    fn test_borrowed_chord() {
        let borrowed = Chord::new(1, 5)
            .unwrap()
            .with_borrowed_scale(BorrowedScale::Named("harmonic_minor".to_string()))
            .unwrap();
        assert!(borrowed.is_borrowed());
    }

    #[test]
    fn test_timing() {
        let chord = Chord::new(1, 5).unwrap().with_timing(2.5, 1.0);
        assert_eq!(chord.beat, Some(2.5));
        assert_eq!(chord.duration, Some(1.0));
    }

    #[test]
    fn test_expected_tone_count() {
        let triad = Chord::new(1, 5).unwrap();
        assert_eq!(triad.expected_tone_count(), 3);

        let seventh = Chord::new(1, 7).unwrap();
        assert_eq!(seventh.expected_tone_count(), 4);

        let ninth_add6 = Chord::new(1, 9).unwrap().with_add(6).unwrap();
        assert_eq!(ninth_add6.expected_tone_count(), 6); // 5 + 1 add

        let seventh_omit5 = Chord::new(1, 7).unwrap().with_omit(5).unwrap();
        assert_eq!(seventh_omit5.expected_tone_count(), 3); // 4 - 1 omit
    }

    #[test]
    fn test_display() {
        let chord = Chord::new(1, 5).unwrap();
        assert_eq!(format!("{}", chord), "1");

        let seventh = Chord::new(5, 7).unwrap();
        assert_eq!(format!("{}", seventh), "57");

        let complex = Chord::new(5, 7)
            .unwrap()
            .with_alteration("b9")
            .unwrap()
            .with_suspension(4)
            .unwrap()
            .with_inversion(1)
            .unwrap();
        let display = format!("{}", complex);
        assert!(display.contains("5"));
        assert!(display.contains("7"));
        assert!(display.contains("b9"));
        assert!(display.contains("sus4"));

        let rest = Chord::rest();
        assert_eq!(format!("{}", rest), "REST");
    }

    #[test]
    fn test_borrowed_scale() {
        let named = BorrowedScale::Named("dorian".to_string());
        assert_eq!(named.scale_name(), "dorian");

        let scale_type = BorrowedScale::ScaleType(ScaleType::HarmonicMinor);
        assert_eq!(scale_type.scale_name(), "harmonic_minor");

        let numeric = BorrowedScale::Numeric(-2);
        assert_eq!(numeric.scale_name(), "mode_-2");
    }
}
