//! WebAssembly bindings for Composer music theory library
//!
//! This crate provides WASM bindings that allow the Composer library to be used
//! from JavaScript in web browsers and Node.js environments.

use composer_ai::{AiEngine, AiEngineConfig, ChordProgressionTrie, ChordSuggestion};
use composer_core::{
    get_chord_complexity, get_relative_chord_graphic, get_stable_scale_degrees, Chord,
    ScaleFingerprint,
};
use composer_serialization::{
    chord_binary_to_hex, deserialize_chord, hex_to_chord_binary, parse_duration_token,
    serialize_chord, tokenize_duration, Note, Timeline, TokenEvent, TokenEventType,
};
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

// Set up console error reporting
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// WASM wrapper for Chord with JavaScript-friendly methods
#[wasm_bindgen]
pub struct WasmChord {
    inner: Chord,
}

#[wasm_bindgen]
impl WasmChord {
    /// Creates a new chord with specified root and chord type.
    ///
    /// This is the primary constructor for creating chord instances in JavaScript/WebAssembly.
    /// All chord properties are validated according to music theory rules before creation.
    ///
    /// @param {number} root - Scale degree (1-7), where 1=tonic, 2=supertonic, etc.
    /// @param {number} chordType - Chord extension level: 5=triad, 7=seventh, 9=ninth, 11=eleventh, 13=thirteenth
    /// @returns {WasmChord} A new chord instance with the specified properties
    /// @throws {Error} If any parameters are musically invalid
    ///
    /// @example
    /// ```javascript
    /// // Create a simple C major triad (I)
    /// const tonic = new WasmChord(1, 5);
    /// console.log(`Root: ${tonic.root}, Type: ${tonic.chordType}`);
    /// // Output: Root: 1, Type: 5
    ///
    /// // Create a dominant seventh chord (V7)
    /// const dominantSeventh = new WasmChord(5, 7);
    /// console.log(`Is seventh: ${dominantSeventh.isSeventh()}`);
    /// // Output: Is seventh: true
    ///
    /// // Invalid chord type will throw error
    /// try {
    ///   const invalid = new WasmChord(1, 6);
    /// } catch (error) {
    ///   console.log("Invalid chord type");
    /// }
    /// ```
    ///
    /// @example
    /// ```javascript
    /// // Create chords for a I-vi-IV-V progression
    /// const progression = [
    ///   new WasmChord(1, 5), // I
    ///   new WasmChord(6, 5), // vi
    ///   new WasmChord(4, 5), // IV
    ///   new WasmChord(5, 5)  // V
    /// ];
    ///
    /// progression.forEach((chord, i) => {
    ///   console.log(`Chord ${i + 1}: ${chord.toString()}`);
    /// });
    /// ```
    ///
    /// ## Performance
    /// Chord creation typically completes in <0.001ms with full validation.
    ///
    /// ## Related Functions
    /// - `WasmChord.triad(root)` - Convenience constructor for triads
    /// - `WasmChord.seventh(root)` - Convenience constructor for seventh chords
    /// - `getStableScaleDegrees()` - Analyze chord in key context
    /// - `getChordComplexity()` - Calculate harmonic complexity
    #[wasm_bindgen(constructor)]
    pub fn new(root: u8, chord_type: u8) -> Result<WasmChord, JsValue> {
        let chord = Chord::new(root, chord_type).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmChord { inner: chord })
    }

    /// Create a rest chord
    #[wasm_bindgen(js_name = "rest")]
    pub fn rest() -> WasmChord {
        WasmChord {
            inner: Chord::rest(),
        }
    }

    /// Create a triad
    #[wasm_bindgen(js_name = "triad")]
    pub fn triad(root: u8) -> Result<WasmChord, JsValue> {
        let chord = Chord::triad(root).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmChord { inner: chord })
    }

    /// Create a seventh chord
    #[wasm_bindgen(js_name = "seventh")]
    pub fn seventh(root: u8) -> Result<WasmChord, JsValue> {
        let chord = Chord::seventh(root).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmChord { inner: chord })
    }

    /// Get the root scale degree
    #[wasm_bindgen(getter)]
    pub fn root(&self) -> u8 {
        self.inner.root
    }

    /// Get the chord type
    #[wasm_bindgen(getter, js_name = "chordType")]
    pub fn chord_type(&self) -> u8 {
        self.inner.chord_type
    }

    /// Get the inversion level
    #[wasm_bindgen(getter)]
    pub fn inversion(&self) -> u8 {
        self.inner.inversion
    }

    /// Check if this is a rest chord
    #[wasm_bindgen(getter, js_name = "isRest")]
    pub fn is_rest(&self) -> bool {
        self.inner.is_rest
    }

    /// Check if this is a triad
    #[wasm_bindgen(js_name = "isTriad")]
    pub fn is_triad(&self) -> bool {
        self.inner.is_triad()
    }

    /// Check if this is a seventh chord
    #[wasm_bindgen(js_name = "isSeventh")]
    pub fn is_seventh(&self) -> bool {
        self.inner.is_seventh()
    }

    /// Check if this is an extended chord
    #[wasm_bindgen(js_name = "isExtended")]
    pub fn is_extended(&self) -> bool {
        self.inner.is_extended()
    }

    /// Add an alteration to the chord
    #[wasm_bindgen(js_name = "withAlteration")]
    pub fn with_alteration(&self, alteration: &str) -> Result<WasmChord, JsValue> {
        let chord = self
            .inner
            .clone()
            .with_alteration(alteration)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmChord { inner: chord })
    }

    /// Set inversion level
    #[wasm_bindgen(js_name = "withInversion")]
    pub fn with_inversion(&self, inversion: u8) -> Result<WasmChord, JsValue> {
        let chord = self
            .inner
            .clone()
            .with_inversion(inversion)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmChord { inner: chord })
    }

    /// Convert to string representation
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        format!("{}", self.inner)
    }

    /// Serialize to hex string
    #[wasm_bindgen(js_name = "toHex")]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        let binary = serialize_chord(&self.inner).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(chord_binary_to_hex(&binary))
    }

    /// Deserialize from hex string
    #[wasm_bindgen(js_name = "fromHex")]
    pub fn from_hex(hex: &str) -> Result<WasmChord, JsValue> {
        let binary = hex_to_chord_binary(hex).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let chord = deserialize_chord(&binary).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmChord { inner: chord })
    }
}

