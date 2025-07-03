//! Diagnostic tests for complexity factor calculation bugs
//! 
//! These tests isolate and validate the individual components of complexity calculation
//! to identify the root causes of the averaging and extended harmony detection issues.

use composer_ai::{ChordProgressionTrie, MusicalAnalyzer};
use composer_core::Chord;
use std::sync::Arc;

#[test]
fn test_calculate_single_chord_complexity_direct() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let analyzer = MusicalAnalyzer::new(trie);

    // Test individual chord complexity calculations work correctly
    let triad = Chord::new(1, 5).unwrap(); // I (triad)
    let seventh = Chord::new(1, 7).unwrap(); // I7
    let ninth = Chord::new(1, 9).unwrap(); // I9
    let eleventh = Chord::new(1, 11).unwrap(); // I11
    let thirteenth = Chord::new(1, 13).unwrap(); // I13

    let triad_complexity = analyzer.calculate_single_chord_complexity(&triad);
    let seventh_complexity = analyzer.calculate_single_chord_complexity(&seventh);
    let ninth_complexity = analyzer.calculate_single_chord_complexity(&ninth);
    let eleventh_complexity = analyzer.calculate_single_chord_complexity(&eleventh);
    let thirteenth_complexity = analyzer.calculate_single_chord_complexity(&thirteenth);

    println!("Individual chord complexities:");
    println!("  Triad (I): {:.2}", triad_complexity);
    println!("  Seventh (I7): {:.2}", seventh_complexity);
    println!("  Ninth (I9): {:.2}", ninth_complexity);
    println!("  Eleventh (I11): {:.2}", eleventh_complexity);
    println!("  Thirteenth (I13): {:.2}", thirteenth_complexity);

    // Verify the progression from simple to complex
    assert!(triad_complexity < seventh_complexity, 
        "Triad ({:.2}) should be simpler than seventh ({:.2})", triad_complexity, seventh_complexity);
    assert!(seventh_complexity < ninth_complexity, 
        "Seventh ({:.2}) should be simpler than ninth ({:.2})", seventh_complexity, ninth_complexity);
    assert!(ninth_complexity < eleventh_complexity, 
        "Ninth ({:.2}) should be simpler than eleventh ({:.2})", ninth_complexity, eleventh_complexity);
    assert!(eleventh_complexity < thirteenth_complexity, 
        "Eleventh ({:.2}) should be simpler than thirteenth ({:.2})", eleventh_complexity, thirteenth_complexity);

    // Verify reasonable ranges
    assert!(triad_complexity >= 1.0 && triad_complexity <= 2.0, 
        "Triad complexity should be 1.0-2.0: {:.2}", triad_complexity);
    assert!(thirteenth_complexity >= 3.0 && thirteenth_complexity <= 5.0, 
        "Thirteenth complexity should be 3.0-5.0: {:.2}", thirteenth_complexity);
}

#[test]
fn test_is_extended_harmony_detection() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let analyzer = MusicalAnalyzer::new(trie);

    // Test extended harmony detection
    let triad = Chord::new(1, 5).unwrap(); // I (triad) - not extended
    let seventh = Chord::new(1, 7).unwrap(); // I7 - extended
    let ninth = Chord::new(1, 9).unwrap(); // I9 - extended
    let eleventh = Chord::new(1, 11).unwrap(); // I11 - extended
    let thirteenth = Chord::new(1, 13).unwrap(); // I13 - extended

    // Create a test progression with mix of simple and extended chords
    let progression = vec![triad.clone(), seventh.clone(), ninth.clone(), eleventh.clone(), thirteenth.clone()];

    // Test the progression through assess_difficulty to see factors
    let result = analyzer.assess_difficulty(&progression, Some(120.0), Some((4, 4)));
    assert!(result.is_ok());
    let assessment = result.unwrap();

    println!("Extended harmony detection test:");
    println!("  Progression length: {}", progression.len());
    println!("  Extended harmonies detected: {}", assessment.factors.extended_harmonies);
    println!("  Expected extended harmonies: 4 (all except triad)");

    // Should detect 4 extended harmonies (all except the triad)
    assert_eq!(assessment.factors.extended_harmonies, 4, 
        "Should detect 4 extended harmonies, got {}", assessment.factors.extended_harmonies);

    // Average chord complexity should be higher than 1.0
    assert!(assessment.factors.avg_chord_complexity > 1.5, 
        "Average chord complexity should be > 1.5 with extended chords, got {:.2}", 
        assessment.factors.avg_chord_complexity);
}

