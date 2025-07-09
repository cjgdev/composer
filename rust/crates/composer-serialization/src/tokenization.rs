//! Musical tokenization for ML applications

use crate::constants::*;
use crate::error::{SerializationError, SerializationResult};
use composer_core::{Chord, ScaleFingerprint};
use regex::Regex;
use std::collections::HashMap;

/// Note structure for tokenization
#[derive(Debug, Clone)]
pub struct Note {
    pub scale_degree: u8,
    pub octave: u8,
    pub is_rest: bool,
}

/// Token event for timeline reconstruction
#[derive(Debug, Clone)]
pub struct TokenEvent {
    pub beat: f64,
    pub event_type: TokenEventType,
}

#[derive(Debug, Clone)]
pub enum TokenEventType {
    Note(Note),
    Chord(Chord),
    Rest,
}

/// Musical timeline structure
#[derive(Debug, Clone)]
pub struct Timeline {
    pub events: Vec<TokenEvent>,
    pub total_duration: f64,
}

/// Token library for ML consistency
#[derive(Debug, Clone)]
pub struct TokenLibrary {
    pub chord_tokens: HashMap<String, Vec<u8>>, // token -> serialized chord
    pub library_size: usize,
}

impl TokenLibrary {
    pub fn new() -> Self {
        Self {
            chord_tokens: HashMap::new(),
            library_size: 0,
        }
    }

    pub fn add_chord_token(&mut self, token: String, chord_binary: Vec<u8>) {
        self.chord_tokens.insert(token, chord_binary);
        self.library_size = self.chord_tokens.len();
    }

    pub fn resolve_chord_token(&self, token: &str) -> SerializationResult<Vec<u8>> {
        self.chord_tokens
            .get(token)
            .cloned()
            .ok_or_else(|| SerializationError::InvalidFormat {
                message: format!("Token not found in library: {}", token),
            })
    }

    pub fn update_library(&mut self, tokens: Vec<String>, chord_binaries: Vec<Vec<u8>>) {
        for (token, binary) in tokens.into_iter().zip(chord_binaries.into_iter()) {
            self.add_chord_token(token, binary);
        }
    }

    pub fn get_library_size(&self) -> usize {
        self.library_size
    }
}

impl Default for TokenLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Duration tokenization
pub fn tokenize_duration(duration: f64) -> String {
    let ticks = (duration * TICKS_PER_BEAT as f64).round() as u32;
    format!("{}{:x}", DURATION_TOKEN_PREFIX, ticks)
}

/// Parse duration from token
pub fn parse_duration_token(token: &str) -> SerializationResult<f64> {
    if !token.starts_with(DURATION_TOKEN_PREFIX) {
        return Err(SerializationError::InvalidFormat {
            message: "Invalid duration token prefix".to_string(),
        });
    }

    let hex_part = &token[DURATION_TOKEN_PREFIX.len()..];
    let ticks =
        u32::from_str_radix(hex_part, 16).map_err(|_| SerializationError::InvalidFormat {
            message: "Invalid hexadecimal in duration token".to_string(),
        })?;

    Ok(ticks as f64 / TICKS_PER_BEAT as f64)
}

/// Note tokenization as raw chromatic values
pub fn tokenize_note_as_raw(note: &Note, _scale: &ScaleFingerprint) -> SerializationResult<String> {
    if note.is_rest {
        return Ok(REST_NOTE_TOKEN.to_string());
    }

    // Simplified MIDI conversion for demo (in practice would use scale context)
    let midi_value = note.octave as u16 * 12 + note.scale_degree as u16;
    let chromatic = (midi_value + 12) % 12;
    let octave = ((midi_value / 12) as u8).clamp(OCTAVE_RANGE_MIN, OCTAVE_RANGE_MAX);

    Ok(format!(
        "{}{:x} {}{:x}",
        RAW_NOTE_TOKEN_PREFIX, chromatic, OCTAVE_TOKEN_PREFIX, octave
    ))
}

/// Chord tokenization as raw chromatic cluster
pub fn tokenize_chord_as_raw(
    chord: &Chord,
    scale: &ScaleFingerprint,
) -> SerializationResult<String> {
    // Get stable scale degrees for the chord
    let scale_degrees = get_stable_scale_degrees(chord, scale)?;

    // Convert to raw chromatic values
    let chromatic_values: Vec<u8> = scale_degrees
        .iter()
        .map(|&degree| scale_degree_to_raw(degree))
        .collect();

    // Format as hyphen-separated tokens
    let tokens: Vec<String> = chromatic_values
        .iter()
        .map(|&value| format!("{}{:x}", RAW_NOTE_TOKEN_PREFIX, value))
        .collect();

    Ok(tokens.join("-"))
}

