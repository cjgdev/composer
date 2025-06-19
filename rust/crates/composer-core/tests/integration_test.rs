//! Integration tests demonstrating the complete Composer system
//!
//! This test suite validates that all components work together correctly
//! and demonstrates the full functionality of the library.

use composer_config::{ALGORITHM, MUSICAL, PERFORMANCE};
use composer_core::{
    get_chord_complexity, get_relative_chord_graphic, get_relative_scale_degrees,
    get_stable_scale_degrees, BorrowedScale, Chord, ScaleFingerprint,
};
use composer_serialization::{
    chord_binary_to_hex, deserialize_chord, hex_to_chord_binary, serialize_chord,
};

/// Test the complete chord theory workflow
#[test]
fn test_complete_chord_analysis_workflow() {
    // Create a complex chord: V7b9/V in C major
    let chord = Chord::new(2, 7)
        .unwrap()
        .with_applied(5)
        .unwrap()
        .with_alteration("b9")
        .unwrap()
        .with_inversion(1)
        .unwrap();

    let scale = ScaleFingerprint::major_scale();

    // Test relative scale degrees
    let relative_degrees = get_relative_scale_degrees(&chord).unwrap();
    assert_eq!(relative_degrees.sd_numbers, vec![3, 5, 7, 1, 9]); // First inversion
    assert!(relative_degrees.sd_accs.contains(&"b".to_string())); // b9 alteration

    // Test stable scale degrees in scale context
    let stable_degrees = get_stable_scale_degrees(&chord, &scale).unwrap();
    assert!(!stable_degrees.is_empty());

    // Test complexity calculation
    let complexity = get_chord_complexity(&chord, "major").unwrap();
    assert!(complexity > 3.0); // Complex chord should have higher complexity

    // Test Roman numeral generation
    let roman = get_relative_chord_graphic(&chord, &scale).unwrap();
    assert!(roman.is_applied());
    assert!(!roman.alterations.is_empty());

    println!("Chord analysis complete:");
    println!("  Chord: {}", chord);
    println!("  Stable degrees: {:?}", stable_degrees);
    println!("  Complexity: {:.1}", complexity);
    println!("  Roman numeral: {}", roman);
}

/// Test binary serialization round-trip with various chord types
#[test]
fn test_serialization_round_trip() {
    let test_chords = vec![
        // Simple triad
        Chord::new(1, 5).unwrap(),
        // Seventh chord with inversion
        Chord::new(5, 7).unwrap().with_inversion(2).unwrap(),
        // Complex chord with multiple features
        Chord::new(2, 9)
            .unwrap()
            .with_alteration("b5")
            .unwrap()
            .with_alteration("#11")
            .unwrap()
            .with_suspension(4)
            .unwrap()
            .with_add(6)
            .unwrap()
            .with_omit(3)
            .unwrap()
            .with_applied(5)
            .unwrap()
            .with_borrowed_scale(BorrowedScale::Named("harmonic_minor".to_string()))
            .unwrap(),
        // Rest chord
        Chord::rest(),
    ];

    for original_chord in &test_chords {
        // Serialize to binary
        let binary = serialize_chord(original_chord).unwrap();
        assert_eq!(binary.len(), 5); // Exactly 5 bytes

        // Convert to hex for debugging
        let hex = chord_binary_to_hex(&binary);
        assert_eq!(hex.len(), 10); // 10 hex characters

        // Convert back from hex
        let binary_from_hex = hex_to_chord_binary(&hex).unwrap();
        assert_eq!(binary, binary_from_hex);

        // Deserialize back to chord
        let deserialized_chord = deserialize_chord(&binary).unwrap();

        // Verify key properties are preserved
        assert_eq!(original_chord.root, deserialized_chord.root);
        assert_eq!(original_chord.chord_type, deserialized_chord.chord_type);
        assert_eq!(original_chord.inversion, deserialized_chord.inversion);
        assert_eq!(original_chord.applied, deserialized_chord.applied);
        assert_eq!(original_chord.is_rest, deserialized_chord.is_rest);

        // Verify collections are preserved
        for alteration in &original_chord.alterations {
            assert!(deserialized_chord.alterations.contains(alteration));
        }

        for &suspension in &original_chord.suspensions {
            assert!(deserialized_chord.suspensions.contains(&suspension));
        }

        for &add in &original_chord.adds {
            assert!(deserialized_chord.adds.contains(&add));
        }

        for &omit in &original_chord.omits {
            assert!(deserialized_chord.omits.contains(&omit));
        }

        println!(
            "Serialization test passed for: {} -> {}",
            original_chord, hex
        );
    }
}

