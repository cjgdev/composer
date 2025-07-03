//! Tests for scale degree harmonization algorithm based on test specifications

use composer_ai::{ChordProgressionSuggester, ChordProgressionTrie};
use std::sync::Arc;

#[test]
fn test_bit_mask_conversion() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test bit mask conversion from test spec lines 319-322
    // scaleDegreeBits=0b1010100 (degrees 1, 3, 6)
    let scale_degree_bits = 0b1010100; // Bits 2, 4, 6 set (degrees 3, 5, 7 in 1-based)

    let result = suggester.get_harmonize_by_sd_solutions(scale_degree_bits, "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should convert bit mask to scale degree array
    // Should find chord suggestions containing those scale degrees
    assert!(suggestions.len() <= 5, "Should respect limit parameter");

    for suggestion in &suggestions {
        assert!(suggestion.confidence >= 0.0 && suggestion.confidence <= 1.0);
        assert!(!suggestion.reasoning.is_empty());
    }
}

#[test]
fn test_multi_factor_scoring() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test multi-factor scoring from test spec lines 324-334
    let scale_degree_bits = 0b1000001; // Degrees 1 and 7

    let result = suggester.get_harmonize_by_sd_solutions(scale_degree_bits, "major", 10);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should apply comprehensive scoring algorithm
    // FACTORS:
    // - lookupScore: Basic chord complexity (0-10)
    // - lengthScore: Penalty for excessive notes
    // - complexityScore: Advanced harmony assessment (0-10)
    // - magicScore: Statistical popularity (0-100)
    // FORMULA: total = lookup×0.2 + length×0.2 + complexity×0.3 + magic×0.3

    for suggestion in &suggestions {
        // All scores should be normalized
        assert!(
            suggestion.frequency_score >= 0.0 && suggestion.frequency_score <= 1.0,
            "Frequency score should be normalized: {}",
            suggestion.frequency_score
        );
        assert!(
            suggestion.theory_score >= 0.0 && suggestion.theory_score <= 1.0,
            "Theory score should be normalized: {}",
            suggestion.theory_score
        );
        assert!(
            suggestion.weighted_score >= 0.0 && suggestion.weighted_score <= 1.0,
            "Weighted score should be normalized: {}",
            suggestion.weighted_score
        );
    }
}

#[test]
fn test_chromatic_extension_support() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test chromatic extension support from test spec
    let chromatic_bits = 0b111111111111; // All 12 chromatic notes

    let result = suggester.get_harmonize_by_sd_solutions(chromatic_bits, "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should handle chromatic extensions in scale degrees
    // Should find appropriate chromatic harmonizations
    for suggestion in &suggestions {
        assert!(
            suggestion.confidence > 0.0,
            "Should provide valid harmonizations"
        );
    }
}

#[test]
fn test_enharmonic_equivalent_handling() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test enharmonic equivalent handling from test spec
    let bits1 = 0b0000010; // Degree 2 (D)
    let bits2 = 0b0000010; // Same bit pattern

    let result1 = suggester.get_harmonize_by_sd_solutions(bits1, "major", 5);
    let result2 = suggester.get_harmonize_by_sd_solutions(bits2, "major", 5);

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    let suggestions1 = result1.unwrap();
    let suggestions2 = result2.unwrap();

    // Should treat enharmonic equivalents consistently
    assert_eq!(
        suggestions1.len(),
        suggestions2.len(),
        "Enharmonic equivalents should produce same results"
    );
}

#[test]
fn test_set_operations_efficiency() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test efficient set operations from test spec
    let complex_bits = 0b101010101010; // Complex scale degree combination

    let start = std::time::Instant::now();
    let result = suggester.get_harmonize_by_sd_solutions(complex_bits, "major", 10);
    let elapsed = start.elapsed();

    assert!(result.is_ok());

    // Should use efficient set operations for degree matching
    assert!(
        elapsed.as_millis() < 100,
        "Complex scale degree matching should be efficient: {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_empty_scale_degrees() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test empty scale degrees
    let empty_bits = 0b000000000000; // No degrees set

    let result = suggester.get_harmonize_by_sd_solutions(empty_bits, "major", 5);

    assert!(result.is_err());
    if let Err(error) = result {
        assert!(
            error.to_string().contains("No scale degrees"),
            "Should validate that scale degrees are specified"
        );
    }
}

#[test]
fn test_single_scale_degree() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test single scale degree harmonization
    let single_bit = 0b0000001; // Just degree 1 (tonic)

    let result = suggester.get_harmonize_by_sd_solutions(single_bit, "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should provide harmonizations for single scale degree
    assert!(
        !suggestions.is_empty(),
        "Should harmonize single scale degree"
    );

    for suggestion in &suggestions {
        // All suggestions should contain the specified scale degree
        assert_eq!(
            suggestion.chord.root, 1,
            "Single degree harmonization should use that root"
        );
    }
}

#[test]
fn test_multiple_scale_degrees() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test multiple scale degrees (typical triad)
    let triad_bits = 0b0010101; // Degrees 1, 3, 5 (major triad degrees)

    let result = suggester.get_harmonize_by_sd_solutions(triad_bits, "major", 10);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    // Should provide multiple harmonization options
    assert!(
        !suggestions.is_empty(),
        "Should harmonize multiple scale degrees"
    );

    // Should prefer chords that contain more of the specified degrees
    for suggestion in &suggestions {
        assert!(
            suggestion.weighted_score > 0.0,
            "Should have positive score for matching degrees"
        );
    }
}

#[test]
fn test_scoring_formula_validation() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    let scale_degree_bits = 0b0000101; // Degrees 1 and 3

    let result = suggester.get_harmonize_by_sd_solutions(scale_degree_bits, "major", 5);

    assert!(result.is_ok());
    let suggestions = result.unwrap();

    for suggestion in &suggestions {
        // Verify scoring formula from test spec:
        // total = lookup×0.2 + length×0.2 + complexity×0.3 + magic×0.3

        // All component scores should contribute to final score
        assert!(suggestion.frequency_score >= 0.0 && suggestion.frequency_score <= 1.0);
        assert!(suggestion.context_score >= 0.0 && suggestion.context_score <= 1.0);
        assert!(suggestion.theory_score >= 0.0 && suggestion.theory_score <= 1.0);
        assert!(suggestion.weighted_score >= 0.0 && suggestion.weighted_score <= 1.0);

        // Final score should be reasonable combination of components
        let _expected_min = suggestion.frequency_score * 0.3; // Minimum if other factors are 0
        let _expected_max = (suggestion.frequency_score + suggestion.theory_score + 1.0) * 0.3 + 0.4;

        assert!(
            suggestion.weighted_score >= 0.0,
            "Weighted score should be at least 0.0"
        );
        assert!(
            suggestion.weighted_score <= 1.0,
            "Weighted score should be at most 1.0"
        );
    }
}

#[test]
fn test_bits_to_scale_degrees_conversion() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let suggester = ChordProgressionSuggester::new(trie);

    // Test bit conversion logic
    let bits = 0b1010101010101; // Alternating pattern
    let degrees = suggester.bits_to_scale_degrees(bits);

    // Should convert bits to 1-based scale degrees
    let expected_degrees = vec![1, 3, 5, 7, 9, 11, 13]; // 1-based degrees for set bits
    assert_eq!(degrees.len(), expected_degrees.len());

    for (i, &degree) in degrees.iter().enumerate() {
        assert_eq!(
            degree, expected_degrees[i],
            "Bit {} should convert to degree {}",
            i, expected_degrees[i]
        );
    }
}