/// WASM wrapper for Note
#[wasm_bindgen]
pub struct WasmNote {
    inner: Note,
}

#[wasm_bindgen]
impl WasmNote {
    /// Create a new note
    #[wasm_bindgen(constructor)]
    pub fn new(scale_degree: u8, octave: u8) -> WasmNote {
        WasmNote {
            inner: Note {
                scale_degree,
                octave,
                is_rest: false,
            },
        }
    }

    /// Create a rest note
    #[wasm_bindgen(js_name = "rest")]
    pub fn rest() -> WasmNote {
        WasmNote {
            inner: Note {
                scale_degree: 0,
                octave: 0,
                is_rest: true,
            },
        }
    }

    /// Get scale degree
    #[wasm_bindgen(getter, js_name = "scaleDegree")]
    pub fn scale_degree(&self) -> u8 {
        self.inner.scale_degree
    }

    /// Get octave
    #[wasm_bindgen(getter)]
    pub fn octave(&self) -> u8 {
        self.inner.octave
    }

    /// Check if this is a rest
    #[wasm_bindgen(getter, js_name = "isRest")]
    pub fn is_rest(&self) -> bool {
        self.inner.is_rest
    }

    /// Convert to string representation
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        if self.inner.is_rest {
            "REST".to_string()
        } else {
            format!("Note({}:{})", self.inner.scale_degree, self.inner.octave)
        }
    }
}

/// WASM wrapper for Timeline
#[wasm_bindgen]
pub struct WasmTimeline {
    inner: Timeline,
}