#[test]
fn test_average_chord_complexity_calculation() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let analyzer = MusicalAnalyzer::new(trie);

    // Test case 1: All triads - should average around 1.0
    let simple_progression = vec![
        Chord::new(1, 5).unwrap(), // I
        Chord::new(4, 5).unwrap(), // IV
        Chord::new(5, 5).unwrap(), // V
        Chord::new(1, 5).unwrap(), // I
    ];

    let simple_result = analyzer.assess_difficulty(&simple_progression, Some(120.0), Some((4, 4)));
    assert!(simple_result.is_ok());
    let simple_assessment = simple_result.unwrap();

    println!("Simple progression (all triads):");
    println!("  Average chord complexity: {:.2}", simple_assessment.factors.avg_chord_complexity);
    println!("  Extended harmonies: {}", simple_assessment.factors.extended_harmonies);

    // Test case 2: All extended chords - should average much higher
    let complex_progression = vec![
        Chord::new(1, 9).unwrap(),  // I9
        Chord::new(6, 11).unwrap(), // vi11
        Chord::new(2, 9).unwrap(),  // ii9
        Chord::new(5, 13).unwrap(), // V13
    ];

    let complex_result = analyzer.assess_difficulty(&complex_progression, Some(120.0), Some((4, 4)));
    assert!(complex_result.is_ok());
    let complex_assessment = complex_result.unwrap();

    println!("\nComplex progression (all extended):");
    println!("  Average chord complexity: {:.2}", complex_assessment.factors.avg_chord_complexity);
    println!("  Extended harmonies: {}", complex_assessment.factors.extended_harmonies);

    // Verify manual calculation matches
    let manual_simple_avg = simple_progression.iter()
        .map(|chord| analyzer.calculate_single_chord_complexity(chord))
        .sum::<f64>() / simple_progression.len() as f64;

    let manual_complex_avg = complex_progression.iter()
        .map(|chord| analyzer.calculate_single_chord_complexity(chord))
        .sum::<f64>() / complex_progression.len() as f64;

    println!("\nManual calculations:");
    println!("  Simple manual average: {:.2}", manual_simple_avg);
    println!("  Complex manual average: {:.2}", manual_complex_avg);

    // The automatic calculation should match the manual calculation
    assert!((simple_assessment.factors.avg_chord_complexity - manual_simple_avg).abs() < 0.01,
        "Simple progression: automatic {:.2} vs manual {:.2}", 
        simple_assessment.factors.avg_chord_complexity, manual_simple_avg);

    assert!((complex_assessment.factors.avg_chord_complexity - manual_complex_avg).abs() < 0.01,
        "Complex progression: automatic {:.2} vs manual {:.2}", 
        complex_assessment.factors.avg_chord_complexity, manual_complex_avg);

    // Complex should be significantly higher than simple
    assert!(complex_assessment.factors.avg_chord_complexity > simple_assessment.factors.avg_chord_complexity + 1.0,
        "Complex avg ({:.2}) should be > simple avg ({:.2}) + 1.0", 
        complex_assessment.factors.avg_chord_complexity, simple_assessment.factors.avg_chord_complexity);

    // Extended harmony counts should be correct
    assert_eq!(simple_assessment.factors.extended_harmonies, 0, 
        "Simple progression should have 0 extended harmonies");
    assert_eq!(complex_assessment.factors.extended_harmonies, 4, 
        "Complex progression should have 4 extended harmonies");
}

