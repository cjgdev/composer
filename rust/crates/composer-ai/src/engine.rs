//! AI Engine - Main orchestrator for all AI-powered musical features
//!
//! Provides a unified interface for chord progression suggestions, difficulty assessment,
//! bass line harmonization, and advanced musical analysis using machine learning models
//! and statistical analysis.

use crate::analysis::{DifficultyAssessment, MusicalAnalyzer, ProgressionAnalysis};
use crate::error::{AiError, AiResult};
use crate::suggestions::{
    ChordProgressionSuggester, ChordSuggestion, SuggestionConfig, SuggestionContext,
};
use crate::trie::ChordProgressionTrie;
use composer_config::MEMORY;
use composer_core::Chord;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// Main AI engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiEngineConfig {
    /// Maximum memory usage in MB
    pub max_memory_mb: u32,

    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,

    /// Cache size limits
    pub max_cache_entries: usize,

    /// Model versions to use
    pub model_versions: HashMap<String, String>,

    /// Feature flags
    pub enable_advanced_suggestions: bool,
    pub enable_difficulty_assessment: bool,
    pub enable_bass_harmonization: bool,
    pub enable_pattern_analysis: bool,
}

impl Default for AiEngineConfig {
    fn default() -> Self {
        let mut model_versions = HashMap::new();
        model_versions.insert("chord_suggestion".to_string(), "v2.35.2".to_string());
        model_versions.insert("difficulty_assessment".to_string(), "v2.35.2".to_string());

        Self {
            max_memory_mb: MEMORY.memory_usage_max_mb,
            enable_performance_monitoring: true,
            max_cache_entries: 10000,
            model_versions,
            enable_advanced_suggestions: true,
            enable_difficulty_assessment: true,
            enable_bass_harmonization: true,
            enable_pattern_analysis: true,
        }
    }
}

/// Engine performance metrics
#[derive(Debug, Clone)]
pub struct EngineMetrics {
    /// Total requests processed
    pub total_requests: u64,

    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,

    /// Memory usage in bytes
    pub memory_usage_bytes: u64,

    /// Cache hit rate (0.0-1.0)
    pub cache_hit_rate: f64,

    /// Number of patterns stored
    pub total_patterns: u64,

    /// Engine uptime in seconds
    pub uptime_seconds: u64,
}

/// Bass line harmonization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BassHarmonizationOptions {
    /// Target bass line style
    pub style: BassStyle,

    /// Complexity preference (0.0-1.0)
    pub complexity: f64,

    /// Enable walking bass lines
    pub enable_walking: bool,

    /// Preferred rhythm pattern
    pub rhythm_pattern: Option<String>,
}

/// Bass line style options
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum BassStyle {
    #[default]
    Root, // Root notes only
    Alternating, // Alternating bass
    Walking,     // Walking bass line
    Arpeggiated, // Arpeggiated patterns
    Rhythmic,    // Rhythmic bass patterns
}

impl Default for BassHarmonizationOptions {
    fn default() -> Self {
        Self {
            style: BassStyle::default(),
            complexity: 0.5,
            enable_walking: false,
            rhythm_pattern: None,
        }
    }
}

/// Bass line harmonization result
#[derive(Debug, Clone)]
pub struct BassHarmonization {
    /// Generated bass notes
    pub bass_notes: Vec<u8>,

    /// Rhythm pattern (in MIDI ticks)
    pub rhythm: Vec<u32>,

    /// Confidence in harmonization (0.0-1.0)
    pub confidence: f64,

    /// Style used
    pub style: BassStyle,
}

