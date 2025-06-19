#!/usr/bin/env python3
"""
Advanced Music Theory Analysis

This example demonstrates advanced music theory functionality:
- Scale degree analysis in various contexts
- Voice leading analysis and validation
- Harmonic function classification
- Tritone substitution validation
- Advanced chord substitutions and transformations
- Cross-scale modulation analysis

Based on the Composer specification: chord-theory-core.spec
"""

import composer


def demonstrate_scale_degree_analysis() -> None:
    """Show comprehensive scale degree analysis."""
    print("=== Scale Degree Analysis ===")

    major_scale = composer.ScaleFingerprint.major()
    minor_scale = composer.ScaleFingerprint.minor()

    # Test chords for scale degree analysis
    test_chords = [
        composer.Chord(1, 5),  # I major
        composer.Chord(2, 7),  # ii7
        composer.Chord(5, 7),  # V7
        composer.Chord(6, 5),  # vi
        composer.Chord(4, 9),  # IV9
    ]

    print("Scale degree analysis in major scale:")
    for chord in test_chords:
        print(f"  {chord}:")

        try:
            # Get stable scale degrees (absolute)
            stable_degrees = composer.get_stable_scale_degrees(chord, major_scale)
            print(f"    Stable scale degrees: {stable_degrees}")

            # Get relative scale degrees
            relative_degrees = composer.get_relative_scale_degrees(chord)
            print(f"    Relative degrees: {relative_degrees}")

            # Show degree numbers and accidentals separately if available
            if hasattr(relative_degrees, "sd_numbers") and hasattr(
                relative_degrees, "sd_accs"
            ):
                print(f"    Degree numbers: {relative_degrees.sd_numbers}")
                print(f"    Accidentals: {relative_degrees.sd_accs}")

        except AttributeError:
            # Fallback analysis
            print(f"    Root: {chord.root}, Type: {chord.chord_type}")
            print(f"    Inversion: {chord.inversion}")

            # Manual scale degree calculation
            base_degrees = {
                5: [1, 3, 5],  # Triad
                7: [1, 3, 5, 7],  # Seventh
                9: [1, 3, 5, 7, 9],  # Ninth
                11: [1, 3, 5, 7, 9, 11],  # Eleventh
                13: [1, 3, 5, 7, 9, 11, 13],  # Thirteenth
            }

            if chord.chord_type in base_degrees:
                chord_degrees = base_degrees[chord.chord_type]
                # Transpose by root
                transposed = [
                    (degree + chord.root - 2) % 7 + 1 for degree in chord_degrees
                ]
                print(f"    Calculated degrees: {transposed}")

        print()

    print("Comparing scale degree analysis across different scales:")
    test_chord = composer.Chord(5, 7)  # V7 chord
    scales = [
        ("Major", major_scale),
        ("Natural Minor", minor_scale),
        ("Dorian", composer.ScaleFingerprint.dorian()),
        ("Mixolydian", composer.ScaleFingerprint.mixolydian()),
    ]

    for scale_name, scale in scales:
        print(f"  {test_chord} in {scale_name}:")

        try:
            stable_degrees = composer.get_stable_scale_degrees(test_chord, scale)
            print(f"    Scale degrees: {stable_degrees}")
        except AttributeError:
            print(f"    Scale: {scale}")

        print()


def demonstrate_harmonic_functions() -> None:
    """Show harmonic function classification."""
    print("=== Harmonic Function Analysis ===")

    # Define harmonic functions
    harmonic_functions = {
        "Tonic": [1, 6, 3],  # I, vi, iii
        "Predominant": [4, 2],  # IV, ii
        "Dominant": [5, 7],  # V, vii°
    }

    print("Harmonic function classification:")
    for function_name, scale_degrees in harmonic_functions.items():
        print(f"  {function_name} function:")

        for degree in scale_degrees:
            # Create different chord types on each degree
            chord_types = [5, 7]  # Triad and seventh

            for chord_type in chord_types:
                chord = composer.Chord(degree, chord_type)
                print(f"    {chord} (scale degree {degree})")

                try:
                    # Analyze harmonic function
                    function = composer.analyze_harmonic_function(chord, "major")
                    print(f"      Function: {function}")

                except AttributeError:
                    # Manual function classification
                    if degree in harmonic_functions["Tonic"]:
                        function = "Tonic"
                    elif degree in harmonic_functions["Predominant"]:
                        function = "Predominant"
                    elif degree in harmonic_functions["Dominant"]:
                        function = "Dominant"
                    else:
                        function = "Chromatic"

                    print(f"      Function: {function}")

                # Show complexity
                complexity = composer.get_chord_complexity(chord)
                print(f"      Complexity: {complexity:.2f}")

        print()