#[test]
fn test_parallel_iterator_vs_sequential() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let analyzer = MusicalAnalyzer::new(trie);

    // Test if par_iter() is causing calculation issues
    let test_progression = vec![
        Chord::new(1, 9).unwrap(),  // I9
        Chord::new(6, 11).unwrap(), // vi11
        Chord::new(2, 9).unwrap(),  // ii9
        Chord::new(5, 13).unwrap(), // V13
    ];

    // Calculate manually with sequential iterator
    let sequential_avg = test_progression.iter()
        .map(|chord| analyzer.calculate_single_chord_complexity(chord))
        .sum::<f64>() / test_progression.len() as f64;

    // Calculate manually with parallel iterator (like the code does)
    use rayon::prelude::*;
    let parallel_avg = test_progression.par_iter()
        .map(|chord| analyzer.calculate_single_chord_complexity(chord))
        .sum::<f64>() / test_progression.len() as f64;

    println!("Iterator comparison:");
    println!("  Sequential average: {:.6}", sequential_avg);
    println!("  Parallel average: {:.6}", parallel_avg);

    // Should be identical
    assert!((sequential_avg - parallel_avg).abs() < 0.000001,
        "Sequential ({:.6}) and parallel ({:.6}) should match", sequential_avg, parallel_avg);

    // Get the actual assessment result
    let assessment = analyzer.assess_difficulty(&test_progression, Some(120.0), Some((4, 4))).unwrap();
    
    println!("  Assessment average: {:.6}", assessment.factors.avg_chord_complexity);

    // Assessment should match manual calculations
    assert!((assessment.factors.avg_chord_complexity - sequential_avg).abs() < 0.01,
        "Assessment ({:.6}) should match manual calculation ({:.6})", 
        assessment.factors.avg_chord_complexity, sequential_avg);
}

#[test]
fn test_complexity_factors_debug_output() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let analyzer = MusicalAnalyzer::new(trie);

    // Create the exact same progressions as in the failing harmonic complexity test
    let simple_progression = vec![
        Chord::new(1, 5).unwrap(), // I
        Chord::new(4, 5).unwrap(), // IV
        Chord::new(5, 5).unwrap(), // V
        Chord::new(1, 5).unwrap(), // I
    ];

    let complex_progression = vec![
        Chord::new(1, 9).unwrap(),  // I9
        Chord::new(6, 11).unwrap(), // vi11
        Chord::new(2, 9).unwrap(),  // ii9
        Chord::new(5, 13).unwrap(), // V13
    ];

    // Analyze both progressions
    let simple_result = analyzer.assess_difficulty(&simple_progression, Some(120.0), Some((4, 4)));
    let complex_result = analyzer.assess_difficulty(&complex_progression, Some(120.0), Some((4, 4)));

    assert!(simple_result.is_ok() && complex_result.is_ok());

    let simple_assessment = simple_result.unwrap();
    let complex_assessment = complex_result.unwrap();

    println!("\n=== DETAILED COMPLEXITY FACTOR ANALYSIS ===");
    
    println!("\nSimple Progression (I-IV-V-I):");
    println!("  Unique chords: {}", simple_assessment.factors.unique_chords);
    println!("  Avg chord complexity: {:.4}", simple_assessment.factors.avg_chord_complexity);
    println!("  Extended harmonies: {}", simple_assessment.factors.extended_harmonies);
    println!("  Voice leading complexity: {:.4}", simple_assessment.factors.voice_leading_complexity);
    println!("  Harmonic complexity: {:.4}", simple_assessment.harmonic_complexity);

    println!("\nComplex Progression (I9-vi11-ii9-V13):");
    println!("  Unique chords: {}", complex_assessment.factors.unique_chords);
    println!("  Avg chord complexity: {:.4}", complex_assessment.factors.avg_chord_complexity);
    println!("  Extended harmonies: {}", complex_assessment.factors.extended_harmonies);
    println!("  Voice leading complexity: {:.4}", complex_assessment.factors.voice_leading_complexity);
    println!("  Harmonic complexity: {:.4}", complex_assessment.harmonic_complexity);

    // Manual verification of individual chord complexities
    println!("\nIndividual chord complexities (Simple):");
    for (i, chord) in simple_progression.iter().enumerate() {
        let complexity = analyzer.calculate_single_chord_complexity(chord);
        println!("  Chord {} (type {}): {:.4}", i, chord.chord_type, complexity);
    }

    println!("\nIndividual chord complexities (Complex):");
    for (i, chord) in complex_progression.iter().enumerate() {
        let complexity = analyzer.calculate_single_chord_complexity(chord);
        println!("  Chord {} (type {}): {:.4}", i, chord.chord_type, complexity);
    }

    // This test should reveal the exact point where the calculation goes wrong
    assert!(complex_assessment.factors.extended_harmonies > simple_assessment.factors.extended_harmonies,
        "Complex should have more extended harmonies than simple");
    
    assert!(complex_assessment.factors.avg_chord_complexity > simple_assessment.factors.avg_chord_complexity + 1.0,
        "Complex should have significantly higher average chord complexity");
}

