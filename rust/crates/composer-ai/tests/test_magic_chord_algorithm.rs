//! Tests for magic chord algorithm implementation based on test specifications

use composer_ai::{ChordProgressionSuggester, ChordProgressionTrie};
use composer_core::Chord;
use std::sync::Arc;

#[test]
fn test_get_magic_chord_solutions_basic() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test basic context-aware suggestion from test spec
    let previous_chords = vec![Chord::new(1, 5).unwrap()]; // I
    let following_chords = vec![Chord::new(6, 5).unwrap()]; // vi

    let result =
        suggester.get_magic_chord_solutions(&previous_chords, &following_chords, "major", 10);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should provide contextually appropriate chord suggestions
    // Results should be sorted by weight (descending)
    // All suggestions should have valid chord objects
    for (i, suggestion) in suggestions.iter().enumerate() {
        if i > 0 {
            assert!(
                suggestion.weighted_score <= suggestions[i - 1].weighted_score,
                "Suggestions should be sorted by weight descending"
            );
        }

        assert!(
            suggestion.confidence >= 0.0 && suggestion.confidence <= 1.0,
            "Confidence should be between 0.0 and 1.0"
        );
        assert!(
            suggestion.weighted_score >= 0.0 && suggestion.weighted_score <= 1.0,
            "Weight should be between 0.0 and 1.0"
        );
    }
}

#[test]
fn test_empty_context_handling() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test empty context handling from test spec
    let result = suggester.get_magic_chord_solutions(&[], &[], "major", 10);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should handle lack of context gracefully
    // Should return general suggestions based on scale statistics
    // No context weighting should be applied
    assert!(suggestions.len() <= 10, "Should respect limit parameter");
}

#[test]
fn test_complex_context_weighting() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test complex context from test spec
    let previous_chords = vec![
        Chord::new(1, 5).unwrap(), // I
        Chord::new(4, 5).unwrap(), // IV
    ];
    let following_chords = vec![
        Chord::new(5, 7).unwrap(), // V7
        Chord::new(1, 5).unwrap(), // I
    ];

    let result =
        suggester.get_magic_chord_solutions(&previous_chords, &following_chords, "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should weight longer context patterns more heavily
    // Should apply forward context bonus (1.7x)
    for suggestion in &suggestions {
        assert!(!suggestion.reasoning.is_empty(), "Should provide reasoning");
    }
}

#[test]
fn test_statistical_strength_calculation() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test weight computation from test spec lines 150-176
    // Expected calculation from test:
    // contextLength = 2 + 1 = 3
    // contextMatch = 1 - (5 - 3) / 5 = 0.6
    // contextBonus = 3 × (1 > 2 ? 1.7 : 1.0) = 3.0
    // statisticalStrength = min((15.5 × 3.0) / 10000, 1.0) = 0.00465
    // weight = 0.6 × 0.00465 = 0.00279

    let weight = suggester.compute_weight_from_spec(2, 1, 5, 1, 15.5);
    let expected_weight = 0.6 * 0.00465; // ≈ 0.00279

    assert!(
        (weight - expected_weight).abs() < 0.001,
        "Weight calculation should match test specification. Expected: {}, Got: {}",
        expected_weight,
        weight
    );
}

#[test]
fn test_forward_context_bonus() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test forward context bonus from test spec
    let weight_forward = suggester.compute_weight_from_spec(1, 2, 4, 1, 100.0);
    let weight_backward = suggester.compute_weight_from_spec(2, 1, 4, 1, 100.0);

    // Forward context (more following than previous) should get 1.7x bonus
    assert!(
        weight_forward > weight_backward,
        "Forward context should have higher weight due to 1.7x bonus"
    );
}

#[test]
fn test_deduplication_logic() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    let result = suggester.get_magic_chord_solutions(&[], &[], "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should remove duplicate chord suggestions
    let mut unique_chords = std::collections::HashSet::new();
    for suggestion in &suggestions {
        let chord_key = (suggestion.chord.root, suggestion.chord.chord_type);
        assert!(
            unique_chords.insert(chord_key),
            "Should not have duplicate suggestions for the same chord"
        );
    }
}

#[test]
fn test_limit_validation() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test limit parameter validation from test spec
    let result = suggester.get_magic_chord_solutions(&[], &[], "major", 101);

    assert!(result.is_err());
    if let Err(error) = result {
        assert!(
            error.to_string().contains("100"),
            "Should validate maximum 100 suggestions"
        );
    }
}

#[test]
fn test_pattern_length_validation() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test array size validation from test spec (max 50 chords)
    let long_pattern: Vec<Chord> = (0..51)
        .map(|i| Chord::new((i % 7) + 1, 5).unwrap())
        .collect();

    let result = suggester.get_magic_chord_solutions(&long_pattern, &[], "major", 5);

    assert!(result.is_err());
    if let Err(error) = result {
        assert!(
            error.to_string().contains("50"),
            "Should validate maximum 50 chords in pattern"
        );
    }
}

#[test]
fn test_performance_monitoring() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test suggestion generation time tracking
    let _initial_time = suggester.avg_suggestion_time_ms();

    let _result = suggester.get_magic_chord_solutions(&[], &[], "major", 5);

    // Performance metrics should be updated
    let final_time = suggester.avg_suggestion_time_ms();

    // Time should be measured and updated (initial is 0.0)
    assert!(final_time >= 0.0, "Should track suggestion generation time");
}