def demonstrate_voice_leading() -> None:
    """Show voice leading analysis and validation."""
    print("=== Voice Leading Analysis ===")

    # Common chord progressions with different voice leading qualities
    progressions = [
        {
            "name": "Good voice leading (I-vi-IV-V)",
            "chords": [
                composer.Chord(1, 5),  # I
                composer.Chord(6, 5),  # vi
                composer.Chord(4, 5),  # IV
                composer.Chord(5, 5),  # V
            ],
        },
        {
            "name": "Parallel motion (I-II-III)",
            "chords": [
                composer.Chord(1, 5),  # I
                composer.Chord(2, 5),  # II (parallel major chords)
                composer.Chord(3, 5),  # III
            ],
        },
        {
            "name": "Large leaps (I-♭VI-I)",
            "chords": [
                composer.Chord(1, 5),  # I
                composer.Chord(6, 5, borrowed="minor"),  # ♭VI
                composer.Chord(1, 5),  # I
            ],
        },
    ]

    print("Voice leading analysis:")
    for progression in progressions:
        print(f"  {progression['name']}:")
        chords = progression["chords"]

        print(f"    Progression: {[str(c) for c in chords]}")

        try:
            # Analyze voice leading quality
            quality = composer.analyze_voice_leading(chords)
            print(f"    Voice leading quality: {quality:.3f}")

            # Check for specific voice leading issues
            parallel_fifths = composer.check_parallel_fifths(chords)
            parallel_octaves = composer.check_parallel_octaves(chords)
            large_leaps = composer.check_large_leaps(chords)

            print(f"    Parallel fifths: {parallel_fifths}")
            print(f"    Parallel octaves: {parallel_octaves}")
            print(f"    Large leaps: {large_leaps}")

        except AttributeError:
            # Manual voice leading analysis
            total_complexity = sum(composer.get_chord_complexity(c) for c in chords)
            avg_complexity = total_complexity / len(chords)

            # Simple heuristics
            print(f"    Average complexity: {avg_complexity:.2f}")

            # Check for chromatic movement
            roots = [c.root for c in chords]
            chromatic_steps = sum(
                1 for i in range(1, len(roots)) if abs(roots[i] - roots[i - 1]) == 1
            )
            print(f"    Chromatic steps: {chromatic_steps}")

            # Estimate voice leading quality
            quality_estimate = max(0, 1 - (avg_complexity - 2) / 8)
            print(f"    Estimated quality: {quality_estimate:.3f}")

        print()


def demonstrate_tritone_substitution() -> None:
    """Show tritone substitution analysis and validation."""
    print("=== Tritone Substitution Analysis ===")

    # Test chords for tritone substitution
    test_chords = [
        composer.Chord(5, 7),  # V7 (should be valid)
        composer.Chord(5, 7, applied=5),  # V7/V (should be valid)
        composer.Chord(5, 7, applied=6),  # V7/vi (should be valid)
        composer.Chord(1, 5),  # I (should not be valid)
        composer.Chord(2, 7),  # ii7 (should not be valid)
    ]

    print("Tritone substitution validation:")
    for chord in test_chords:
        print(f"  {chord}:")

        try:
            # Check if tritone substitution is valid
            is_valid = composer.is_valid_tri_sub(chord, "major")
            print(f"    Valid for tritone substitution: {is_valid}")

            if is_valid:
                # Calculate the tritone substitute
                substitute = composer.get_tritone_substitute(chord)
                print(f"    Tritone substitute: {substitute}")

        except AttributeError:
            # Manual tritone sub validation
            is_dominant = chord.chord_type == 7 and (
                chord.root == 5 or chord.applied != 0
            )

            print(f"    Is dominant seventh: {is_dominant}")
            print(f"    Applied chord: {chord.applied != 0}")

            if is_dominant:
                # Calculate manual substitute (tritone away)
                sub_root = (chord.root + 3) % 7 + 1  # Tritone = 3 scale degrees
                print(f"    Would substitute with root: {sub_root}")

        print()


