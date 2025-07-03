//! Chord progression suggestion algorithms
//!
//! Implements AI-powered chord progression generation using pattern matching,
//! context-aware weighting, and statistical analysis of musical progressions.

use crate::error::{AiError, AiResult};
use crate::trie::{ChordProgressionTrie, PatternResult};
use composer_config::{PERFORMANCE, PROCESSING, QUALITY};
use composer_core::{Chord, ScaleFingerprint};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// Suggestion context for contextual weighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionContext {
    /// Current scale/key context
    pub scale_fingerprint: Option<ScaleFingerprint>,

    /// Position in progression (0.0-1.0)
    pub position_in_progression: f64,

    /// Target emotional valence (-1.0 to 1.0)
    pub target_valence: f64,

    /// Complexity preference (0.0-1.0)
    pub complexity_preference: f64,

    /// Genre preferences with weights
    pub genre_weights: HashMap<String, f64>,

    /// Avoid repetition within this many chords
    pub avoid_repetition_within: usize,

    /// Recent chord history for repetition avoidance
    pub recent_chords: SmallVec<[Chord; 8]>,
}

/// Weighted chord suggestion result
#[derive(Debug, Clone)]
pub struct ChordSuggestion {
    /// The suggested chord
    pub chord: Chord,

    /// Overall suggestion confidence (0.0-1.0)
    pub confidence: f64,

    /// Pattern frequency score
    pub frequency_score: f64,

    /// Context relevance score
    pub context_score: f64,

    /// Theoretical appropriateness score
    pub theory_score: f64,

    /// Final weighted score
    pub weighted_score: f64,

    /// Source pattern information
    pub pattern_info: PatternResult,

    /// Explanation of suggestion reasoning
    pub reasoning: String,
}

/// Configuration for suggestion generation
#[derive(Debug, Clone)]
pub struct SuggestionConfig {
    /// Maximum number of suggestions to return
    pub max_suggestions: usize,

    /// Minimum confidence threshold (0.0-1.0)
    pub min_confidence: f64,

    /// Pattern search depth
    pub search_depth: usize,

    /// Use probabilistic vs deterministic selection
    pub use_probabilistic: bool,

    /// Temperature for probabilistic selection (0.0-2.0)
    pub temperature: f64,

    /// Enable advanced context weighting
    pub enable_context_weighting: bool,
}

impl Default for SuggestionContext {
    fn default() -> Self {
        Self {
            scale_fingerprint: None,
            position_in_progression: 0.5,
            target_valence: 0.0,
            complexity_preference: 0.5,
            genre_weights: HashMap::new(),
            avoid_repetition_within: 4,
            recent_chords: SmallVec::new(),
        }
    }
}

impl Default for SuggestionConfig {
    fn default() -> Self {
        Self {
            max_suggestions: PROCESSING.max_suggestions as usize,
            min_confidence: QUALITY.confidence_threshold,
            search_depth: 3, // Default search depth
            use_probabilistic: false,
            temperature: 1.0,
            enable_context_weighting: true,
        }
    }
}

/// Main chord progression suggestion engine
pub struct ChordProgressionSuggester {
    /// Pattern storage trie
    trie: Arc<ChordProgressionTrie>,

    /// Cached suggestions for performance
    suggestion_cache: Arc<dashmap::DashMap<String, Vec<ChordSuggestion>>>,

    /// Performance metrics
    avg_suggestion_time_ms: Arc<parking_lot::RwLock<f64>>,
}

impl ChordProgressionSuggester {
    /// Create a new chord progression suggester
    pub fn new(trie: Arc<ChordProgressionTrie>) -> Self {
        Self {
            trie,
            suggestion_cache: Arc::new(dashmap::DashMap::new()),
            avg_suggestion_time_ms: Arc::new(parking_lot::RwLock::new(0.0)),
        }
    }

    /// Generate chord suggestions using magic chord algorithm from test specifications
    /// Implements getMagicChordSolutions with proper statistical weighting
    pub fn get_magic_chord_solutions(
        &self,
        previous_chords: &[Chord],
        following_chords: &[Chord],
        _scale: &str,
        limit: usize,
    ) -> AiResult<Vec<ChordSuggestion>> {
        let start_time = Instant::now();

        // Validate inputs according to test specs
        if previous_chords.len() > 50 || following_chords.len() > 50 {
            return Err(AiError::InvalidPattern {
                reason: "Maximum 50 chords allowed in pattern".to_string(),
            });
        }

        if limit > 100 {
            return Err(AiError::InvalidPattern {
                reason: "Maximum 100 suggestions allowed".to_string(),
            });
        }

        // Create search pattern with wildcard: [...previous, "*", ...following]
        let mut search_pattern: Vec<Option<Chord>> = previous_chords
            .iter()
            .map(|chord| Some(chord.clone()))
            .collect();
        search_pattern.push(None); // Wildcard for suggestions
        search_pattern.extend(following_chords.iter().map(|chord| Some(chord.clone())));

        // Search for patterns in scale-specific trie branch
        let pattern_results = self.trie.search_with_wildcard(
            &search_pattern,
            limit * 3, // Get more results for filtering
        )?;

        // Apply statistical weighting algorithm from test specs (lines 150-176)
        let suggestions = self.apply_magic_chord_weighting(
            pattern_results,
            previous_chords.len(),
            following_chords.len(),
            search_pattern.len(),
            limit,
        )?;

        // Check performance threshold (sub-millisecond requirement)
        let elapsed_ms = start_time.elapsed().as_millis() as f64;
        self.update_performance_metrics(elapsed_ms);

        if elapsed_ms > PERFORMANCE.chord_lookup_max_ms as f64 {
            return Err(AiError::PerformanceDegradation {
                operation: "magic_chord_solutions".to_string(),
                ms: elapsed_ms as u64,
                limit_ms: PERFORMANCE.chord_lookup_max_ms as u64,
            });
        }

        Ok(suggestions)
    }