/// Extract stable scale degrees from chord
fn get_stable_scale_degrees(
    chord: &Chord,
    _scale: &ScaleFingerprint,
) -> SerializationResult<Vec<u8>> {
    let mut degrees = Vec::new();

    // Root
    degrees.push(chord.root);

    // Basic triad intervals based on chord type
    match chord.chord_type {
        5 => {
            // Triad
            degrees.push(((chord.root - 1 + 2) % 7) + 1); // Third
            degrees.push(((chord.root - 1 + 4) % 7) + 1); // Fifth
        },
        7 => {
            // Seventh
            degrees.push(((chord.root - 1 + 2) % 7) + 1); // Third
            degrees.push(((chord.root - 1 + 4) % 7) + 1); // Fifth
            degrees.push(((chord.root - 1 + 6) % 7) + 1); // Seventh
        },
        9 => {
            // Ninth
            degrees.push(((chord.root - 1 + 2) % 7) + 1); // Third
            degrees.push(((chord.root - 1 + 4) % 7) + 1); // Fifth
            degrees.push(((chord.root - 1 + 6) % 7) + 1); // Seventh
            degrees.push(((chord.root - 1 + 1) % 7) + 1); // Ninth (9th = 2nd in next octave)
        },
        _ => {
            degrees.push(((chord.root - 1 + 2) % 7) + 1); // Default to triad
            degrees.push(((chord.root - 1 + 4) % 7) + 1);
        },
    }

    // Add specified add notes
    for &add in &chord.adds {
        degrees.push(add);
    }

    // Remove duplicates and sort
    degrees.sort_unstable();
    degrees.dedup();

    Ok(degrees)
}

/// Convert scale degree to raw chromatic value (simplified)
fn scale_degree_to_raw(scale_degree: u8) -> u8 {
    // Major scale intervals: W-W-H-W-W-W-H
    let major_intervals = [0, 2, 4, 5, 7, 9, 11]; // C major scale
    major_intervals
        .get((scale_degree.saturating_sub(1)) as usize % 7)
        .copied()
        .unwrap_or(0)
}

/// Cluster-based detokenization
pub fn detokenize_cluster(
    token_string: &str,
    _scale: &ScaleFingerprint,
) -> SerializationResult<(Vec<Chord>, Vec<Note>, f64)> {
    let mut chords = Vec::new();
    let mut notes = Vec::new();
    let mut total_duration = 0.0;

    // Parse XML-like structure with regex
    let chord_regex =
        Regex::new(r"<CHORD>(.*?)</CHORD>").map_err(|_| SerializationError::InvalidFormat {
            message: "Failed to compile chord regex".to_string(),
        })?;

    let note_regex =
        Regex::new(r"<NOTES>(.*?)</NOTES>").map_err(|_| SerializationError::InvalidFormat {
            message: "Failed to compile note regex".to_string(),
        })?;

    let duration_regex =
        Regex::new(r"D_([0-9a-f]+)").map_err(|_| SerializationError::InvalidFormat {
            message: "Failed to compile duration regex".to_string(),
        })?;

    // Extract duration
    if let Some(duration_match) = duration_regex.find(token_string) {
        let duration_token = duration_match.as_str();
        total_duration = parse_duration_token(duration_token)?;
    }

    // Extract chord clusters
    for chord_match in chord_regex.find_iter(token_string) {
        let chord_content = chord_match.as_str();
        if let Ok(chord) = parse_chord_cluster(chord_content) {
            chords.push(chord);
        }
    }

    // Extract note groups
    for note_match in note_regex.find_iter(token_string) {
        let note_content = note_match.as_str();
        if let Ok(parsed_notes) = parse_note_group(note_content) {
            notes.extend(parsed_notes);
        }
    }

    Ok((chords, notes, total_duration))
}

/// MIDI-like detokenization
pub fn detokenize_midi_like(
    tokens: &[String],
    _scale: &ScaleFingerprint,
) -> SerializationResult<Timeline> {
    let mut events = Vec::new();
    let mut current_beat = 0.0;

    for token in tokens {
        if token.starts_with("NOTE-") && token.ends_with("-ON") {
            // Parse MIDI note
            if let Ok(note) = parse_midi_note_token(token) {
                events.push(TokenEvent {
                    beat: current_beat,
                    event_type: TokenEventType::Note(note),
                });
            }
        } else if token.starts_with("CHORD-") && token.ends_with("-ON") {
            // Parse chord token
            if let Ok(chord) = parse_chord_token(token) {
                events.push(TokenEvent {
                    beat: current_beat,
                    event_type: TokenEventType::Chord(chord),
                });
            }
        } else if token.starts_with("DELAY-") {
            // Advance timeline
            if let Ok(delay) = parse_delay_token(token) {
                current_beat += delay;
            }
        } else if token == "NOTE-REST-ON" {
            events.push(TokenEvent {
                beat: current_beat,
                event_type: TokenEventType::Rest,
            });
        }
    }

    Ok(Timeline {
        events,
        total_duration: current_beat,
    })
}