def demonstrate_chord_substitutions() -> None:
    """Show various chord substitution techniques."""
    print("=== Chord Substitution Techniques ===")

    original_progression = [
        composer.Chord(1, 5),  # I
        composer.Chord(6, 5),  # vi
        composer.Chord(4, 5),  # IV
        composer.Chord(5, 5),  # V
    ]

    print(f"Original progression: {[str(c) for c in original_progression]}")
    print()

    # Different substitution techniques
    substitution_techniques = [
        {
            "name": "Relative minor/major substitution",
            "substitutions": {
                0: composer.Chord(6, 5),  # I -> vi
                1: composer.Chord(1, 5),  # vi -> I
            },
        },
        {
            "name": "Modal interchange (borrowed chords)",
            "substitutions": {
                2: composer.Chord(4, 5, borrowed="minor"),  # IV -> iv
            },
        },
        {
            "name": "Secondary dominants",
            "substitutions": {
                3: composer.Chord(5, 7, applied=6),  # V -> V7/vi
            },
        },
        {
            "name": "Extended harmonies",
            "substitutions": {
                0: composer.Chord(1, 7),  # I -> IM7
                3: composer.Chord(5, 9),  # V -> V9
            },
        },
    ]

    for technique in substitution_techniques:
        print(f"  {technique['name']}:")

        # Apply substitutions
        substituted = original_progression.copy()
        for index, substitute_chord in technique["substitutions"].items():
            original_chord = substituted[index]
            substituted[index] = substitute_chord

            print(f"    Position {index + 1}: {original_chord} -> {substitute_chord}")

            # Analyze the substitution
            try:
                # Check if chords are isotonal
                major_scale = composer.ScaleFingerprint.major()
                are_isotonal = composer.is_isotonal(
                    original_chord, substitute_chord, major_scale
                )
                print(f"      Isotonal: {are_isotonal}")

            except AttributeError:
                # Compare complexity
                orig_complexity = composer.get_chord_complexity(original_chord)
                sub_complexity = composer.get_chord_complexity(substitute_chord)

                complexity_change = f"{orig_complexity:.2f} -> {sub_complexity:.2f}"
                print(f"      Complexity change: {complexity_change}")

        print(f"    Result: {[str(c) for c in substituted]}")
        print()


def demonstrate_modulation_analysis() -> None:
    """Show analysis of modulation between keys/scales."""
    print("=== Modulation Analysis ===")

    # Common modulation progression: C major to G major
    modulation_progression = [
        composer.Chord(1, 5),  # I in C (C major)
        composer.Chord(6, 5),  # vi in C (A minor)
        composer.Chord(2, 7),  # ii7 in C (D minor 7) - becomes vi7 in G
        composer.Chord(5, 7),  # V7 in C (G7) - becomes I7 in G
        composer.Chord(1, 5),  # I in G (G major) - new key
    ]

    scales = [
        composer.ScaleFingerprint.major(),  # C major
        composer.ScaleFingerprint.major(),  # G major (same pattern, different tonic)
    ]

    print("Analyzing modulation from C major to G major:")
    print(f"Progression: {[str(c) for c in modulation_progression]}")
    print()

    # Analyze each chord in both key contexts
    for i, chord in enumerate(modulation_progression):
        print(f"  Chord {i + 1}: {chord}")

        # Analyze in C major context
        try:
            degrees_c = composer.get_stable_scale_degrees(chord, scales[0])
            print(f"    In C major: {degrees_c}")
        except AttributeError:
            print(f"    In C major: root {chord.root}")

        # Analyze in G major context (transpose analysis)
        try:
            # For G major, we need to transpose the analysis
            degrees_g = composer.get_stable_scale_degrees(chord, scales[1])
            print(f"    In G major: {degrees_g}")
        except AttributeError:
            # Manual transposition
            transposed_root = (chord.root + 4) % 7 + 1  # G is 5th degree of C
            print(f"    In G major: root {transposed_root}")

        # Determine pivot chord quality
        if i == 2 or i == 3:  # These are pivot chords
            print("    Pivot chord function")

        print()

    # Analyze modulation smoothness
    print("Modulation analysis:")
    total_complexity = sum(
        composer.get_chord_complexity(c) for c in modulation_progression
    )
    avg_complexity = total_complexity / len(modulation_progression)

    print(f"  Average complexity: {avg_complexity:.2f}")
    print(f"  Modulation smoothness: {'Smooth' if avg_complexity < 3 else 'Complex'}")
    print()