    /// Original context-aware suggestion method for backward compatibility
    pub fn get_chord_suggestions(
        &self,
        pattern: &[Chord],
        context: &SuggestionContext,
        config: &SuggestionConfig,
    ) -> AiResult<Vec<ChordSuggestion>> {
        let start_time = Instant::now();

        // Validate inputs
        if pattern.is_empty() {
            return Err(AiError::InvalidPattern {
                reason: "Pattern cannot be empty".to_string(),
            });
        }

        if pattern.len() > 20 {
            // Max pattern length
            return Err(AiError::InvalidPattern {
                reason: format!("Pattern too long: {} (max: {})", pattern.len(), 20),
            });
        }

        // Check cache first
        let cache_key = self.generate_cache_key(pattern, context, config);
        if let Some(cached) = self.suggestion_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Create search pattern with wildcard at the end
        let mut search_pattern: Vec<Option<Chord>> =
            pattern.iter().map(|chord| Some(chord.clone())).collect();
        search_pattern.push(None); // Wildcard for suggestions

        // Search for patterns
        let pattern_results = self.trie.search_with_wildcard(
            &search_pattern,
            config.max_suggestions * 3, // Get more results for filtering
        )?;

        // Convert pattern results to suggestions with scoring
        let suggestions =
            self.score_and_rank_suggestions(pattern_results, pattern, context, config)?;

        // Cache the results
        self.suggestion_cache.insert(cache_key, suggestions.clone());

        // Update performance metrics
        let elapsed_ms = start_time.elapsed().as_millis() as f64;
        self.update_performance_metrics(elapsed_ms);

        // Check performance threshold
        if elapsed_ms > PERFORMANCE.chord_lookup_max_ms as f64 {
            return Err(AiError::PerformanceDegradation {
                operation: "chord_suggestion".to_string(),
                ms: elapsed_ms as u64,
                limit_ms: PERFORMANCE.chord_lookup_max_ms as u64,
            });
        }

        Ok(suggestions)
    }