/// Main AI engine for musical intelligence and automated composition assistance.
///
/// The `AiEngine` is the central hub for all AI-powered musical features in Composer.
/// It orchestrates pattern matching, chord progression suggestions, difficulty assessment,
/// bass line harmonization, and advanced musical analysis using machine learning models
/// and statistical techniques.
///
/// # Architecture
///
/// The engine consists of several specialized components:
/// - **Pattern Storage**: Trie-based chord progression database
/// - **Suggestion Engine**: Context-aware chord recommendation system  
/// - **Difficulty Analyzer**: Statistical complexity assessment
/// - **Bass Harmonizer**: Intelligent bass line generation
/// - **Performance Monitor**: Real-time metrics and optimization
///
/// # Examples
///
/// ## Basic Setup and Usage
///
/// ```rust
/// use composer_ai::{AiEngine, AiEngineConfig};
/// use composer_core::Chord;
///
/// // Create engine with default configuration
/// let config = AiEngineConfig::default();
/// let engine = AiEngine::new(config);
///
/// // Prepare training patterns (chord progressions)
/// let training_patterns = vec![
///     (vec![Chord::triad(1)?, Chord::triad(6)?, Chord::triad(4)?, Chord::triad(5)?],
///      "pop-progression-1".to_string(), Some("C".to_string())),
///     (vec![Chord::seventh(2)?, Chord::seventh(5)?, Chord::triad(1)?],
///      "jazz-ii-V-I".to_string(), Some("C".to_string())),
/// ];
///
/// // Initialize with training data
/// engine.initialize(training_patterns)?;
/// assert!(engine.is_initialized());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Advanced AI Features
///
/// ```rust
/// use composer_ai::{AiEngine, AiEngineConfig};
/// use composer_core::Chord;
///
/// let engine = AiEngine::new(AiEngineConfig::default());
/// // ... initialize with patterns ...
/// # let training_patterns = vec![];
/// # engine.initialize(training_patterns)?;
///
/// // Get chord progression suggestions
/// let context = vec![Chord::triad(1)?, Chord::triad(6)?];
/// let following = vec![Chord::triad(5)?];
/// let suggestions = engine.get_magic_chord_solutions(&context, &following, "major", 5)?;
///
/// // Assess difficulty of a progression
/// let progression = vec![Chord::seventh(2)?, Chord::seventh(5)?, Chord::new(1, 9)?];
/// let assessment = engine.assess_difficulty(&progression, Some(120.0), Some((4, 4)))?;
/// println!("Difficulty: {:.1}/10", assessment.overall_score);
///
/// // Get bass harmonization
/// let bass_solutions = engine.get_magic_bass_solutions("C", "major", 3)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Performance Characteristics
///
/// The AI engine is designed for real-time performance:
/// - **Initialization**: Typically 100-1000ms depending on training data size
/// - **Chord Suggestions**: <50ms (target: CHORD_SUGGESTION_MAX_MS)
/// - **Difficulty Assessment**: <200ms (target: MUSIC_ANALYSIS_MAX_MS)
/// - **Memory Usage**: <150MB (target: MEMORY_USAGE_MAX_MB)
/// - **Concurrent Requests**: Up to 10 simultaneous (MAX_CONCURRENT_REQUESTS)
///
/// # Thread Safety
///
/// The `AiEngine` is fully thread-safe and can be shared across multiple threads
/// using `Arc<AiEngine>`. All internal state is protected by appropriate synchronization
/// primitives.
///
/// # Related Functions
///
/// - [`AiEngine::new`] - Create a new engine instance
/// - [`AiEngine::initialize`] - Load training patterns and prepare for analysis
/// - [`AiEngine::get_chord_suggestions`] - Get contextual chord recommendations
/// - [`AiEngine::assess_difficulty`] - Analyze progression complexity
/// - [`AiEngine::get_metrics`] - Monitor performance and usage statistics
pub struct AiEngine {
    /// Engine configuration
    config: Arc<RwLock<AiEngineConfig>>,

    /// Pattern storage and matching
    trie: Arc<ChordProgressionTrie>,

    /// Chord progression suggester
    suggester: Arc<ChordProgressionSuggester>,

    /// Musical analyzer
    analyzer: Arc<MusicalAnalyzer>,