/// Reconstruct timeline from events
pub fn reconstruct_timeline(events: Vec<TokenEvent>) -> Timeline {
    let mut sorted_events = events;

    // Sort by beat position
    sorted_events.sort_by(|a, b| {
        a.beat
            .partial_cmp(&b.beat)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Calculate total duration
    let total_duration = sorted_events.last().map(|event| event.beat).unwrap_or(0.0);

    // Group simultaneous events and resolve conflicts
    let grouped_events = group_simultaneous_events(sorted_events);

    // Apply quantization to beat grid
    let quantized_events = quantize_to_beat_grid(grouped_events);

    // Fill gaps with rests if needed
    let filled_events = fill_gaps_with_rests(quantized_events);

    Timeline {
        events: filled_events,
        total_duration,
    }
}

// Helper functions for parsing

fn parse_chord_cluster(_chord_content: &str) -> SerializationResult<Chord> {
    // Simplified chord parsing - in practice would use token library
    Chord::new(1, 5).map_err(|e| SerializationError::InvalidFormat {
        message: format!("Failed to create chord: {:?}", e),
    })
}

fn parse_note_group(_note_content: &str) -> SerializationResult<Vec<Note>> {
    // Simplified note parsing
    Ok(vec![Note {
        scale_degree: 1,
        octave: 4,
        is_rest: false,
    }])
}

fn parse_midi_note_token(token: &str) -> SerializationResult<Note> {
    // Parse "NOTE-{midi}-ON" format
    let parts: Vec<&str> = token.split('-').collect();
    if parts.len() != 3 {
        return Err(SerializationError::InvalidFormat {
            message: "Invalid MIDI note token format".to_string(),
        });
    }

    let midi_value = parts[1]
        .parse::<u8>()
        .map_err(|_| SerializationError::InvalidFormat {
            message: "Invalid MIDI value".to_string(),
        })?;

    let octave = midi_value / 12;
    let scale_degree = midi_value % 12;

    Ok(Note {
        scale_degree,
        octave,
        is_rest: false,
    })
}

fn parse_chord_token(_token: &str) -> SerializationResult<Chord> {
    // Simplified chord token parsing
    Chord::new(1, 5).map_err(|e| SerializationError::InvalidFormat {
        message: format!("Failed to parse chord token: {:?}", e),
    })
}

fn parse_delay_token(token: &str) -> SerializationResult<f64> {
    // Parse "DELAY-{amount}" format
    let parts: Vec<&str> = token.split('-').collect();
    if parts.len() != 2 {
        return Err(SerializationError::InvalidFormat {
            message: "Invalid delay token format".to_string(),
        });
    }

    parts[1]
        .parse::<f64>()
        .map_err(|_| SerializationError::InvalidFormat {
            message: "Invalid delay value".to_string(),
        })
}

fn group_simultaneous_events(events: Vec<TokenEvent>) -> Vec<TokenEvent> {
    // Group events that happen at the same beat
    // For now, just return as-is (could be enhanced)
    events
}

fn quantize_to_beat_grid(events: Vec<TokenEvent>) -> Vec<TokenEvent> {
    // Apply quantization to beat grid
    events
        .into_iter()
        .map(|mut event| {
            event.beat = (event.beat * TICKS_PER_BEAT as f64).round() / TICKS_PER_BEAT as f64;
            event
        })
        .collect()
}

fn fill_gaps_with_rests(events: Vec<TokenEvent>) -> Vec<TokenEvent> {
    // Fill gaps with appropriate rests
    // For now, just return as-is (could be enhanced)
    events
}

/// Token validation functions
pub fn validate_token(token: &str) -> bool {
    validate_duration_token(token)
        || validate_raw_note_token(token)
        || validate_octave_token(token)
        || validate_chord_cluster_token(token)
}

pub fn validate_duration_token(token: &str) -> bool {
    Regex::new(DURATION_PATTERN)
        .map(|regex| regex.is_match(token))
        .unwrap_or(false)
}

pub fn validate_raw_note_token(token: &str) -> bool {
    Regex::new(RAW_NOTE_PATTERN)
        .map(|regex| regex.is_match(token))
        .unwrap_or(false)
}

pub fn validate_octave_token(token: &str) -> bool {
    Regex::new(OCTAVE_PATTERN)
        .map(|regex| regex.is_match(token))
        .unwrap_or(false)
}

pub fn validate_chord_cluster_token(token: &str) -> bool {
    Regex::new(CHORD_CLUSTER_PATTERN)
        .map(|regex| regex.is_match(token))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_duration() {
        let token = tokenize_duration(1.5);
        assert_eq!(token, "D_24"); // 1.5 * 24 = 36 = 0x24
    }

    #[test]
    fn test_parse_duration_token() {
        let duration = parse_duration_token("D_24").unwrap();
        assert_eq!(duration, 1.5);
    }

    #[test]
    fn test_validate_tokens() {
        assert!(validate_duration_token("D_24"));
        assert!(validate_raw_note_token("R_0"));
        assert!(validate_octave_token("O_4"));
        assert!(!validate_duration_token("invalid"));
    }

    #[test]
    fn test_token_library() {
        let mut library = TokenLibrary::new();
        library.add_chord_token("test".to_string(), vec![1, 2, 3, 4, 5]);

        assert_eq!(library.get_library_size(), 1);
        let resolved = library.resolve_chord_token("test").unwrap();
        assert_eq!(resolved, vec![1, 2, 3, 4, 5]);
    }
}