#[wasm_bindgen]
impl WasmTimeline {
    /// Create new empty timeline
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTimeline {
        WasmTimeline {
            inner: Timeline {
                events: Vec::new(),
                total_duration: 0.0,
            },
        }
    }

    /// Get total duration
    #[wasm_bindgen(getter, js_name = "totalDuration")]
    pub fn total_duration(&self) -> f64 {
        self.inner.total_duration
    }

    /// Get number of events
    #[wasm_bindgen(getter, js_name = "eventCount")]
    pub fn event_count(&self) -> usize {
        self.inner.events.len()
    }

    /// Add a chord event
    #[wasm_bindgen(js_name = "addChordEvent")]
    pub fn add_chord_event(&mut self, chord: &WasmChord, beat: f64) {
        self.inner.events.push(TokenEvent {
            beat,
            event_type: TokenEventType::Chord(chord.inner.clone()),
        });
        self.inner.total_duration = self.inner.total_duration.max(beat);
    }

    /// Add a note event
    #[wasm_bindgen(js_name = "addNoteEvent")]
    pub fn add_note_event(&mut self, note: &WasmNote, beat: f64) {
        self.inner.events.push(TokenEvent {
            beat,
            event_type: TokenEventType::Note(note.inner.clone()),
        });
        self.inner.total_duration = self.inner.total_duration.max(beat);
    }

    /// Add a rest event
    #[wasm_bindgen(js_name = "addRestEvent")]
    pub fn add_rest_event(&mut self, beat: f64) {
        self.inner.events.push(TokenEvent {
            beat,
            event_type: TokenEventType::Rest,
        });
        self.inner.total_duration = self.inner.total_duration.max(beat);
    }
}

/// WASM wrapper for ScaleFingerprint
#[wasm_bindgen]
pub struct WasmScaleFingerprint {
    inner: ScaleFingerprint,
}

#[wasm_bindgen]
impl WasmScaleFingerprint {
    /// Create a major scale
    #[wasm_bindgen(js_name = "major")]
    pub fn major() -> WasmScaleFingerprint {
        WasmScaleFingerprint {
            inner: ScaleFingerprint::major_scale(),
        }
    }

    /// Create a minor scale
    #[wasm_bindgen(js_name = "minor")]
    pub fn minor() -> WasmScaleFingerprint {
        WasmScaleFingerprint {
            inner: ScaleFingerprint::minor_scale(),
        }
    }

    /// Create a harmonic minor scale
    #[wasm_bindgen(js_name = "harmonicMinor")]
    pub fn harmonic_minor() -> WasmScaleFingerprint {
        WasmScaleFingerprint {
            inner: ScaleFingerprint::harmonic_minor_scale(),
        }
    }

    /// Create from array
    #[wasm_bindgen(js_name = "fromArray")]
    pub fn from_array(semitones: &[u8]) -> Result<WasmScaleFingerprint, JsValue> {
        let fingerprint = ScaleFingerprint::from_slice(semitones)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmScaleFingerprint { inner: fingerprint })
    }

    /// Get the note count
    #[wasm_bindgen(js_name = "noteCount")]
    pub fn note_count(&self) -> usize {
        self.inner.note_count()
    }

    /// Check if diatonic
    #[wasm_bindgen(js_name = "isDiatonic")]
    pub fn is_diatonic(&self) -> bool {
        self.inner.is_diatonic()
    }

    /// Convert to string representation
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        format!("{}", self.inner)
    }
}

