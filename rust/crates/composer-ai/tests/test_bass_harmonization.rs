//! Tests for bass line harmonization algorithm based on test specifications

use composer_ai::{ChordProgressionSuggester, ChordProgressionTrie};
use std::sync::Arc;

#[test]
fn test_basic_bass_harmonization() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test basic bass harmonization from test spec
    let result = suggester.get_magic_bass_solutions("C", "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // All suggestions should contain C as bass note
    // Results should be sorted by appropriateness
    // Scale compatibility should be verified
    assert!(suggestions.len() <= 5, "Should respect limit parameter");

    for suggestion in &suggestions {
        assert!(
            suggestion.confidence >= 0.0 && suggestion.confidence <= 1.0,
            "Confidence should be between 0.0 and 1.0"
        );
        assert!(!suggestion.reasoning.is_empty(), "Should provide reasoning");
    }
}

#[test]
fn test_bass_note_conversion() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test bass note conversion from test spec
    assert_eq!(suggester.parse_bass_note("C").unwrap(), 0);
    assert_eq!(suggester.parse_bass_note("F#").unwrap(), 6);
    assert_eq!(suggester.parse_bass_note("Gb").unwrap(), 6);
    assert_eq!(suggester.parse_bass_note("B").unwrap(), 11);

    // Test invalid bass note
    assert!(suggester.parse_bass_note("H").is_err());
}

#[test]
fn test_frequency_complexity_weighting() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test frequency-complexity weighting from test spec
    // Formula: weight = frequency × (1 / (1 + complexity × 0.1))

    let result = suggester.get_magic_bass_solutions("C", "major", 10);
    assert!(result.is_ok());

    let suggestions = result.unwrap();

    // Should balance frequency and complexity in scoring
    // Simpler chords should generally have higher weights for same frequency
    for suggestion in &suggestions {
        assert!(suggestion.frequency_score >= 0.0 && suggestion.frequency_score <= 1.0);
        assert!(suggestion.theory_score >= 0.0 && suggestion.theory_score <= 1.0);
    }
}

#[test]
fn test_scale_compatibility_filtering() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test scale compatibility filtering from test spec
    let major_result = suggester.get_magic_bass_solutions("C", "major", 5);
    let minor_result = suggester.get_magic_bass_solutions("C", "minor", 5);

    assert!(major_result.is_ok());
    assert!(minor_result.is_ok());

    // Should filter suggestions by scale membership
    // Different scales should potentially give different suggestions
    let major_suggestions = major_result.unwrap();
    let minor_suggestions = minor_result.unwrap();

    // Both should provide valid suggestions
    assert!(!major_suggestions.is_empty());
    assert!(!minor_suggestions.is_empty());
}

#[test]
fn test_slash_chord_recognition() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test slash chord recognition from test spec
    // Bass note different from chord root should suggest slash chords
    let result = suggester.get_magic_bass_solutions("E", "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should suggest appropriate slash chords or inversions
    for suggestion in &suggestions {
        // All suggestions should be harmonically valid
        assert!(suggestion.confidence > 0.0);
    }
}

#[test]
fn test_bass_harmonization_performance() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    let start = std::time::Instant::now();
    let _result = suggester.get_magic_bass_solutions("G", "major", 10);
    let elapsed = start.elapsed();

    // Should complete within reasonable time (sub-millisecond target)
    assert!(
        elapsed.as_millis() < 100,
        "Bass harmonization should be fast: {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_bass_harmonization_edge_cases() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test edge cases

    // Empty scale name
    let result = suggester.get_magic_bass_solutions("C", "", 5);
    // Should handle gracefully

    // Zero limit
    let result = suggester.get_magic_bass_solutions("C", "major", 0);
    assert!(result.is_ok());
    let suggestions = result.unwrap();
    assert!(
        suggestions.is_empty(),
        "Zero limit should return no suggestions"
    );

    // Very high limit
    let result = suggester.get_magic_bass_solutions("C", "major", 1000);
    // Should not crash and should respect available suggestions
    assert!(result.is_ok());
}

#[test]
fn test_chromatic_bass_notes() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test all chromatic bass notes
    let notes = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    for note in &notes {
        let result = suggester.get_magic_bass_solutions(note, "major", 3);
        assert!(result.is_ok(), "Should handle bass note: {}", note);

        let suggestions = result.unwrap();
        assert!(
            suggestions.len() <= 3,
            "Should respect limit for note: {}",
            note
        );
    }
}

#[test]
fn test_bass_complexity_scoring() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    let result = suggester.get_magic_bass_solutions("C", "major", 10);
    assert!(result.is_ok());

    let suggestions = result.unwrap();

    // Should score complexity appropriately
    // Simpler chords should generally score higher in theory_score
    for suggestion in &suggestions {
        let complexity = match suggestion.chord.chord_type {
            5 => 1.0, // Triad - simple
            7 => 2.0, // Seventh - moderate
            9 => 3.0, // Ninth - complex
            _ => 4.0,
        };

        // Theory score should be inversely related to complexity
        let expected_theory_score = 1.0 - complexity / 10.0;
        assert!(
            (suggestion.theory_score - expected_theory_score).abs() < 0.1,
            "Theory score should reflect complexity appropriately"
        );
    }
}