    /// Score and rank pattern results as chord suggestions
    fn score_and_rank_suggestions(
        &self,
        pattern_results: Vec<PatternResult>,
        current_pattern: &[Chord],
        context: &SuggestionContext,
        config: &SuggestionConfig,
    ) -> AiResult<Vec<ChordSuggestion>> {
        let suggestions: Vec<ChordSuggestion> = pattern_results
            .par_iter()
            .filter_map(|result| {
                // Deserialize the chord from binary
                let chord =
                    match composer_serialization::deserialize_chord(&result.serialized_chord) {
                        Ok(chord) => chord,
                        Err(_) => return None,
                    };

                // Check for repetition avoidance
                if self.should_avoid_chord(&chord, context) {
                    return None;
                }

                // Calculate various scores
                let frequency_score = self.calculate_frequency_score(result);
                let context_score = if config.enable_context_weighting {
                    self.calculate_context_score(&chord, context)
                } else {
                    0.5 // Neutral score when context weighting is disabled
                };
                let theory_score = self.calculate_theory_score(&chord, current_pattern, context);

                // Calculate weighted final score
                let weighted_score = self.calculate_weighted_score(
                    frequency_score,
                    context_score,
                    theory_score,
                    config,
                );

                // Calculate overall confidence
                let confidence = self.calculate_confidence(weighted_score, result);

                // Filter by minimum confidence
                if confidence < config.min_confidence {
                    return None;
                }

                // Generate reasoning explanation
                let reasoning = self.generate_reasoning(
                    &chord,
                    current_pattern,
                    context,
                    frequency_score,
                    context_score,
                    theory_score,
                );

                Some(ChordSuggestion {
                    chord,
                    confidence,
                    frequency_score,
                    context_score,
                    theory_score,
                    weighted_score,
                    pattern_info: result.clone(),
                    reasoning,
                })
            })
            .collect();

        // Sort by weighted score descending
        let mut sorted_suggestions = suggestions;
        sorted_suggestions.sort_by(|a, b| {
            b.weighted_score
                .partial_cmp(&a.weighted_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply probabilistic selection if requested
        if config.use_probabilistic && !sorted_suggestions.is_empty() {
            sorted_suggestions = self.apply_probabilistic_selection(
                sorted_suggestions,
                config.temperature,
                config.max_suggestions,
            );
        } else {
            // Take top suggestions
            sorted_suggestions.truncate(config.max_suggestions);
        }

        Ok(sorted_suggestions)
    }

    /// Check if chord should be avoided due to repetition rules
    fn should_avoid_chord(&self, chord: &Chord, context: &SuggestionContext) -> bool {
        if context.recent_chords.len() < context.avoid_repetition_within {
            return false;
        }

        let recent_window =
            &context.recent_chords[context.recent_chords.len() - context.avoid_repetition_within..];

        recent_window
            .iter()
            .any(|recent| recent.root == chord.root && recent.chord_type == chord.chord_type)
    }

    /// Calculate frequency-based score from pattern results
    fn calculate_frequency_score(&self, result: &PatternResult) -> f64 {
        // Normalize frequency score
        let max_count = 1000.0; // Assumed maximum count for normalization
        (result.count as f64 / max_count).min(1.0)
    }

    /// Calculate context-aware relevance score
    fn calculate_context_score(&self, chord: &Chord, context: &SuggestionContext) -> f64 {
        let mut score = 0.5; // Base score

        // Scale compatibility
        if let Some(ref scale) = context.scale_fingerprint {
            score += self.calculate_scale_compatibility(chord, scale) * 0.3;
        }

        // Position-based weighting
        score += self.calculate_position_score(chord, context.position_in_progression) * 0.2;

        // Valence matching
        score += self.calculate_valence_score(chord, context.target_valence) * 0.2;

        // Complexity matching
        score += self.calculate_complexity_score(chord, context.complexity_preference) * 0.1;

        // Genre weighting
        if !context.genre_weights.is_empty() {
            score += self.calculate_genre_score(chord, &context.genre_weights) * 0.2;
        }

        score.max(0.0).min(1.0)
    }

    /// Calculate theoretical appropriateness score
    fn calculate_theory_score(
        &self,
        chord: &Chord,
        pattern: &[Chord],
        _context: &SuggestionContext,
    ) -> f64 {
        let mut score = 0.5;

        // Voice leading analysis
        if let Some(prev_chord) = pattern.last() {
            score += self.calculate_voice_leading_score(prev_chord, chord) * 0.4;
        }

        // Harmonic function analysis
        score += self.calculate_harmonic_function_score(chord, pattern) * 0.3;

        // Resolution tendency analysis
        score += self.calculate_resolution_score(chord, pattern) * 0.3;

        score.max(0.0).min(1.0)
    }

    /// Calculate final weighted score combining all factors
    fn calculate_weighted_score(
        &self,
        frequency_score: f64,
        context_score: f64,
        theory_score: f64,
        config: &SuggestionConfig,
    ) -> f64 {
        let frequency_weight = 0.4;
        let context_weight = if config.enable_context_weighting {
            0.35
        } else {
            0.0
        };
        let theory_weight = 0.25;

        frequency_score * frequency_weight
            + context_score * context_weight
            + theory_score * theory_weight
    }

    /// Calculate overall confidence in suggestion
    fn calculate_confidence(&self, weighted_score: f64, result: &PatternResult) -> f64 {
        let score_confidence = weighted_score;
        let frequency_confidence = (result.count as f64 / 100.0).min(1.0);

        (score_confidence + frequency_confidence) / 2.0
    }

    /// Generate comprehensive human-readable reasoning for suggestion
    fn generate_reasoning(
        &self,
        chord: &Chord,
        pattern: &[Chord],
        context: &SuggestionContext,
        frequency_score: f64,
        context_score: f64,
        theory_score: f64,
    ) -> String {
        let mut reasons = Vec::new();

        // Frequency-based reasoning
        if frequency_score > 0.8 {
            reasons.push("very commonly used in similar progressions".to_string());
        } else if frequency_score > 0.6 {
            reasons.push("frequently found in similar contexts".to_string());
        } else if frequency_score > 0.4 {
            reasons.push("occasionally used in this style".to_string());
        }

        // Context-based reasoning
        if context_score > 0.7 && context.scale_fingerprint.is_some() {
            reasons.push("fits excellently in the current key".to_string());
        } else if context_score > 0.5 && context.scale_fingerprint.is_some() {
            reasons.push("works well in the current key".to_string());
        }

        // Theory-based reasoning
        if theory_score > 0.7 && !pattern.is_empty() {
            if let Some(prev_chord) = pattern.last() {
                match (prev_chord.root, chord.root) {
                    (5, 1) => reasons.push("provides strong dominant-tonic resolution".to_string()),
                    (2, 5) => reasons.push("continues ii-V motion".to_string()),
                    (4, 5) => reasons.push("creates effective IV-V progression".to_string()),
                    _ => reasons.push("provides good voice leading".to_string()),
                }
            } else {
                reasons.push("harmonically appropriate".to_string());
            }
        }

        // Chord-specific reasoning
        match chord.chord_type {
            7 if chord.root == 5 => {
                reasons.push("dominant seventh adds harmonic tension".to_string());
            },
            7 => {
                reasons.push("seventh chord enriches the harmony".to_string());
            },
            9 | 11 | 13 => {
                reasons.push("extended harmony adds sophistication".to_string());
            },
            _ => {},
        }

        // Position-based reasoning
        if context.position_in_progression > 0.8 {
            match chord.root {
                1 => reasons.push("provides strong conclusive resolution".to_string()),
                5 => reasons.push("builds tension toward resolution".to_string()),
                _ => {},
            }
        } else if context.position_in_progression < 0.3 {
            match chord.root {
                1 => reasons.push("establishes tonal center".to_string()),
                4 => reasons.push("creates departure from tonic".to_string()),
                _ => {},
            }
        }

        if reasons.is_empty() {
            "reasonable harmonic choice".to_string()
        } else if reasons.len() == 1 {
            format!("Suggested because it {}", reasons[0])
        } else {
            let last = reasons.pop().unwrap();
            format!("Suggested because it {} and {}", reasons.join(", "), last)
        }
    }

    /// Apply probabilistic selection based on weighted scores
    fn apply_probabilistic_selection(
        &self,
        mut suggestions: Vec<ChordSuggestion>,
        temperature: f64,
        max_suggestions: usize,
    ) -> Vec<ChordSuggestion> {
        if suggestions.is_empty() || temperature <= 0.0 {
            suggestions.truncate(max_suggestions);
            return suggestions;
        }

        // Apply temperature scaling to scores
        let scaled_scores: Vec<f64> = suggestions
            .iter()
            .map(|s| (s.weighted_score / temperature).exp())
            .collect();

        let total_score: f64 = scaled_scores.iter().sum();

        // Convert to probabilities
        let probabilities: Vec<f64> = scaled_scores
            .iter()
            .map(|score| score / total_score)
            .collect();

        // Select suggestions based on probabilities
        // For simplicity, we'll use a weighted selection approach
        let mut selected = Vec::new();
        let mut used_indices = std::collections::HashSet::new();

        for _ in 0..max_suggestions.min(suggestions.len()) {
            if let Some(index) = self.weighted_random_selection(&probabilities, &used_indices) {
                selected.push(suggestions[index].clone());
                used_indices.insert(index);
            }
        }

        selected
    }

    /// Weighted random selection helper
    fn weighted_random_selection(
        &self,
        probabilities: &[f64],
        used_indices: &std::collections::HashSet<usize>,
    ) -> Option<usize> {
        // Simple deterministic selection for now - in practice would use proper RNG
        for (i, &prob) in probabilities.iter().enumerate() {
            if !used_indices.contains(&i) && prob > 0.1 {
                return Some(i);
            }
        }
        None
    }

    /// Helper scoring functions with proper implementations
    fn calculate_scale_compatibility(&self, chord: &Chord, scale: &ScaleFingerprint) -> f64 {
        // Use stable scale degrees to determine chord-scale compatibility
        match composer_core::get_stable_scale_degrees(chord, scale) {
            Ok(degrees) => {
                // More scale degrees in the chord = better compatibility
                let compatibility_base = (degrees.len() as f64 / 7.0).min(1.0);

                // Check for chromatic notes (scale degrees outside the scale)
                let chromatic_penalty = degrees
                    .iter()
                    .filter(|d| d.contains('#') || d.contains('b'))
                    .count() as f64
                    * 0.1;

                (compatibility_base - chromatic_penalty).max(0.0).min(1.0)
            },
            Err(_) => 0.3, // Default moderate compatibility if analysis fails
        }
    }

    fn calculate_position_score(&self, chord: &Chord, position: f64) -> f64 {
        // Position-based weighting: certain chords work better at different positions
        let base_score: f64 = match chord.chord_type {
            5 => {
                // Triads
                if position < 0.25 {
                    0.8 // Strong chords work well at beginning
                } else if position > 0.75 {
                    0.9 // Triads provide good resolution at end
                } else {
                    0.6 // Moderate in middle
                }
            },
            7 => {
                // Seventh chords
                if position > 0.5 {
                    0.8 // Seventh chords create good tension toward end
                } else {
                    0.5 // Less ideal at beginning
                }
            },
            _ => 0.5, // Extended chords - neutral
        };

        // Adjust for chord root (tonic works well at start/end)
        let root_adjustment: f64 = if chord.root == 1 {
            if !(0.2..=0.8).contains(&position) {
                0.2 // Bonus for tonic at beginning/end
            } else {
                0.0
            }
        } else {
            0.0
        };

        (base_score + root_adjustment).min(1.0_f64)
    }

    fn calculate_valence_score(&self, chord: &Chord, target_valence: f64) -> f64 {
        // Map chord types to emotional valence (-1.0 to 1.0)
        let chord_valence = match chord.chord_type {
            5 => {
                // Triads
                match chord.root {
                    1 | 4 | 5 => 0.6,  // Major triads: positive
                    2 | 3 | 6 => -0.2, // Minor triads: slightly negative
                    7 => -0.4,         // Diminished: negative
                    _ => 0.0,
                }
            },
            7 => {
                // Seventh chords
                match chord.root {
                    1 => 0.4,      // Major 7: warm positive
                    5 => 0.1,      // Dominant 7: tension (neutral)
                    2 | 6 => -0.1, // Minor 7: mild negative
                    _ => 0.0,
                }
            },
            _ => 0.0, // Extended chords: neutral
        };

        // Adjust for alterations (generally more dissonant)
        let alteration_adjustment = -0.1 * chord.alterations.len() as f64;
        let final_valence = (chord_valence + alteration_adjustment).max(-1.0).min(1.0);

        // Score based on how close to target valence
        let distance = (final_valence - target_valence).abs();
        (1.0 - distance / 2.0).max(0.0)
    }

    fn calculate_complexity_score(&self, chord: &Chord, preference: f64) -> f64 {
        // Calculate chord complexity (0.0 = simple, 1.0 = very complex)
        let base_complexity = match chord.chord_type {
            5 => 0.1,  // Triads are simple
            7 => 0.3,  // Sevenths are moderate
            9 => 0.5,  // Ninths are complex
            11 => 0.7, // Elevenths are very complex
            13 => 0.9, // Thirteenths are extremely complex
            _ => 0.5,
        };

        // Add complexity for alterations
        let alteration_complexity = chord.alterations.len() as f64 * 0.1;

        // Add complexity for suspensions, adds, omissions
        let modification_complexity =
            (chord.suspensions.len() + chord.adds.len() + chord.omits.len()) as f64 * 0.05;

        let total_complexity =
            (base_complexity + alteration_complexity + modification_complexity).min(1.0);

        // Score based on how well complexity matches preference
        let distance = (total_complexity - preference).abs();
        (1.0 - distance).max(0.0)
    }

    fn calculate_genre_score(&self, chord: &Chord, genre_weights: &HashMap<String, f64>) -> f64 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for (genre, weight) in genre_weights {
            if *weight > 0.0 {
                let genre_score = match genre.to_lowercase().as_str() {
                    "pop" => {
                        // Pop favors simple triads and basic sevenths
                        match chord.chord_type {
                            5 => 0.9,
                            7 => 0.6,
                            _ => 0.3,
                        }
                    },
                    "jazz" => {
                        // Jazz favors complex harmonies
                        (match chord.chord_type {
                            5 => 0.4,
                            7 => 0.8,
                            9 | 11 | 13 => 0.9,
                            _ => 0.5,
                        }) + (chord.alterations.len() as f64 * 0.1)
                    },
                    "classical" => {
                        // Classical favors traditional voice leading
                        (match chord.chord_type {
                            5 => 0.8,
                            7 => 0.7,
                            _ => 0.5,
                        }) - (chord.alterations.len() as f64 * 0.05)
                    },
                    "blues" => {
                        // Blues favors sevenths and dominant chords
                        let base = match chord.chord_type {
                            7 => 0.9,
                            5 => 0.6,
                            _ => 0.4,
                        };
                        if chord.root == 5 {
                            base + 0.1
                        } else {
                            base
                        } // Bonus for dominant
                    },
                    "rock" => {
                        // Rock favors power chords and simple progressions
                        match chord.chord_type {
                            5 => 0.8,
                            7 => 0.5,
                            _ => 0.3,
                        }
                    },
                    _ => 0.5, // Unknown genre
                };

                total_score += genre_score * weight;
                total_weight += weight;
            }
        }

        if total_weight > 0.0 {
            (total_score / total_weight).max(0.0).min(1.0)
        } else {
            0.5 // Neutral if no genre weights
        }
    }

    fn calculate_voice_leading_score(&self, prev_chord: &Chord, chord: &Chord) -> f64 {
        // Calculate voice leading quality between two chords
        if prev_chord.is_rest || chord.is_rest {
            return 0.5; // Neutral for rests
        }

        // Calculate root movement (in semitones)
        let root_movement = ((chord.root as i8 - prev_chord.root as i8) % 7).abs();

        // Score based on root movement (smaller movements generally better)
        let root_score: f64 = match root_movement {
            0 => 0.3, // Same root - static
            1 => 0.9, // Step movement - excellent
            2 => 0.8, // Second - very good
            3 => 0.6, // Third - good
            4 => 0.7, // Fourth/Fifth - good (circle of fifths)
            5 => 0.5, // Sixth - moderate
            6 => 0.4, // Seventh - more distant
            _ => 0.3,
        };

        // Bonus for common voice leading patterns
        let pattern_bonus: f64 = match (prev_chord.root, chord.root) {
            (5, 1) => 0.2, // V-I resolution
            (4, 5) => 0.1, // IV-V movement
            (2, 5) => 0.2, // ii-V movement
            (1, 6) => 0.1, // I-vi deceptive
            _ => 0.0,
        };

        // Consider chord type compatibility
        let type_score: f64 = if prev_chord.chord_type == chord.chord_type {
            0.1 // Slight bonus for same chord type
        } else if prev_chord.chord_type > chord.chord_type {
            0.05 // Slight bonus for complexity reduction
        } else {
            0.0
        };

        (root_score + pattern_bonus + type_score).min(1.0_f64)
    }

    fn calculate_harmonic_function_score(&self, chord: &Chord, pattern: &[Chord]) -> f64 {
        if pattern.is_empty() {
            return 0.5; // Neutral if no context
        }

        // Analyze harmonic function based on scale degree
        let function_strength: f64 = match chord.root {
            1 => 0.9, // Tonic - very strong
            5 => 0.8, // Dominant - strong
            4 => 0.7, // Subdominant - strong
            2 => 0.6, // Supertonic - moderate (often ii in ii-V-I)
            6 => 0.6, // Submediant - moderate (relative minor)
            3 => 0.4, // Mediant - weaker
            7 => 0.5, // Leading tone - contextual
            _ => 0.3,
        };

        // Analyze context from previous chord
        let context_bonus: f64 = if let Some(prev_chord) = pattern.last() {
            match (prev_chord.root, chord.root) {
                // Strong functional progressions
                (5, 1) => 0.3, // V-I (dominant to tonic)
                (2, 5) => 0.2, // ii-V
                (4, 5) => 0.2, // IV-V
                (1, 4) => 0.1, // I-IV (tonic to subdominant)
                (1, 6) => 0.1, // I-vi (tonic to relative minor)
                (6, 4) => 0.1, // vi-IV
                // Avoid weak progressions
                (1, 2) => -0.1, // I-ii (weak)
                (1, 3) => -0.1, // I-iii (weak)
                _ => 0.0,
            }
        } else {
            0.0
        };

        // Consider chord type appropriateness for function
        let type_bonus: f64 = match (chord.root, chord.chord_type) {
            (5, 7) => 0.1,  // V7 - classic dominant
            (2, 7) => 0.1,  // ii7 - common
            (1, 5) => 0.05, // I triad - solid tonic
            (4, 5) => 0.05, // IV triad - solid subdominant
            _ => 0.0,
        };

        (function_strength + context_bonus + type_bonus)
            .max(0.0_f64)
            .min(1.0_f64)
    }

    fn calculate_resolution_score(&self, chord: &Chord, pattern: &[Chord]) -> f64 {
        if pattern.is_empty() {
            return 0.5; // Neutral if no context
        }

        // Analyze resolution tendencies based on the current chord
        let resolution_tendency = match chord.root {
            5 => {
                // Dominant - strong tendency to resolve to tonic
                if chord.chord_type == 7 {
                    0.9 // V7 has very strong resolution tendency
                } else {
                    0.7 // V has strong resolution tendency
                }
            },
            7 => 0.8, // Leading tone - strong upward resolution tendency
            2 => 0.6, // Supertonic - moderate tendency (often moves to V)
            4 => 0.4, // Subdominant - mild tendency (can go many places)
            6 => 0.3, // Submediant - weak tendency
            3 => 0.3, // Mediant - weak tendency
            1 => 0.1, // Tonic - very stable, little resolution tendency
            _ => 0.3,
        };

        // Check if we're in a resolution context
        let context_factor = if let Some(prev_chord) = pattern.last() {
            match prev_chord.root {
                5 => 1.2, // After dominant, resolution is more important
                7 => 1.1, // After leading tone
                2 => 1.1, // After supertonic (in ii-V context)
                _ => 1.0,
            }
        } else {
            1.0
        };

        // Seventh chords generally have stronger resolution tendencies
        let chord_type_factor = match chord.chord_type {
            7 => 1.1,
            9 | 11 | 13 => 1.2, // Extended chords have even stronger tendencies
            _ => 1.0,
        };

        // Alterations typically increase resolution tendency
        let alteration_factor = 1.0 + (chord.alterations.len() as f64 * 0.05);

        (resolution_tendency * context_factor * chord_type_factor * alteration_factor).min(1.0)
    }

    /// Generate cache key for suggestion caching
    fn generate_cache_key(
        &self,
        pattern: &[Chord],
        context: &SuggestionContext,
        config: &SuggestionConfig,
    ) -> String {
        // Simplified cache key - in practice would be more sophisticated
        format!(
            "{}_{:.2}_{:.2}_{}",
            pattern.len(),
            context.position_in_progression,
            context.target_valence,
            config.max_suggestions
        )
    }

    /// Update performance metrics
    fn update_performance_metrics(&self, elapsed_ms: f64) {
        let mut avg = self.avg_suggestion_time_ms.write();
        *avg = (*avg * 0.9) + (elapsed_ms * 0.1); // Exponential moving average
    }

    /// Get average suggestion time
    pub fn avg_suggestion_time_ms(&self) -> f64 {
        *self.avg_suggestion_time_ms.read()
    }

    /// Clear suggestion cache
    pub fn clear_cache(&self) {
        self.suggestion_cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (
            self.suggestion_cache.len(),
            self.suggestion_cache.capacity(),
        )
    }

    /// Apply statistical weighting algorithm from test specifications (lines 150-176)
    fn apply_magic_chord_weighting(
        &self,
        pattern_results: Vec<PatternResult>,
        prev_length: usize,
        next_length: usize,
        total_length: usize,
        limit: usize,
    ) -> AiResult<Vec<ChordSuggestion>> {
        let suggestions: Vec<ChordSuggestion> = pattern_results
            .par_iter()
            .filter_map(|result| {
                // Deserialize the chord from binary
                let chord =
                    match composer_serialization::deserialize_chord(&result.serialized_chord) {
                        Ok(chord) => chord,
                        Err(_) => return None,
                    };

                // Calculate weight using algorithm from test specification
                let weight = self.compute_weight_from_spec(
                    prev_length,
                    next_length,
                    total_length,
                    result.count as u64,
                    result.relative_count,
                );

                // Filter by minimum weight threshold
                if weight < QUALITY.confidence_threshold {
                    return None;
                }

                // Generate reasoning explanation
                let reasoning = self.generate_magic_chord_reasoning(weight, result);

                Some(ChordSuggestion {
                    chord,
                    confidence: weight,
                    frequency_score: result.relative_count,
                    context_score: weight,
                    theory_score: 0.5, // Simplified for magic chord algorithm
                    weighted_score: weight,
                    pattern_info: result.clone(),
                    reasoning,
                })
            })
            .collect();

        // Sort by weighted score descending
        let mut sorted_suggestions = suggestions;
        sorted_suggestions.sort_by(|a, b| {
            b.weighted_score
                .partial_cmp(&a.weighted_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply deduplication and limit
        sorted_suggestions.truncate(limit);
        Ok(sorted_suggestions)
    }

    /// Compute weight using exact algorithm from test specification (lines 150-176)
    pub fn compute_weight_from_spec(
        &self,
        prev_length: usize,
        next_length: usize,
        total_length: usize,
        _id_count: u64,
        relative_count: f64,
    ) -> f64 {
        // Algorithm from test spec lines 161-172:
        // contextLength = prevLength + nextLength
        let context_length = prev_length + next_length;

        // contextMatch = totalLength == 0 ? 1 : 1 - (totalLength - contextLength) / totalLength
        let context_match = if total_length == 0 {
            1.0
        } else {
            1.0 - (total_length - context_length) as f64 / total_length as f64
        };

        // contextBonus = contextLength × (nextLength > prevLength ? 1.7 : 1.0)
        let context_bonus =
            context_length as f64 * if next_length > prev_length { 1.7 } else { 1.0 };

        // statisticalStrength = min((relativeCount × contextBonus) / 10000, 1.0)
        let statistical_strength = ((relative_count * context_bonus) / 10000.0).min(1.0);

        // finalWeight = contextMatch × statisticalStrength
        context_match * statistical_strength
    }

    /// Generate reasoning for magic chord suggestions using the unified system
    fn generate_magic_chord_reasoning(&self, weight: f64, result: &PatternResult) -> String {
        let mut reasons = Vec::new();

        // Statistical strength reasoning
        if weight > 0.8 {
            reasons.push("exceptionally strong statistical match".to_string());
        } else if weight > 0.6 {
            reasons.push("strong statistical match with context".to_string());
        } else if weight > 0.4 {
            reasons.push("good statistical support".to_string());
        } else if weight > 0.2 {
            reasons.push("moderate statistical evidence".to_string());
        } else {
            reasons.push("limited statistical support".to_string());
        }

        // Frequency-based reasoning
        if result.count > 500 {
            reasons.push("very frequently used in similar progressions".to_string());
        } else if result.count > 100 {
            reasons.push("commonly found in this context".to_string());
        } else if result.count > 20 {
            reasons.push("occasionally used in similar situations".to_string());
        } else if result.count > 5 {
            reasons.push("rarely but consistently appears".to_string());
        }

        // Ranking-based reasoning
        if result.rank <= 3 {
            reasons.push("among top choices for this pattern".to_string());
        } else if result.rank <= 10 {
            reasons.push("popular choice for this context".to_string());
        }

        // Relative frequency reasoning
        if result.relative_count > 0.1 {
            reasons.push("represents significant portion of pattern usage".to_string());
        } else if result.relative_count > 0.05 {
            reasons.push("notable frequency in this pattern type".to_string());
        }

        if reasons.len() <= 1 {
            reasons
                .into_iter()
                .next()
                .unwrap_or("suggested by algorithm".to_string())
        } else {
            let last = reasons.pop().unwrap();
            format!("{} and {}", reasons.join(", "), last)
        }
    }

    /// Bass line harmonization algorithm from test specification
    pub fn get_magic_bass_solutions(
        &self,
        bass_note: &str,
        _scale: &str,
        limit: usize,
    ) -> AiResult<Vec<ChordSuggestion>> {
        let start_time = Instant::now();

        // Convert bass note to raw chromatic value (0-11)
        let chromatic_bass = self.parse_bass_note(bass_note)?;

        // Query bass progression database for matching chords
        // For now, use a simplified approach with common chord types that contain the bass note
        let mut bass_suggestions = Vec::new();

        // Generate common chord types with this bass note as root
        for chord_type in [5, 7, 9] {
            // Triad, seventh, ninth
            if let Ok(chord) = Chord::new(chromatic_bass + 1, chord_type) {
                // Convert to scale degree
                let complexity = self.calculate_bass_chord_complexity(&chord);
                let frequency = self.estimate_bass_chord_frequency(chromatic_bass, chord_type);

                // Apply frequency-complexity weighting from test spec
                let weight = frequency * (1.0 / (1.0 + complexity * 0.1));

                if weight > QUALITY.confidence_threshold {
                    bass_suggestions.push(ChordSuggestion {
                        chord,
                        confidence: weight,
                        frequency_score: frequency,
                        context_score: 0.5,
                        theory_score: 1.0 - complexity / 10.0,
                        weighted_score: weight,
                        pattern_info: PatternResult {
                            serialized_chord: [0; 5], // Simplified
                            count: (frequency * 1000.0) as u32,
                            rank: 1,
                            relative_count: frequency,
                            id_list: vec![],
                            weight,
                        },
                        reasoning: format!("Bass note {} harmonization", bass_note),
                    });
                }
            }
        }

        // Sort by weight and limit
        bass_suggestions.sort_by(|a, b| {
            b.weighted_score
                .partial_cmp(&a.weighted_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        bass_suggestions.truncate(limit);

        // Performance check
        let elapsed_ms = start_time.elapsed().as_millis() as f64;
        self.update_performance_metrics(elapsed_ms);

        Ok(bass_suggestions)
    }

    /// Scale degree harmonization algorithm from test specification
    pub fn get_harmonize_by_sd_solutions(
        &self,
        scale_degree_bits: u32,
        _scale: &str,
        limit: usize,
    ) -> AiResult<Vec<ChordSuggestion>> {
        let start_time = Instant::now();

        // Convert bit mask to scale degree array (test spec lines 319-322)
        let scale_degrees = self.bits_to_scale_degrees(scale_degree_bits);

        if scale_degrees.is_empty() {
            return Err(AiError::InvalidPattern {
                reason: "No scale degrees specified".to_string(),
            });
        }

        // Search harmonization database for matching chords
        let mut harmonizations = Vec::new();

        // Try different chord types that can harmonize these scale degrees
        for chord_type in [5, 7, 9, 11, 13] {
            for root in 1..=7 {
                if let Ok(chord) = Chord::new(root, chord_type) {
                    if self.chord_contains_scale_degrees(&chord, &scale_degrees) {
                        // For single scale degree, only allow chords with that root
                        if scale_degrees.len() == 1 && scale_degrees[0] != root {
                            continue; // Skip chords that don't have the target root
                        }
                        // Apply multi-factor scoring from test spec (lines 324-334)
                        let lookup_score = self.calculate_lookup_score(&chord);
                        let length_score = self.calculate_length_score(&chord);
                        let complexity_score = self.calculate_complexity_score_for_sd(&chord);
                        let magic_score = self.calculate_magic_score(&chord);

                        // Total score formula from test spec
                        let total_score = lookup_score * 0.2
                            + length_score * 0.2
                            + complexity_score * 0.3
                            + magic_score * 0.3;

                        if total_score > QUALITY.confidence_threshold {
                            harmonizations.push(ChordSuggestion {
                                chord,
                                confidence: total_score,
                                frequency_score: magic_score,
                                context_score: 0.5,
                                theory_score: complexity_score,
                                weighted_score: total_score,
                                pattern_info: PatternResult {
                                    serialized_chord: [0; 5], // Simplified
                                    count: (magic_score * 100.0) as u32,
                                    rank: 1,
                                    relative_count: magic_score,
                                    id_list: vec![],
                                    weight: total_score,
                                },
                                reasoning: format!("Harmonizes scale degrees {:?}", scale_degrees),
                            });
                        }
                    }
                }
            }
        }

        // Sort by total score and limit
        harmonizations.sort_by(|a, b| {
            b.weighted_score
                .partial_cmp(&a.weighted_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        harmonizations.truncate(limit);

        // Performance check
        let elapsed_ms = start_time.elapsed().as_millis() as f64;
        self.update_performance_metrics(elapsed_ms);

        Ok(harmonizations)
    }

    // Helper methods for the new algorithms

    pub fn parse_bass_note(&self, bass_note: &str) -> AiResult<u8> {
        // Simple note parsing (could be enhanced)
        match bass_note.to_uppercase().as_str() {
            "C" => Ok(0),
            "C#" | "DB" => Ok(1),
            "D" => Ok(2),
            "D#" | "EB" => Ok(3),
            "E" => Ok(4),
            "F" => Ok(5),
            "F#" | "GB" => Ok(6),
            "G" => Ok(7),
            "G#" | "AB" => Ok(8),
            "A" => Ok(9),
            "A#" | "BB" => Ok(10),
            "B" => Ok(11),
            _ => Err(AiError::InvalidPattern {
                reason: format!("Invalid bass note: {}", bass_note),
            }),
        }
    }

    fn calculate_bass_chord_complexity(&self, chord: &Chord) -> f64 {
        let mut complexity = 1.0;

        // Extended chords are more complex
        if chord.chord_type >= 9 {
            complexity += 1.0;
        }

        // Alterations add complexity
        complexity += chord.alterations.len() as f64 * 0.5;

        complexity.min(10.0)
    }

    fn estimate_bass_chord_frequency(&self, _bass_note: u8, chord_type: u8) -> f64 {
        // Simplified frequency estimation
        match chord_type {
            5 => 0.8, // Triads are common
            7 => 0.6, // Sevenths moderately common
            9 => 0.3, // Ninths less common
            _ => 0.1,
        }
    }

    pub fn bits_to_scale_degrees(&self, bits: u32) -> Vec<u8> {
        let mut degrees = Vec::new();
        for i in 0..13 {
            if (bits & (1 << i)) != 0 {
                degrees.push((i + 1) as u8);
            }
        }
        degrees
    }

    fn chord_contains_scale_degrees(&self, chord: &Chord, scale_degrees: &[u8]) -> bool {
        if scale_degrees.is_empty() {
            return false;
        }

        // Get the chord tones based on chord type
        let chord_tones = match chord.chord_type {
            5 => vec![
                chord.root,
                ((chord.root + 2 - 1) % 7) + 1,
                ((chord.root + 4 - 1) % 7) + 1,
            ], // 1, 3, 5
            7 => vec![
                chord.root,
                ((chord.root + 2 - 1) % 7) + 1,
                ((chord.root + 4 - 1) % 7) + 1,
                ((chord.root + 6 - 1) % 7) + 1,
            ], // 1, 3, 5, 7
            9 => vec![
                chord.root,
                ((chord.root + 2 - 1) % 7) + 1,
                ((chord.root + 4 - 1) % 7) + 1,
                ((chord.root + 6 - 1) % 7) + 1,
                ((chord.root + 1 - 1) % 7) + 1,
            ], // 1, 3, 5, 7, 9
            _ => vec![chord.root], // Basic fallback
        };

        // Check if any chord tones match the required scale degrees
        chord_tones.iter().any(|tone| scale_degrees.contains(tone))
    }

    fn calculate_lookup_score(&self, chord: &Chord) -> f64 {
        // Score based on chord complexity (0-10 scale, normalized to 0-1)
        let complexity = match chord.chord_type {
            5 => 1.0,  // Triads are simple
            7 => 3.0,  // Sevenths are moderate
            9 => 5.0,  // Ninths are complex
            11 => 7.0, // Elevenths are very complex
            13 => 9.0, // Thirteenths are extremely complex
            _ => 5.0,
        };

        // Add complexity for modifications
        let modification_complexity = chord.alterations.len() as f64 * 0.5
            + chord.suspensions.len() as f64 * 0.3
            + chord.adds.len() as f64 * 0.2
            + chord.omits.len() as f64 * 0.4;

        let total_complexity = complexity + modification_complexity;

        // Invert score (lower complexity = higher lookup score)
        ((10.0 - total_complexity) / 10.0).max(0.0).min(1.0)
    }

    fn calculate_length_score(&self, chord: &Chord) -> f64 {
        // Calculate estimated note count in chord
        let base_notes = match chord.chord_type {
            5 => 3,  // Triad: 3 notes
            7 => 4,  // Seventh: 4 notes
            9 => 5,  // Ninth: 5 notes
            11 => 6, // Eleventh: 6 notes
            13 => 7, // Thirteenth: 7 notes
            _ => 3,
        };

        let additional_notes = chord.adds.len() + chord.suspensions.len() - chord.omits.len(); // Omissions reduce note count

        let total_notes = (base_notes + additional_notes as i32).max(1) as usize;

        // Score based on note count (penalize excessive notes)
        match total_notes {
            1..=4 => 1.0, // Ideal range
            5 => 0.9,     // Still good
            6 => 0.7,     // Getting complex
            7 => 0.5,     // Complex
            8 => 0.3,     // Very complex
            _ => 0.1,     // Excessively complex
        }
    }

    fn calculate_complexity_score_for_sd(&self, chord: &Chord) -> f64 {
        let base_complexity = match chord.chord_type {
            5 => 1.0,  // Triad
            7 => 2.0,  // Seventh
            9 => 3.0,  // Ninth
            11 => 4.0, // Eleventh
            13 => 5.0, // Thirteenth
            _ => 6.0,
        };

        // Add complexity for chord modifications
        let alteration_complexity = chord.alterations.len() as f64 * 0.5;
        let suspension_complexity = chord.suspensions.len() as f64 * 0.3;
        let add_complexity = chord.adds.len() as f64 * 0.2;

        // Account for applied chords and borrowed chords
        let applied_complexity = if chord.applied > 0 { 1.0 } else { 0.0 };
        let borrowed_complexity = if chord.borrowed.is_some() { 1.5 } else { 0.0 };

        let total_complexity = base_complexity
            + alteration_complexity
            + suspension_complexity
            + add_complexity
            + applied_complexity
            + borrowed_complexity;

        // Invert and normalize so higher complexity = lower score
        let normalized = (10.0 - total_complexity) / 10.0;
        normalized.max(0.0).min(1.0)
    }

    fn calculate_magic_score(&self, chord: &Chord) -> f64 {
        // Statistical popularity scoring based on common usage patterns
        let base_popularity = match chord.chord_type {
            5 => 0.9,  // Triads very popular
            7 => 0.7,  // Sevenths popular
            9 => 0.5,  // Ninths moderately popular
            11 => 0.3, // Elevenths less popular
            13 => 0.2, // Thirteenths rare
            _ => 0.1,
        };

        // Adjust for chord root popularity (some roots more common)
        let root_popularity = match chord.root {
            1 => 1.0, // Tonic very common
            5 => 0.9, // Dominant very common
            4 => 0.8, // Subdominant common
            6 => 0.7, // Submediant fairly common
            2 => 0.6, // Supertonic moderate
            3 => 0.4, // Mediant less common
            7 => 0.5, // Leading tone moderate (context dependent)
            _ => 0.3,
        };

        // Penalty for complex alterations and modifications
        let complexity_penalty = chord.alterations.len() as f64 * 0.05
            + chord.suspensions.len() as f64 * 0.02
            + chord.adds.len() as f64 * 0.02;

        // Special bonuses for very common chord types
        let special_bonus = match (chord.root, chord.chord_type) {
            (1, 5) => 0.1,  // I chord
            (5, 7) => 0.1,  // V7 chord
            (2, 7) => 0.05, // ii7 chord
            (4, 5) => 0.05, // IV chord
            _ => 0.0,
        };

        ((base_popularity * root_popularity) - complexity_penalty + special_bonus)
            .max(0.0)
            .min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use composer_core::Chord;

    #[test]
    fn test_suggestion_context_default() {
        let context = SuggestionContext::default();
        assert_eq!(context.position_in_progression, 0.5);
        assert_eq!(context.target_valence, 0.0);
        assert_eq!(context.complexity_preference, 0.5);
        assert_eq!(context.avoid_repetition_within, 4);
    }

    #[test]
    fn test_suggestion_config_default() {
        let config = SuggestionConfig::default();
        assert!(config.max_suggestions > 0);
        assert!(config.min_confidence >= 0.0 && config.min_confidence <= 1.0);
        assert_eq!(config.temperature, 1.0);
    }

    #[test]
    fn test_suggester_creation() {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = ChordProgressionSuggester::new(trie);
        assert_eq!(suggester.avg_suggestion_time_ms(), 0.0);
    }

    #[test]
    fn test_empty_pattern_error() {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = ChordProgressionSuggester::new(trie);
        let context = SuggestionContext::default();
        let config = SuggestionConfig::default();

        let result = suggester.get_chord_suggestions(&[], &context, &config);
        assert!(result.is_err());

        if let Err(AiError::InvalidPattern { reason }) = result {
            assert!(reason.contains("empty"));
        } else {
            panic!("Expected InvalidPattern error");
        }
    }

    #[test]
    fn test_repetition_avoidance() {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = ChordProgressionSuggester::new(trie);

        let chord = Chord::new(1, 5).unwrap();
        let mut context = SuggestionContext::default();
        context.recent_chords.push(chord.clone());
        context.avoid_repetition_within = 1;

        assert!(suggester.should_avoid_chord(&chord, &context));
    }

    #[test]
    fn test_scoring_functions() {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = ChordProgressionSuggester::new(trie);

        let pattern_result = PatternResult {
            serialized_chord: [0; 5],
            count: 50,
            rank: 1,
            relative_count: 0.5,
            id_list: vec![],
            weight: 0.8,
        };

        let frequency_score = suggester.calculate_frequency_score(&pattern_result);
        assert!((0.0..=1.0).contains(&frequency_score));

        let context = SuggestionContext::default();
        let chord = Chord::new(1, 5).unwrap();
        let context_score = suggester.calculate_context_score(&chord, &context);
        assert!((0.0..=1.0).contains(&context_score));
    }

    #[test]
    fn test_cache_operations() {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = ChordProgressionSuggester::new(trie);

        let (len, _cap) = suggester.cache_stats();
        assert_eq!(len, 0);

        suggester.clear_cache();
        let (len, _cap) = suggester.cache_stats();
        assert_eq!(len, 0);
    }

    #[test]
    fn test_weighted_score_calculation() {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = ChordProgressionSuggester::new(trie);
        let config = SuggestionConfig::default();

        let score = suggester.calculate_weighted_score(0.8, 0.6, 0.7, &config);
        assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn test_confidence_calculation() {
        let trie = Arc::new(ChordProgressionTrie::new());
        let suggester = ChordProgressionSuggester::new(trie);

        let pattern_result = PatternResult {
            serialized_chord: [0; 5],
            count: 25,
            rank: 2,
            relative_count: 0.25,
            id_list: vec![],
            weight: 0.6,
        };

        let confidence = suggester.calculate_confidence(0.7, &pattern_result);
        assert!((0.0..=1.0).contains(&confidence));
    }
}