/// Test scale fingerprint functionality with various scales
#[test]
fn test_scale_functionality() {
    let scales = vec![
        ("Major", ScaleFingerprint::major_scale()),
        ("Minor", ScaleFingerprint::minor_scale()),
        ("Harmonic Minor", ScaleFingerprint::harmonic_minor_scale()),
        ("Dorian", ScaleFingerprint::dorian_scale()),
        ("Chromatic", ScaleFingerprint::chromatic_scale()),
    ];

    for (name, scale) in &scales {
        println!("Testing {} scale: {}", name, scale);

        // Test basic properties
        assert!(scale.note_count() > 0);
        assert!(scale.note_count() <= 12);

        // Test chromatic note queries
        let chromatic_notes = scale.chromatic_notes();
        assert_eq!(chromatic_notes.len(), scale.note_count());

        // Test scale degree conversions
        for (i, &chromatic) in chromatic_notes.iter().enumerate() {
            let scale_degree = scale.chromatic_to_scale_degree(chromatic);
            assert_eq!(scale_degree, Some((i + 1) as u8));

            // Only test conversion back if within valid range
            if (i + 1) <= 7 {
                let back_to_chromatic = scale.scale_degree_to_chromatic((i + 1) as u8);
                assert_eq!(back_to_chromatic, Some(chromatic));
            }
        }

        // Test contains_chromatic
        for i in 0..12 {
            let expected = chromatic_notes.contains(&i);
            assert_eq!(scale.contains_chromatic(i), expected);
        }
    }
}

/// Test performance requirements are met
#[test]
fn test_performance_requirements() {
    use std::time::Instant;

    let chord = Chord::new(5, 7)
        .unwrap()
        .with_alteration("b9")
        .unwrap()
        .with_inversion(1)
        .unwrap();

    let scale = ScaleFingerprint::major_scale();

    // Test chord lookup performance (target: <1ms)
    let start = Instant::now();
    for _ in 0..100 {
        let _degrees = get_stable_scale_degrees(&chord, &scale).unwrap();
    }
    let elapsed = start.elapsed();
    let avg_time_ms = elapsed.as_millis() as f64 / 100.0;

    println!("Average chord lookup time: {:.3}ms", avg_time_ms);
    assert!(avg_time_ms < PERFORMANCE.chord_lookup_max_ms as f64);

    // Test serialization performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _binary = serialize_chord(&chord).unwrap();
    }
    let elapsed = start.elapsed();
    let avg_serialization_ms = elapsed.as_millis() as f64 / 1000.0;

    println!("Average serialization time: {:.3}ms", avg_serialization_ms);
    assert!(avg_serialization_ms < 0.1); // Should be much faster than 0.1ms

    // Test memory usage
    let memory_size = std::mem::size_of::<Chord>();
    println!("Chord struct size: {} bytes", memory_size);
    assert!(memory_size < 500); // Target: <500 bytes per chord
}

/// Test configuration constants are properly set
#[test]
fn test_configuration_constants() {
    // Test musical constants
    assert_eq!(MUSICAL.scale_degrees, 7);
    assert_eq!(MUSICAL.chromatic_notes, 12);
    assert_eq!(MUSICAL.chord_types, &[5, 7, 9, 11, 13]);
    assert_eq!(MUSICAL.ticks_per_beat, 24);

    // Test performance thresholds
    assert_eq!(PERFORMANCE.chord_lookup_max_ms, 1);
    assert_eq!(PERFORMANCE.chord_suggestion_max_ms, 50);

    // Test algorithm constants
    assert_eq!(ALGORITHM.context_bonus_forward, 1.7);
    assert_eq!(ALGORITHM.statistical_strength_divisor, 10000.0);

    println!("Configuration validation passed");
}

/// Test error handling and validation
#[test]
fn test_error_handling() {
    // Test invalid chord creation
    assert!(Chord::new(8, 5).is_err()); // Invalid root
    assert!(Chord::new(1, 6).is_err()); // Invalid type

    // Test invalid scale fingerprint
    let invalid_scale = ScaleFingerprint::from_slice(&[2, 0, 1, 0, 1]); // Invalid value
    assert!(invalid_scale.is_err());

    let empty_scale = ScaleFingerprint::from_slice(&[0; 12]); // Empty scale
    assert!(empty_scale.is_err());

    // Test invalid hex serialization
    assert!(hex_to_chord_binary("invalid").is_err());
    assert!(hex_to_chord_binary("123").is_err()); // Too short

    println!("Error handling tests passed");
}