def demonstrate_cross_scale_analysis() -> None:
    """Show analysis of chords across different scale types."""
    print("=== Cross-Scale Analysis ===")

    test_chord = composer.Chord(5, 7)  # V7 chord

    # Analyze the same chord in different scale contexts
    scale_contexts = [
        ("Major", composer.ScaleFingerprint.major()),
        ("Natural Minor", composer.ScaleFingerprint.minor()),
        ("Dorian", composer.ScaleFingerprint.dorian()),
        ("Mixolydian", composer.ScaleFingerprint.mixolydian()),
        (
            "Harmonic Minor",
            composer.ScaleFingerprint(
                [
                    True,
                    False,
                    True,
                    True,
                    False,
                    True,
                    False,
                    True,
                    True,
                    False,
                    False,
                    True,
                ]
            ),
        ),
    ]

    print(f"Analyzing {test_chord} across different scales:")
    print()

    for scale_name, scale in scale_contexts:
        print(f"  In {scale_name} scale:")
        print(f"    Scale pattern: {scale.chromatic_notes}")

        try:
            stable_degrees = composer.get_stable_scale_degrees(test_chord, scale)
            print(f"    Scale degrees: {stable_degrees}")

            # Check if chord fits naturally in scale
            chord_notes = set(stable_degrees)
            scale_notes = set(
                str(i) for i, present in enumerate(scale.chromatic_notes, 1) if present
            )

            chord_in_scale = chord_notes.issubset(scale_notes)
            print(f"    Fits in scale: {chord_in_scale}")

        except AttributeError:
            # Manual analysis
            print(f"    Root: {test_chord.root}, Type: {test_chord.chord_type}")

            # Count how many chord tones are in the scale
            chord_tones = 4 if test_chord.chord_type == 7 else 3  # Rough estimate
            print(f"    Estimated chord tones: {chord_tones}")

        # Function in this scale context
        if test_chord.root == 5:
            if scale_name == "Major":
                function = "Dominant (V7)"
            elif scale_name == "Mixolydian":
                function = "Tonic (I7)"
            else:
                function = "Fifth degree seventh"
        else:
            function = f"Degree {test_chord.root} seventh"

        print(f"    Harmonic function: {function}")
        print()


def main() -> None:
    """Run all advanced theory demonstration functions."""
    print("Composer Library - Advanced Music Theory Examples")
    print("=" * 60)
    print()

    demonstrate_scale_degree_analysis()
    demonstrate_harmonic_functions()
    demonstrate_voice_leading()
    demonstrate_tritone_substitution()
    demonstrate_chord_substitutions()
    demonstrate_modulation_analysis()
    demonstrate_cross_scale_analysis()

    print("All advanced theory examples completed!")
    print()
    print("Note: Some advanced theory features may require additional Python")
    print("bindings to be fully functional. This example demonstrates the")
    print("intended usage patterns based on the specification.")


if __name__ == "__main__":
    main()