/// Calculates the stable scale degrees that a chord occupies within a scale context.
///
/// This is the primary algorithm for harmonic analysis in Composer. It determines which
/// absolute scale degrees (1, 2, 3, 4, 5, 6, 7) a chord occupies when analyzed within
/// a specific scale. The function accounts for chord inversions, alterations, extensions,
/// and all other chord modifications.
///
/// @param {WasmChord} chord - The chord to analyze
/// @param {WasmScaleFingerprint} scale - The scale context for analysis (major, minor, modes, etc.)
/// @returns {string[]} Array of scale degree strings (e.g., ["1", "3", "5"] for a tonic triad)
/// @throws {Error} If chord or scale data is invalid
///
/// @example
/// ```javascript
/// // Analyze basic triads in major scale
/// const majorScale = WasmScaleFingerprint.major();
///
/// // Tonic triad (I): contains scale degrees 1, 3, 5
/// const tonic = WasmChord.triad(1);
/// const degrees = getStableScaleDegrees(tonic, majorScale);
/// console.log(degrees); // ["1", "3", "5"]
///
/// // Subdominant triad (IV): contains scale degrees 4, 6, 1
/// const subdominant = WasmChord.triad(4);
/// const subdDegrees = getStableScaleDegrees(subdominant, majorScale);
/// console.log(subdDegrees); // ["4", "6", "1"]
/// ```
///
/// @example
/// ```javascript
/// // Analyze extended and altered chords
/// const majorScale = WasmScaleFingerprint.major();
///
/// // Dominant seventh with alterations (V7♭9)
/// const alteredDom = WasmChord.seventh(5).withAlteration("b9");
/// const degrees = getStableScaleDegrees(alteredDom, majorScale);
/// console.log(degrees); // ["5", "7", "2", "4", "b6"]
/// // Contains: 5 (root), 7 (third), 2 (fifth), 4 (seventh), b6 (♭9)
/// ```
///
/// @example
/// ```javascript
/// // Compare chord inversions (same scale degrees, different order)
/// const majorScale = WasmScaleFingerprint.major();
///
/// const rootPosition = WasmChord.triad(1);
/// const firstInversion = WasmChord.triad(1).withInversion(1);
///
/// const rootDegrees = getStableScaleDegrees(rootPosition, majorScale);
/// const invDegrees = getStableScaleDegrees(firstInversion, majorScale);
///
/// // Same scale degrees, potentially different ordering
/// console.log("Root position:", rootDegrees);
/// console.log("First inversion:", invDegrees);
/// console.log("Same content:",
///   rootDegrees.every(d => invDegrees.includes(d)) &&
///   invDegrees.every(d => rootDegrees.includes(d))
/// );
/// ```
///
/// ## Performance
/// This function typically completes in <1ms and meets the CHORD_LOOKUP_MAX_MS = 1 requirement.
/// The algorithm is optimized for real-time harmonic analysis.
///
/// ## Musical Theory Background
/// Scale degrees represent the functional relationships between chord tones and the tonic.
/// This analysis is fundamental to:
/// - Roman numeral analysis
/// - Functional harmony understanding  
/// - Voice leading analysis
/// - Jazz chord-scale relationships
/// - Modal harmonic analysis
///
/// ## Use Cases
/// - Music theory education and analysis tools
/// - Composition software harmonic feedback
/// - Improvisation apps (chord-scale suggestions)
/// - Automatic chord progression generation
/// - Voice leading analysis in DAWs
///
/// ## Related Functions
/// - `getChordComplexity()` - Uses scale degree analysis for complexity scoring
/// - `getRomanNumeral()` - Generates Roman numeral representations
/// - `isIsotonal()` - Compares chords by their scale degree content
/// - `WasmAiEngine.getChordSuggestions()` - Uses this for harmonic context analysis
#[wasm_bindgen(js_name = "getStableScaleDegrees")]
pub fn get_stable_scale_degrees_wasm(
    chord: &WasmChord,
    scale: &WasmScaleFingerprint,
) -> Result<Vec<String>, JsValue> {
    get_stable_scale_degrees(&chord.inner, &scale.inner)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Calculate chord complexity
#[wasm_bindgen(js_name = "getChordComplexity")]
pub fn get_chord_complexity_wasm(chord: &WasmChord, scale_name: &str) -> Result<f64, JsValue> {
    get_chord_complexity(&chord.inner, scale_name).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get Roman numeral representation
#[wasm_bindgen(js_name = "getRomanNumeral")]
pub fn get_roman_numeral_wasm(
    chord: &WasmChord,
    scale: &WasmScaleFingerprint,
) -> Result<String, JsValue> {
    let graphic = get_relative_chord_graphic(&chord.inner, &scale.inner)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(graphic.full_symbol())
}

/// Get library version
#[wasm_bindgen(js_name = "getVersion")]
pub fn get_version() -> String {
    composer_config::APPLICATION.version.to_string()
}

/// Tokenize duration for ML applications
#[wasm_bindgen(js_name = "tokenizeDuration")]
pub fn tokenize_duration_wasm(duration: f64) -> String {
    tokenize_duration(duration)
}

/// Parse duration token
#[wasm_bindgen(js_name = "parseDurationToken")]
pub fn parse_duration_token_wasm(token: &str) -> Result<f64, JsValue> {
    parse_duration_token(token).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// WASM wrapper for AiEngine
#[wasm_bindgen]
pub struct WasmAiEngine {
    inner: AiEngine,
}

#[wasm_bindgen]
impl WasmAiEngine {
    /// Create a new AI engine with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmAiEngine {
        let config = AiEngineConfig::default();
        WasmAiEngine {
            inner: AiEngine::new(config),
        }
    }

    /// Create AI engine with custom memory limit
    #[wasm_bindgen(js_name = "withMemoryLimit")]
    pub fn with_memory_limit(max_memory_mb: u32) -> WasmAiEngine {
        let mut config = AiEngineConfig::default();
        config.max_memory_mb = max_memory_mb;
        WasmAiEngine {
            inner: AiEngine::new(config),
        }
    }

    /// Initialize the engine with training patterns
    #[wasm_bindgen(js_name = "initialize")]
    pub fn initialize(&self, training_patterns: Vec<JsValue>) -> Result<(), JsValue> {
        // Convert JS training patterns to Rust format
        let mut patterns = Vec::new();

        for js_pattern in training_patterns {
            // Expect patterns in format: { chords: WasmChord[], name: string, key?: string }
            let pattern_obj = js_pattern.dyn_into::<js_sys::Object>()?;
            let chords_val = js_sys::Reflect::get(&pattern_obj, &JsValue::from_str("chords"))?;
            let name_val = js_sys::Reflect::get(&pattern_obj, &JsValue::from_str("name"))?;
            let key_val = js_sys::Reflect::get(&pattern_obj, &JsValue::from_str("key")).ok();

            let chords_array = chords_val.dyn_into::<js_sys::Array>()?;
            let mut chord_pattern = Vec::new();

            for i in 0..chords_array.length() {
                let chord_js = chords_array.get(i);
                // For now, we'll need to handle chord conversion
                // This is a simplified approach - in practice, we'd want better serialization
                if let Some(chord_hex) = chord_js.as_string() {
                    let binary = hex_to_chord_binary(&chord_hex)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let chord = deserialize_chord(&binary)
                        .map_err(|e| JsValue::from_str(&e.to_string()))?;
                    chord_pattern.push(chord);
                }
            }

            let name = name_val
                .as_string()
                .unwrap_or_else(|| "unknown".to_string());
            let key = key_val.and_then(|v| v.as_string());

            patterns.push((chord_pattern, name, key));
        }

        self.inner
            .initialize(patterns)
            .map_err(|e| JsValue::from_str(&format!("AI Engine initialization failed: {}", e)))
    }

    /// Check if the engine is initialized
    #[wasm_bindgen(getter, js_name = "isInitialized")]
    pub fn is_initialized(&self) -> bool {
        self.inner.is_initialized()
    }

    /// Get magic chord solutions
    #[wasm_bindgen(js_name = "getMagicChordSolutions")]
    pub fn get_magic_chord_solutions(
        &self,
        previous_chords: Vec<JsValue>,
        following_chords: Vec<JsValue>,
        scale: &str,
        limit: usize,
    ) -> Result<Vec<JsValue>, JsValue> {
        // Convert JS chords to Rust chords
        let prev_chords = self.convert_js_chords_to_rust(previous_chords)?;
        let follow_chords = self.convert_js_chords_to_rust(following_chords)?;

        let suggestions = self
            .inner
            .get_magic_chord_solutions(&prev_chords, &follow_chords, scale, limit)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Convert suggestions to JS objects
        let mut js_suggestions = Vec::new();
        for suggestion in suggestions {
            let js_suggestion = self.suggestion_to_js(&suggestion)?;
            js_suggestions.push(js_suggestion);
        }

        Ok(js_suggestions)
    }

    /// Get bass harmonization solutions
    #[wasm_bindgen(js_name = "getMagicBassSolutions")]
    pub fn get_magic_bass_solutions(
        &self,
        bass_note: &str,
        scale: &str,
        limit: usize,
    ) -> Result<Vec<JsValue>, JsValue> {
        let suggestions = self
            .inner
            .get_magic_bass_solutions(bass_note, scale, limit)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let mut js_suggestions = Vec::new();
        for suggestion in suggestions {
            let js_suggestion = self.suggestion_to_js(&suggestion)?;
            js_suggestions.push(js_suggestion);
        }

        Ok(js_suggestions)
    }

    /// Get scale degree harmonization solutions
    #[wasm_bindgen(js_name = "getHarmonizeBySdSolutions")]
    pub fn get_harmonize_by_sd_solutions(
        &self,
        scale_degree_bits: u32,
        scale: &str,
        limit: usize,
    ) -> Result<Vec<JsValue>, JsValue> {
        let suggestions = self
            .inner
            .get_harmonize_by_sd_solutions(scale_degree_bits, scale, limit)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let mut js_suggestions = Vec::new();
        for suggestion in suggestions {
            let js_suggestion = self.suggestion_to_js(&suggestion)?;
            js_suggestions.push(js_suggestion);
        }

        Ok(js_suggestions)
    }

    /// Assess difficulty of a chord progression
    #[wasm_bindgen(js_name = "assessDifficulty")]
    pub fn assess_difficulty(
        &self,
        progression: Vec<JsValue>,
        tempo_bpm: Option<f64>,
        time_signature: Option<Vec<u8>>,
    ) -> Result<JsValue, JsValue> {
        let chords = self.convert_js_chords_to_rust(progression)?;

        let time_sig = time_signature.and_then(|v| {
            if v.len() >= 2 {
                Some((v[0], v[1]))
            } else {
                None
            }
        });

        let assessment = self
            .inner
            .assess_difficulty(&chords, tempo_bpm, time_sig)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Convert assessment to JS object
        let js_obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("overallScore"),
            &JsValue::from_f64(assessment.overall_score),
        )?;
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("skillLevel"),
            &JsValue::from_str(&format!("{:?}", assessment.skill_level)),
        )?;
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("confidence"),
            &JsValue::from_f64(assessment.confidence),
        )?;

        Ok(js_obj.into())
    }

    /// Get engine performance metrics
    #[wasm_bindgen(js_name = "getMetrics")]
    pub fn get_metrics(&self) -> JsValue {
        let metrics = self.inner.get_metrics();

        let js_obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("totalRequests"),
            &JsValue::from_f64(metrics.total_requests as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("avgResponseTimeMs"),
            &JsValue::from_f64(metrics.avg_response_time_ms),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("memoryUsageBytes"),
            &JsValue::from_f64(metrics.memory_usage_bytes as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("cacheHitRate"),
            &JsValue::from_f64(metrics.cache_hit_rate),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("totalPatterns"),
            &JsValue::from_f64(metrics.total_patterns as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("uptimeSeconds"),
            &JsValue::from_f64(metrics.uptime_seconds as f64),
        )
        .unwrap();

        js_obj.into()
    }

    /// Clear all caches
    #[wasm_bindgen(js_name = "clearCaches")]
    pub fn clear_caches(&self) {
        self.inner.clear_caches();
    }

    /// Shutdown the engine
    #[wasm_bindgen(js_name = "shutdown")]
    pub fn shutdown(&self) -> Result<(), JsValue> {
        self.inner
            .shutdown()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    // Helper methods for converting between JS and Rust types
    fn convert_js_chords_to_rust(&self, js_chords: Vec<JsValue>) -> Result<Vec<Chord>, JsValue> {
        let mut chords = Vec::new();

        for js_chord in js_chords {
            if let Some(chord_hex) = js_chord.as_string() {
                let binary = hex_to_chord_binary(&chord_hex)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                let chord =
                    deserialize_chord(&binary).map_err(|e| JsValue::from_str(&e.to_string()))?;
                chords.push(chord);
            }
        }

        Ok(chords)
    }

    fn suggestion_to_js(&self, suggestion: &ChordSuggestion) -> Result<JsValue, JsValue> {
        let js_obj = js_sys::Object::new();

        // Serialize chord to hex for JS consumption
        let chord_binary =
            serialize_chord(&suggestion.chord).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let chord_hex = chord_binary_to_hex(&chord_binary);

        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("chordHex"),
            &JsValue::from_str(&chord_hex),
        )?;
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("confidence"),
            &JsValue::from_f64(suggestion.confidence),
        )?;
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("weightedScore"),
            &JsValue::from_f64(suggestion.weighted_score),
        )?;
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("reasoning"),
            &JsValue::from_str(&suggestion.reasoning),
        )?;

        Ok(js_obj.into())
    }
}