/// Test chord theory algorithms produce expected results
#[test]
fn test_chord_theory_accuracy() {
    // Test I chord in C major
    let i_chord = Chord::new(1, 5).unwrap();
    let major_scale = ScaleFingerprint::major_scale();

    let degrees = get_stable_scale_degrees(&i_chord, &major_scale).unwrap();
    assert_eq!(degrees, vec!["1", "3", "5"]);

    let complexity = get_chord_complexity(&i_chord, "major").unwrap();
    assert_eq!(complexity, 1.0); // Simple triad

    let roman = get_relative_chord_graphic(&i_chord, &major_scale).unwrap();
    assert_eq!(roman.symbol, "I");
    assert!(roman.is_major());

    // Test ii7 chord in C major
    let ii7_chord = Chord::new(2, 7).unwrap();
    let degrees = get_stable_scale_degrees(&ii7_chord, &major_scale).unwrap();
    assert_eq!(degrees, vec!["2", "4", "6", "1"]);

    let complexity = get_chord_complexity(&ii7_chord, "major").unwrap();
    assert_eq!(complexity, 2.0); // Seventh chord

    let roman = get_relative_chord_graphic(&ii7_chord, &major_scale).unwrap();
    assert_eq!(roman.symbol, "ii7");
    assert!(roman.is_minor());

    println!("Chord theory accuracy tests passed");
}

/// Integration test demonstrating a complete harmonic analysis
#[test]
fn test_harmonic_analysis_workflow() {
    // Analyze the progression: I - vi - IV - V7
    let progression = vec![
        Chord::new(1, 5).unwrap(), // I
        Chord::new(6, 5).unwrap(), // vi
        Chord::new(4, 5).unwrap(), // IV
        Chord::new(5, 7).unwrap(), // V7
    ];

    let scale = ScaleFingerprint::major_scale();
    let mut analysis_results = Vec::new();

    for (i, chord) in progression.iter().enumerate() {
        // Analyze each chord
        let degrees = get_stable_scale_degrees(chord, &scale).unwrap();
        let complexity = get_chord_complexity(chord, "major").unwrap();
        let roman = get_relative_chord_graphic(chord, &scale).unwrap();

        // Serialize for storage/transmission
        let binary = serialize_chord(chord).unwrap();
        let hex = chord_binary_to_hex(&binary);

        analysis_results.push((
            i + 1,
            chord.clone(),
            degrees,
            complexity,
            roman.full_symbol(),
            hex,
        ));
    }

    // Print analysis results
    println!("Harmonic Analysis of I-vi-IV-V7 progression:");
    println!("Pos | Chord | Degrees    | Complexity | Roman | Binary");
    println!("----|-------|------------|------------|-------|----------");

    for (pos, chord, degrees, complexity, roman, hex) in &analysis_results {
        println!(
            "{:3} | {:5} | {:10} | {:8.1} | {:5} | {}",
            pos,
            chord,
            degrees.join(","),
            complexity,
            roman,
            hex
        );
    }

    // Verify the analysis makes sense
    assert_eq!(analysis_results.len(), 4);
    assert_eq!(analysis_results[0].4, "I"); // First chord is I
    assert!(analysis_results[1].4.contains("vi")); // Second chord is vi (may have quality symbol)
    assert_eq!(analysis_results[2].4, "IV"); // Third chord is IV
    assert!(analysis_results[3].4.contains("V")); // Fourth chord is V7 (may have quality symbol)

    // Verify complexity progression (V7 should be most complex)
    let complexities: Vec<f64> = analysis_results.iter().map(|r| r.3).collect();
    assert!(complexities[3] > complexities[0]); // V7 > I

    println!("Complete harmonic analysis workflow test passed");
}

/// Test that the system behaves correctly under edge cases
#[test]
fn test_edge_cases() {
    // Test maximum complexity chord
    let complex_chord = Chord::new(1, 13)
        .unwrap()
        .with_alteration("b5")
        .unwrap()
        .with_alteration("#9")
        .unwrap()
        .with_alteration("#11")
        .unwrap()
        .with_alteration("b13")
        .unwrap()
        .with_suspension(4)
        .unwrap()
        .with_add(6)
        .unwrap()
        .with_omit(3)
        .unwrap()
        .with_inversion(3)
        .unwrap()
        .with_applied(5)
        .unwrap()
        .with_borrowed_scale(BorrowedScale::Named("harmonic_minor".to_string()))
        .unwrap();

    // Should still serialize/deserialize correctly
    let binary = serialize_chord(&complex_chord).unwrap();
    let deserialized = deserialize_chord(&binary).unwrap();
    assert_eq!(complex_chord.root, deserialized.root);

    // Should calculate complexity
    let complexity = get_chord_complexity(&complex_chord, "major").unwrap();
    assert!(complexity > 5.0); // Very complex chord
    assert!(complexity <= 10.0); // But within bounds

    // Test chromatic scale with all chord types
    let chromatic = ScaleFingerprint::chromatic_scale();
    for &chord_type in &[5, 7, 9, 11, 13] {
        for root in 1..=7 {
            let chord = Chord::new(root, chord_type).unwrap();
            let degrees = get_stable_scale_degrees(&chord, &chromatic).unwrap();
            assert!(!degrees.is_empty());
        }
    }

    println!("Edge case tests passed");
}