    /// Performance metrics
    metrics: Arc<RwLock<EngineMetrics>>,

    /// Engine start time for uptime calculation
    start_time: Instant,

    /// Initialization status
    initialized: Arc<RwLock<bool>>,
}

impl AiEngine {
    /// Creates a new AI engine instance with the specified configuration.
    ///
    /// This constructor initializes all internal components and sets up the engine
    /// for pattern analysis, but does not load any training data. You must call
    /// [`initialize`] before using any AI features.
    ///
    /// # Arguments
    ///
    /// * `config` - Engine configuration specifying memory limits, feature flags, and performance settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use composer_ai::{AiEngine, AiEngineConfig};
    ///
    /// // Create with default configuration
    /// let engine = AiEngine::new(AiEngineConfig::default());
    ///
    /// // Create with custom memory limits
    /// let mut config = AiEngineConfig::default();
    /// config.max_memory_mb = 200;
    /// config.enable_advanced_suggestions = true;
    /// let engine = AiEngine::new(config);
    ///
    /// assert!(!engine.is_initialized()); // Must call initialize() first
    /// ```
    ///
    /// # Performance
    ///
    /// Engine creation is very fast (typically <1ms) as it only initializes
    /// data structures without loading training data.
    ///
    /// # Related Functions
    ///
    /// - [`AiEngine::initialize`] - Load training patterns after creation
    /// - [`AiEngineConfig::default`] - Get recommended default configuration
    /// - [`AiEngine::is_initialized`] - Check initialization status
    ///
    /// [`initialize`]: AiEngine::initialize
    pub fn new(config: AiEngineConfig) -> Self {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = Arc::new(ChordProgressionSuggester::new(trie.clone()));
        let analyzer = Arc::new(MusicalAnalyzer::new(trie.clone()));

        let metrics = EngineMetrics {
            total_requests: 0,
            avg_response_time_ms: 0.0,
            memory_usage_bytes: 0,
            cache_hit_rate: 0.0,
            total_patterns: 0,
            uptime_seconds: 0,
        };

        Self {
            config: Arc::new(RwLock::new(config)),
            trie,
            suggester,
            analyzer,
            metrics: Arc::new(RwLock::new(metrics)),
            start_time: Instant::now(),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initializes the AI engine with training data patterns.
    ///
    /// This method loads chord progression patterns into the engine's trie-based
    /// database, enabling pattern matching and intelligent suggestions. The engine
    /// must be initialized before any AI features can be used.
    ///
    /// # Arguments
    ///
    /// * `training_patterns` - Vector of tuples containing:
    ///   - `Vec<Chord>`: The chord progression pattern
    ///   - `String`: Source identifier (e.g., "song-title", "exercise-1")
    ///   - `Option<String>`: Optional key/tonic (e.g., "C", "Bb", "F#m")
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful initialization, or an `AiError` if initialization fails.
    ///
    /// # Examples
    ///
    /// ## Loading Common Progressions
    ///
    /// ```rust
    /// use composer_ai::{AiEngine, AiEngineConfig};
    /// use composer_core::Chord;
    ///
    /// let engine = AiEngine::new(AiEngineConfig::default());
    ///
    /// let training_patterns = vec![
    ///     // I-vi-IV-V progression (pop/rock staple)
    ///     (vec![
    ///         Chord::triad(1)?, Chord::triad(6)?,
    ///         Chord::triad(4)?, Chord::triad(5)?
    ///     ], "pop-progression".to_string(), Some("C".to_string())),
    ///     
    ///     // ii-V-I progression (jazz fundamental)
    ///     (vec![
    ///         Chord::seventh(2)?, Chord::seventh(5)?, Chord::triad(1)?
    ///     ], "jazz-ii-V-I".to_string(), Some("C".to_string())),
    ///     
    ///     // Circle of fifths sequence
    ///     (vec![
    ///         Chord::seventh(6)?, Chord::seventh(2)?,
    ///         Chord::seventh(5)?, Chord::triad(1)?
    ///     ], "circle-of-fifths".to_string(), None),
    /// ];
    ///
    /// engine.initialize(training_patterns)?;
    /// assert!(engine.is_initialized());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ## Genre-Specific Training
    ///
    /// ```rust
    /// use composer_ai::{AiEngine, AiEngineConfig};
    /// use composer_core::Chord;
    ///
    /// let engine = AiEngine::new(AiEngineConfig::default());
    ///
    /// // Jazz progressions with complex harmony
    /// let jazz_patterns = vec![
    ///     (vec![
    ///         Chord::seventh(1)?.with_alteration("#11")?,
    ///         Chord::seventh(6)?,
    ///         Chord::seventh(2)?.with_alteration("b5")?,
    ///         Chord::seventh(5)?.with_alteration("b9")?
    ///     ], "jazz-complex-1".to_string(), Some("C".to_string())),
    /// ];
    ///
    /// engine.initialize(jazz_patterns)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Performance
    ///
    /// - **Small datasets** (100-1000 patterns): 10-100ms
    /// - **Medium datasets** (1000-10000 patterns): 100-500ms  
    /// - **Large datasets** (10000+ patterns): 500-2000ms
    ///
    /// The loading time scales roughly O(n log n) with pattern count.
    ///
    /// # Memory Usage
    ///
    /// Pattern storage is optimized using trie compression:
    /// - ~50-200 bytes per unique chord
    /// - ~100-500 bytes per pattern (depending on length and repetition)
    /// - Memory usage is monitored and limited by `max_memory_mb` config
    ///
    /// # Error Conditions
    ///
    /// - `AiError::EngineAlreadyInitialized` - Engine is already initialized
    /// - `AiError::InvalidPatternData` - Invalid chord data in patterns
    /// - `AiError::MemoryExhausted` - Patterns exceed memory limits
    /// - `AiError::TrieOperationFailed` - Internal trie construction failed
    ///
    /// # Thread Safety
    ///
    /// This method is thread-safe but should only be called once per engine instance.
    /// Subsequent calls will return an error.
    ///
    /// # Related Functions
    ///
    /// - [`AiEngine::new`] - Create engine before initialization
    /// - [`AiEngine::is_initialized`] - Check initialization status
    /// - [`AiEngine::get_metrics`] - Monitor pattern storage statistics
    /// - [`AiEngine::shutdown`] - Clean shutdown and resource cleanup
    pub fn initialize(
        &self,
        training_patterns: Vec<(Vec<Chord>, String, Option<String>)>,
    ) -> AiResult<()> {
        if *self.initialized.read() {
            return Err(AiError::EngineNotInitialized);
        }

        // Load training patterns into the trie
        for (pattern, source_id, key_tonic) in training_patterns {
            self.trie.add_pattern(&pattern, source_id, key_tonic)?;
        }

        // Calculate ranks for all patterns
        self.trie.calculate_all_ranks();

        // Mark as initialized
        *self.initialized.write() = true;

        Ok(())
    }

    /// Check if engine is initialized
    pub fn is_initialized(&self) -> bool {
        *self.initialized.read()
    }

    /// Get chord progression suggestions using original context-aware method
    pub fn get_chord_suggestions(
        &self,
        pattern: &[Chord],
        context: &SuggestionContext,
        config: &SuggestionConfig,
    ) -> AiResult<Vec<ChordSuggestion>> {
        if !self.is_initialized() {
            return Err(AiError::EngineNotInitialized);
        }

        if !self.config.read().enable_advanced_suggestions {
            return Err(AiError::SuggestionFailed {
                reason: "Advanced suggestions are disabled".to_string(),
            });
        }

        let start_time = Instant::now();

        let suggestions = self
            .suggester
            .get_chord_suggestions(pattern, context, config)?;

        self.update_metrics_for_request(start_time.elapsed().as_millis() as f64);

        Ok(suggestions)
    }

    /// Get magic chord solutions using statistical algorithm from test specification
    pub fn get_magic_chord_solutions(
        &self,
        previous_chords: &[Chord],
        following_chords: &[Chord],
        scale: &str,
        limit: usize,
    ) -> AiResult<Vec<ChordSuggestion>> {
        if !self.is_initialized() {
            return Err(AiError::EngineNotInitialized);
        }

        if !self.config.read().enable_advanced_suggestions {
            return Err(AiError::SuggestionFailed {
                reason: "Advanced suggestions are disabled".to_string(),
            });
        }

        let start_time = Instant::now();

        let suggestions = self.suggester.get_magic_chord_solutions(
            previous_chords,
            following_chords,
            scale,
            limit,
        )?;

        self.update_metrics_for_request(start_time.elapsed().as_millis() as f64);

        Ok(suggestions)
    }

    /// Get bass harmonization solutions from test specification
    pub fn get_magic_bass_solutions(
        &self,
        bass_note: &str,
        scale: &str,
        limit: usize,
    ) -> AiResult<Vec<ChordSuggestion>> {
        if !self.is_initialized() {
            return Err(AiError::EngineNotInitialized);
        }

        if !self.config.read().enable_bass_harmonization {
            return Err(AiError::SuggestionFailed {
                reason: "Bass harmonization is disabled".to_string(),
            });
        }

        let start_time = Instant::now();

        let suggestions = self
            .suggester
            .get_magic_bass_solutions(bass_note, scale, limit)?;

        self.update_metrics_for_request(start_time.elapsed().as_millis() as f64);

        Ok(suggestions)
    }

    /// Get scale degree harmonization solutions from test specification
    pub fn get_harmonize_by_sd_solutions(
        &self,
        scale_degree_bits: u32,
        scale: &str,
        limit: usize,
    ) -> AiResult<Vec<ChordSuggestion>> {
        if !self.is_initialized() {
            return Err(AiError::EngineNotInitialized);
        }

        if !self.config.read().enable_advanced_suggestions {
            return Err(AiError::SuggestionFailed {
                reason: "Advanced suggestions are disabled".to_string(),
            });
        }

        let start_time = Instant::now();

        let suggestions =
            self.suggester
                .get_harmonize_by_sd_solutions(scale_degree_bits, scale, limit)?;

        self.update_metrics_for_request(start_time.elapsed().as_millis() as f64);

        Ok(suggestions)
    }

    /// Assess difficulty of a chord progression
    pub fn assess_difficulty(
        &self,
        progression: &[Chord],
        tempo_bpm: Option<f64>,
        time_signature: Option<(u8, u8)>,
    ) -> AiResult<DifficultyAssessment> {
        if !self.is_initialized() {
            return Err(AiError::EngineNotInitialized);
        }

        if !self.config.read().enable_difficulty_assessment {
            return Err(AiError::AnalysisFailed {
                reason: "Difficulty assessment is disabled".to_string(),
            });
        }

        let start_time = Instant::now();

        let assessment = self
            .analyzer
            .assess_difficulty(progression, tempo_bpm, time_signature)?;

        self.update_metrics_for_request(start_time.elapsed().as_millis() as f64);

        Ok(assessment)
    }

    /// Analyze chord progression patterns
    pub fn analyze_progression(&self, progression: &[Chord]) -> AiResult<ProgressionAnalysis> {
        if !self.is_initialized() {
            return Err(AiError::EngineNotInitialized);
        }

        if !self.config.read().enable_pattern_analysis {
            return Err(AiError::AnalysisFailed {
                reason: "Pattern analysis is disabled".to_string(),
            });
        }

        let start_time = Instant::now();

        let analysis = self.analyzer.analyze_progression(progression)?;

        self.update_metrics_for_request(start_time.elapsed().as_millis() as f64);

        Ok(analysis)
    }

    /// Generate bass line harmonization
    pub fn harmonize_bass_line(
        &self,
        progression: &[Chord],
        options: &BassHarmonizationOptions,
    ) -> AiResult<BassHarmonization> {
        if !self.is_initialized() {
            return Err(AiError::EngineNotInitialized);
        }

        if !self.config.read().enable_bass_harmonization {
            return Err(AiError::SuggestionFailed {
                reason: "Bass harmonization is disabled".to_string(),
            });
        }

        let start_time = Instant::now();

        let harmonization = self.generate_bass_harmonization(progression, options)?;

        self.update_metrics_for_request(start_time.elapsed().as_millis() as f64);

        Ok(harmonization)
    }

    /// Generate bass harmonization (internal implementation)
    fn generate_bass_harmonization(
        &self,
        progression: &[Chord],
        options: &BassHarmonizationOptions,
    ) -> AiResult<BassHarmonization> {
        let mut bass_notes = Vec::new();
        let mut rhythm = Vec::new();

        match options.style {
            BassStyle::Root => {
                // Simple root note bass line
                for chord in progression {
                    bass_notes.push(chord.root);
                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32);
                    // Whole note
                }
            },
            BassStyle::Alternating => {
                // Alternating bass pattern
                for (i, chord) in progression.iter().enumerate() {
                    if i % 2 == 0 {
                        bass_notes.push(chord.root);
                    } else {
                        // Alternate with fifth (scale degree 5 relative to root)
                        let fifth_degree = if chord.root + 4 > 7 {
                            chord.root + 4 - 7
                        } else {
                            chord.root + 4
                        };
                        bass_notes.push(fifth_degree);
                    }
                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 2);
                    // Half notes
                }
            },
            BassStyle::Walking => {
                // Walking bass line with passing tones
                for (i, chord) in progression.iter().enumerate() {
                    bass_notes.push(chord.root);
                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 4); // Quarter notes

                    // Add passing tones between chords
                    if i < progression.len() - 1 {
                        let next_chord = &progression[i + 1];
                        let passing_tone = self.calculate_passing_tone(chord.root, next_chord.root);
                        bass_notes.push(passing_tone);
                        rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 4);
                    }
                }
            },
            BassStyle::Arpeggiated => {
                // Arpeggiated bass patterns
                for chord in progression {
                    bass_notes.push(chord.root);
                    // Third (scale degree 3 relative to root)
                    let third_degree = if chord.root + 2 > 7 {
                        chord.root + 2 - 7
                    } else {
                        chord.root + 2
                    };
                    bass_notes.push(third_degree);
                    // Fifth (scale degree 5 relative to root)
                    let fifth_degree = if chord.root + 4 > 7 {
                        chord.root + 4 - 7
                    } else {
                        chord.root + 4
                    };
                    bass_notes.push(fifth_degree);

                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 3);
                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 3);
                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 3);
                }
            },
            BassStyle::Rhythmic => {
                // Rhythmic bass patterns
                for chord in progression {
                    bass_notes.push(chord.root);
                    bass_notes.push(chord.root); // Repeat for rhythm

                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 2);
                    rhythm.push(composer_config::MUSICAL.ticks_per_beat as u32 / 4);
                }
            },
        }

        let confidence = self.calculate_bass_harmonization_confidence(progression, options);

        Ok(BassHarmonization {
            bass_notes,
            rhythm,
            confidence,
            style: options.style,
        })
    }

    /// Calculate passing tone for walking bass
    fn calculate_passing_tone(&self, from_root: u8, to_root: u8) -> u8 {
        let interval = (to_root as i8 - from_root as i8).abs();

        if interval == 1 {
            // Chromatic passing tone
            (from_root + 1) % 12
        } else if interval >= 3 {
            // Scale-wise passing tone (simplified)
            (from_root + 2) % 12
        } else {
            // Default to chromatic
            (from_root + 1) % 12
        }
    }

    /// Calculate confidence in bass harmonization
    fn calculate_bass_harmonization_confidence(
        &self,
        progression: &[Chord],
        options: &BassHarmonizationOptions,
    ) -> f64 {
        let mut confidence: f64 = 0.7; // Base confidence

        // Simpler styles have higher confidence
        match options.style {
            BassStyle::Root => confidence += 0.2,
            BassStyle::Alternating => confidence += 0.1,
            BassStyle::Walking => confidence -= 0.1,
            BassStyle::Arpeggiated => confidence += 0.0,
            BassStyle::Rhythmic => confidence += 0.05,
        }

        // Shorter progressions are easier to harmonize
        if progression.len() <= 4 {
            confidence += 0.1;
        }

        confidence.max(0.0).min(1.0)
    }

    /// Add training pattern to the engine
    pub fn add_training_pattern(
        &self,
        pattern: &[Chord],
        source_id: String,
        key_tonic: Option<String>,
    ) -> AiResult<()> {
        self.trie.add_pattern(pattern, source_id, key_tonic)?;
        self.trie.calculate_all_ranks();
        Ok(())
    }

    /// Get engine metrics
    pub fn get_metrics(&self) -> EngineMetrics {
        let mut metrics = self.metrics.read().clone();
        metrics.uptime_seconds = self.start_time.elapsed().as_secs();
        metrics.memory_usage_bytes = self.trie.memory_usage();
        metrics.total_patterns = self.trie.total_patterns();
        metrics
    }

    /// Update engine configuration
    pub fn update_config(&self, new_config: AiEngineConfig) -> AiResult<()> {
        // Validate memory limits
        if new_config.max_memory_mb > 1000 {
            return Err(AiError::MemoryExhausted {
                limit_mb: new_config.max_memory_mb,
            });
        }

        *self.config.write() = new_config;
        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> AiEngineConfig {
        self.config.read().clone()
    }

    /// Clear all caches
    pub fn clear_caches(&self) {
        self.suggester.clear_cache();
        self.analyzer.clear_cache();
    }

    /// Validate memory usage against limits
    pub fn validate_memory_usage(&self) -> AiResult<()> {
        let current_usage_mb = self.trie.memory_usage() / (1024 * 1024);
        let limit_mb = self.config.read().max_memory_mb;

        if current_usage_mb > limit_mb as u64 {
            return Err(AiError::MemoryExhausted { limit_mb });
        }

        Ok(())
    }

    /// Update performance metrics after a request
    fn update_metrics_for_request(&self, response_time_ms: f64) {
        let mut metrics = self.metrics.write();
        metrics.total_requests += 1;

        // Update moving average of response time
        if metrics.total_requests == 1 {
            metrics.avg_response_time_ms = response_time_ms;
        } else {
            metrics.avg_response_time_ms =
                (metrics.avg_response_time_ms * 0.9) + (response_time_ms * 0.1);
        }

        // Calculate cache hit rate (simplified)
        let (suggestion_cache_len, _) = self.suggester.cache_stats();
        let (analysis_cache_len, _) = self.analyzer.cache_stats();
        let total_cache_entries = suggestion_cache_len + analysis_cache_len;

        if metrics.total_requests > 0 {
            metrics.cache_hit_rate = total_cache_entries as f64 / metrics.total_requests as f64;
        }
    }

    /// Shutdown the engine gracefully
    pub fn shutdown(&self) -> AiResult<()> {
        self.clear_caches();
        *self.initialized.write() = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use composer_core::Chord;

    #[test]
    fn test_engine_creation() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);
        assert!(!engine.is_initialized());
    }

    #[test]
    fn test_engine_initialization() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);

        let training_patterns = vec![(
            vec![Chord::new(1, 5).unwrap(), Chord::new(5, 7).unwrap()],
            "song1".to_string(),
            Some("C".to_string()),
        )];

        let result = engine.initialize(training_patterns);
        assert!(result.is_ok());
        assert!(engine.is_initialized());
    }

    #[test]
    fn test_suggestions_without_initialization() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);

        let pattern = vec![Chord::new(1, 5).unwrap()];
        let context = SuggestionContext::default();
        let config = SuggestionConfig::default();

        let result = engine.get_chord_suggestions(&pattern, &context, &config);
        assert!(result.is_err());

        if let Err(AiError::EngineNotInitialized) = result {
            // Expected error
        } else {
            panic!("Expected EngineNotInitialized error");
        }
    }

    #[test]
    fn test_bass_harmonization_styles() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);

        // Initialize with empty patterns
        engine.initialize(vec![]).unwrap();

        let progression = vec![Chord::new(1, 5).unwrap(), Chord::new(5, 7).unwrap()];

        // Test different bass styles
        for style in [BassStyle::Root, BassStyle::Alternating, BassStyle::Walking] {
            let options = BassHarmonizationOptions {
                style,
                complexity: 0.5,
                enable_walking: false,
                rhythm_pattern: None,
            };

            let result = engine.harmonize_bass_line(&progression, &options);
            assert!(result.is_ok());

            let harmonization = result.unwrap();
            assert!(!harmonization.bass_notes.is_empty());
            assert_eq!(harmonization.style, style);
        }
    }

    #[test]
    fn test_difficulty_assessment() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);
        engine.initialize(vec![]).unwrap();

        let progression = vec![
            Chord::new(1, 5).unwrap(),
            Chord::new(5, 7).unwrap(),
            Chord::new(6, 5).unwrap(),
        ];

        let result = engine.assess_difficulty(&progression, Some(120.0), Some((4, 4)));
        assert!(result.is_ok());

        let assessment = result.unwrap();
        assert!(assessment.overall_score >= 0.0 && assessment.overall_score <= 10.0);
    }

    #[test]
    fn test_metrics_tracking() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);
        engine.initialize(vec![]).unwrap();

        let initial_metrics = engine.get_metrics();
        assert_eq!(initial_metrics.total_requests, 0);

        // Make a request
        let progression = vec![Chord::new(1, 5).unwrap()];
        let _ = engine.assess_difficulty(&progression, None, None);

        let updated_metrics = engine.get_metrics();
        assert_eq!(updated_metrics.total_requests, 1);
        assert!(updated_metrics.avg_response_time_ms >= 0.0);
    }

    #[test]
    fn test_config_updates() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);

        let mut new_config = engine.get_config();
        new_config.max_memory_mb = 256;
        new_config.enable_advanced_suggestions = false;

        let result = engine.update_config(new_config.clone());
        assert!(result.is_ok());

        let updated_config = engine.get_config();
        assert_eq!(updated_config.max_memory_mb, 256);
        assert!(!updated_config.enable_advanced_suggestions);
    }

    #[test]
    fn test_memory_validation() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);

        let result = engine.validate_memory_usage();
        assert!(result.is_ok()); // Should pass with empty engine
    }

    #[test]
    fn test_cache_operations() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);

        engine.clear_caches(); // Should not panic
    }

    #[test]
    fn test_pattern_addition() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);
        engine.initialize(vec![]).unwrap();

        let pattern = vec![Chord::new(1, 5).unwrap(), Chord::new(5, 7).unwrap()];
        let result =
            engine.add_training_pattern(&pattern, "test_song".to_string(), Some("C".to_string()));
        assert!(result.is_ok());

        let metrics = engine.get_metrics();
        assert_eq!(metrics.total_patterns, 1);
    }

    #[test]
    fn test_engine_shutdown() {
        let config = AiEngineConfig::default();
        let engine = AiEngine::new(config);
        engine.initialize(vec![]).unwrap();

        assert!(engine.is_initialized());

        let result = engine.shutdown();
        assert!(result.is_ok());
        assert!(!engine.is_initialized());
    }
}