/// WASM wrapper for ChordProgressionTrie (Pattern Matching)
#[wasm_bindgen]
pub struct WasmTrieNode {
    inner: ChordProgressionTrie,
}

#[wasm_bindgen]
impl WasmTrieNode {
    /// Create a new empty trie
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTrieNode {
        WasmTrieNode {
            inner: ChordProgressionTrie::new(),
        }
    }

    /// Add a pattern to the trie
    #[wasm_bindgen(js_name = "addPattern")]
    pub fn add_pattern(
        &self,
        pattern: Vec<JsValue>,
        source_id: &str,
        key_tonic: Option<String>,
    ) -> Result<(), JsValue> {
        // Convert JS chord pattern to Rust chords
        let mut chord_pattern = Vec::new();

        for js_chord in pattern {
            if let Some(chord_hex) = js_chord.as_string() {
                let binary = hex_to_chord_binary(&chord_hex)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                let chord =
                    deserialize_chord(&binary).map_err(|e| JsValue::from_str(&e.to_string()))?;
                chord_pattern.push(chord);
            }
        }

        self.inner
            .add_pattern(&chord_pattern, source_id.to_string(), key_tonic)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Calculate ranks for all patterns
    #[wasm_bindgen(js_name = "calculateAllRanks")]
    pub fn calculate_all_ranks(&self) {
        self.inner.calculate_all_ranks();
    }

    /// Get trie statistics
    #[wasm_bindgen(js_name = "getStatistics")]
    pub fn get_statistics(&self) -> JsValue {
        let stats = self.inner.statistics();
        let js_obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("totalPatterns"),
            &JsValue::from_f64(stats.total_patterns as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("totalNodes"),
            &JsValue::from_f64(stats.total_nodes as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("memoryUsageBytes"),
            &JsValue::from_f64(stats.memory_usage_bytes as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("scaleBranches"),
            &JsValue::from_f64(stats.scale_branches as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("maxDepth"),
            &JsValue::from_f64(stats.max_depth as f64),
        )
        .unwrap();
        js_sys::Reflect::set(
            &js_obj,
            &JsValue::from_str("avgBranchingFactor"),
            &JsValue::from_f64(stats.avg_branching_factor),
        )
        .unwrap();
        js_obj.into()
    }

    /// Get total patterns stored
    #[wasm_bindgen(getter, js_name = "totalPatterns")]
    pub fn total_patterns(&self) -> u64 {
        self.inner.total_patterns()
    }

    /// Get memory usage in bytes
    #[wasm_bindgen(getter, js_name = "memoryUsage")]
    pub fn memory_usage(&self) -> u64 {
        self.inner.memory_usage()
    }
}

/// Utility functions for chord symbol parsing
#[wasm_bindgen(js_name = "parseChordSymbol")]
pub fn parse_chord_symbol(symbol: &str, _scale_name: &str) -> Result<Vec<JsValue>, JsValue> {
    // This is a simplified chord symbol parser
    // In a full implementation, this would use a more sophisticated parser

    let mut candidate_chords = Vec::new();

    // Simple Roman numeral parsing
    if symbol
        .chars()
        .all(|c| c.is_alphabetic() || "°#♭".contains(c))
    {
        // Try to parse as Roman numeral
        let root = match symbol
            .to_lowercase()
            .trim_end_matches("7")
            .trim_end_matches("°")
        {
            "i" => 1,
            "ii" => 2,
            "iii" => 3,
            "iv" => 4,
            "v" => 5,
            "vi" => 6,
            "vii" => 7,
            _ => return Err(JsValue::from_str("Unrecognized Roman numeral")),
        };

        let chord_type = if symbol.contains("7") { 7 } else { 5 };

        if let Ok(chord) = Chord::new(root, chord_type) {
            let binary = serialize_chord(&chord).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let hex = chord_binary_to_hex(&binary);
            candidate_chords.push(JsValue::from_str(&hex));
        }
    }

    Ok(candidate_chords)
}

/// Check if two chords are isotonal (harmonically equivalent)
#[wasm_bindgen(js_name = "isIsotonal")]
pub fn is_isotonal(
    chord1_hex: &str,
    chord2_hex: &str,
    scale: &WasmScaleFingerprint,
) -> Result<bool, JsValue> {
    // Deserialize chords
    let binary1 = hex_to_chord_binary(chord1_hex).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let chord1 = deserialize_chord(&binary1).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let binary2 = hex_to_chord_binary(chord2_hex).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let chord2 = deserialize_chord(&binary2).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Get scale degrees for both chords
    let degrees1 = get_stable_scale_degrees(&chord1, &scale.inner)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let degrees2 = get_stable_scale_degrees(&chord2, &scale.inner)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Simple isotonal check: same scale degrees
    Ok(degrees1 == degrees2)
}

/// Utility function for debugging - log to console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    #[test]
    fn test_chord_creation() {
        let chord = WasmChord::new(1, 5).unwrap();
        assert_eq!(chord.root(), 1);
        assert_eq!(chord.chord_type(), 5);
        assert!(chord.is_triad());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_chord_serialization() {
        let chord = WasmChord::new(1, 5).unwrap();
        let hex = chord.to_hex().unwrap();
        let deserialized = WasmChord::from_hex(&hex).unwrap();

        assert_eq!(chord.root(), deserialized.root());
        assert_eq!(chord.chord_type(), deserialized.chord_type());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_scale_creation() {
        let major = WasmScaleFingerprint::major();
        assert!(major.is_diatonic());
        assert_eq!(major.note_count(), 7);
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_stable_scale_degrees() {
        let chord = WasmChord::new(1, 5).unwrap();
        let scale = WasmScaleFingerprint::major();
        let degrees = get_stable_scale_degrees_wasm(&chord, &scale).unwrap();

        assert_eq!(degrees.len(), 3);
        assert_eq!(degrees[0], "1");
        assert_eq!(degrees[1], "3");
        assert_eq!(degrees[2], "5");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_chord_complexity() {
        let triad = WasmChord::new(1, 5).unwrap();
        let complexity = get_chord_complexity_wasm(&triad, "major").unwrap();
        assert_eq!(complexity, 1.0);

        let seventh = WasmChord::new(5, 7).unwrap();
        let complexity = get_chord_complexity_wasm(&seventh, "major").unwrap();
        assert_eq!(complexity, 2.0);
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_roman_numeral() {
        let chord = WasmChord::new(1, 5).unwrap();
        let scale = WasmScaleFingerprint::major();
        let roman = get_roman_numeral_wasm(&chord, &scale).unwrap();
        assert_eq!(roman, "I");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_version() {
        let version = get_version();
        assert_eq!(version, "2.35.2");
    }
}