#[test]
fn test_caching_issue_diagnosis() {
    let trie = Arc::new(ChordProgressionTrie::new());
    let analyzer = MusicalAnalyzer::new(trie);

    // Test if caching is causing incorrect results by using fresh analyzer instances
    
    // Test 1: Simple progression with new analyzer
    let analyzer1 = MusicalAnalyzer::new(Arc::new(ChordProgressionTrie::new()));
    let simple_progression = vec![
        Chord::new(1, 5).unwrap(), // I
        Chord::new(4, 5).unwrap(), // IV
        Chord::new(5, 5).unwrap(), // V
        Chord::new(1, 5).unwrap(), // I
    ];
    let simple_result = analyzer1.assess_difficulty(&simple_progression, Some(120.0), Some((4, 4))).unwrap();
    
    // Test 2: Complex progression with another new analyzer
    let analyzer2 = MusicalAnalyzer::new(Arc::new(ChordProgressionTrie::new()));
    let complex_progression = vec![
        Chord::new(1, 9).unwrap(),  // I9
        Chord::new(6, 11).unwrap(), // vi11
        Chord::new(2, 9).unwrap(),  // ii9
        Chord::new(5, 13).unwrap(), // V13
    ];
    let complex_result = analyzer2.assess_difficulty(&complex_progression, Some(120.0), Some((4, 4))).unwrap();
    
    println!("Fresh analyzer test (no shared cache):");
    println!("  Simple - Avg: {:.4}, Extended: {}", simple_result.factors.avg_chord_complexity, simple_result.factors.extended_harmonies);
    println!("  Complex - Avg: {:.4}, Extended: {}", complex_result.factors.avg_chord_complexity, complex_result.factors.extended_harmonies);
    
    // Test 3: Same analyzer, sequential tests (potential cache pollution)
    analyzer.clear_cache(); // Clear any previous cache
    let simple_sequential = analyzer.assess_difficulty(&simple_progression, Some(120.0), Some((4, 4))).unwrap();
    let complex_sequential = analyzer.assess_difficulty(&complex_progression, Some(120.0), Some((4, 4))).unwrap();
    
    println!("\nSame analyzer, sequential (potential cache issue):");
    println!("  Simple - Avg: {:.4}, Extended: {}", simple_sequential.factors.avg_chord_complexity, simple_sequential.factors.extended_harmonies);
    println!("  Complex - Avg: {:.4}, Extended: {}", complex_sequential.factors.avg_chord_complexity, complex_sequential.factors.extended_harmonies);
    
    // Test 4: Same analyzer, reverse order
    analyzer.clear_cache();
    let complex_first = analyzer.assess_difficulty(&complex_progression, Some(120.0), Some((4, 4))).unwrap();
    let simple_second = analyzer.assess_difficulty(&simple_progression, Some(120.0), Some((4, 4))).unwrap();
    
    println!("\nSame analyzer, reverse order:");
    println!("  Complex first - Avg: {:.4}, Extended: {}", complex_first.factors.avg_chord_complexity, complex_first.factors.extended_harmonies);
    println!("  Simple second - Avg: {:.4}, Extended: {}", simple_second.factors.avg_chord_complexity, simple_second.factors.extended_harmonies);
    
    // All tests should show consistent results
    assert_eq!(simple_result.factors.avg_chord_complexity, simple_sequential.factors.avg_chord_complexity, 
        "Simple progression should have consistent results across analyzers");
    assert_eq!(complex_result.factors.avg_chord_complexity, complex_sequential.factors.avg_chord_complexity, 
        "Complex progression should have consistent results across analyzers");
    
    // Complex should always be more complex than simple
    assert!(complex_result.factors.avg_chord_complexity > simple_result.factors.avg_chord_complexity + 1.0,
        "Complex should be significantly harder than simple (fresh analyzers)");
    assert!(complex_sequential.factors.avg_chord_complexity > simple_sequential.factors.avg_chord_complexity + 1.0,
        "Complex should be significantly harder than simple (sequential)");
}