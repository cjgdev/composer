#!/usr/bin/env python3
"""
Roman Numeral Analysis and Chord Graphics

This example demonstrates Roman numeral analysis functionality:
- Generating Roman numeral representations from chords
- Scale degree analysis in various musical contexts
- Figured bass notation and chord quality symbols
- Applied chord notation (/V, /vi, etc.)
- Borrowed chord analysis from different modes

Based on the Composer specification: chord-theory-core.spec
"""

import composer


def demonstrate_basic_roman_numerals() -> None:
    """Show basic Roman numeral generation for common chords."""
    print("=== Basic Roman Numeral Analysis ===")

    # Create a major scale for context
    major_scale = composer.ScaleFingerprint.major()
    print(f"Analyzing in C Major scale: {major_scale}")
    print()

    # Common diatonic chords in major
    diatonic_chords = [
        (composer.Chord(1, 5), "I"),  # Tonic major
        (composer.Chord(2, 5), "ii"),  # Supertonic minor
        (composer.Chord(3, 5), "iii"),  # Mediant minor
        (composer.Chord(4, 5), "IV"),  # Subdominant major
        (composer.Chord(5, 5), "V"),  # Dominant major
        (composer.Chord(6, 5), "vi"),  # Submediant minor
        (composer.Chord(7, 5), "vii°"),  # Leading tone diminished
    ]

    print("Diatonic Roman Numerals:")
    for chord, expected_numeral in diatonic_chords:
        try:
            # Get Roman numeral representation
            graphic = composer.get_relative_chord_graphic(chord, major_scale)

            print(f"  {chord}")
            print(f"    Roman numeral: {graphic.symbol}")
            print(f"    Expected: {expected_numeral}")
            print(f"    Quality: {graphic.quality}")
            print(f"    Figured bass: {graphic.figured_bass}")
            print()

        except AttributeError:
            # If function doesn't exist, use available analysis
            print(f"  {chord}")
            print(f"    Expected: {expected_numeral}")
            try:
                complexity = composer.get_chord_complexity(chord)
                print(f"    Complexity: {complexity:.2f}")
            except AttributeError:
                pass
            print()


def demonstrate_seventh_chords() -> None:
    """Show Roman numeral analysis for seventh chords."""
    print("=== Seventh Chord Roman Numerals ===")

    major_scale = composer.ScaleFingerprint.major()

    # Seventh chords built on each scale degree
    seventh_chords = [
        (composer.Chord(1, 7), "IM7"),  # Major 7th
        (composer.Chord(2, 7), "ii7"),  # Minor 7th
        (composer.Chord(3, 7), "iii7"),  # Minor 7th
        (composer.Chord(4, 7), "IVM7"),  # Major 7th
        (composer.Chord(5, 7), "V7"),  # Dominant 7th
        (composer.Chord(6, 7), "vi7"),  # Minor 7th
        (composer.Chord(7, 7), "viiø7"),  # Half-diminished 7th
    ]

    print("Seventh Chord Analysis:")
    for chord, expected_numeral in seventh_chords:
        try:
            graphic = composer.get_relative_chord_graphic(chord, major_scale)

            print(f"  {chord}")
            print(f"    Roman numeral: {graphic.symbol}")
            print(f"    Expected: {expected_numeral}")
            print(f"    Quality: {graphic.quality}")
            print(f"    Figured bass: {graphic.figured_bass}")

        except AttributeError:
            # Fallback analysis
            print(f"  {chord}")
            print(f"    Expected: {expected_numeral}")
            try:
                stable_degrees = composer.get_stable_scale_degrees(chord, major_scale)
                print(f"    Scale degrees: {stable_degrees}")
            except AttributeError:
                pass

        print()


def demonstrate_chord_inversions() -> None:
    """Show figured bass notation for chord inversions."""
    print("=== Chord Inversions and Figured Bass ===")

    major_scale = composer.ScaleFingerprint.major()

    # Test different inversions
    base_chord_root = 1  # C major chord
    inversions = [
        (0, "root position", ""),
        (1, "first inversion", "6"),
        (2, "second inversion", "64"),
    ]

    for inversion, name, expected_bass in inversions:
        chord = composer.Chord(base_chord_root, 5, inversion=inversion)

        print(f"C Major - {name}:")
        print(f"  Chord: {chord}")
        print(f"  Inversion: {inversion}")

        try:
            graphic = composer.get_relative_chord_graphic(chord, major_scale)
            print(f"  Roman numeral: {graphic.symbol}")
            print(f"  Figured bass: {graphic.figured_bass}")
            print(f"  Expected bass: {expected_bass}")

        except AttributeError:
            print(f"  Expected bass: {expected_bass}")

        print()

    # Seventh chord inversions
    print("Seventh Chord Inversions:")
    seventh_inversions = [
        (0, "root position", "7"),
        (1, "first inversion", "65"),
        (2, "second inversion", "43"),
        (3, "third inversion", "42"),
    ]

    for inversion, name, expected_bass in seventh_inversions:
        chord = composer.Chord(5, 7, inversion=inversion)  # G7 chord

        print(f"G7 - {name}:")
        print(f"  Chord: {chord}")

        try:
            graphic = composer.get_relative_chord_graphic(chord, major_scale)
            print(f"  Roman numeral: {graphic.symbol}")
            print(f"  Figured bass: {graphic.figured_bass}")
            print(f"  Expected bass: {expected_bass}")

        except AttributeError:
            print(f"  Expected bass: {expected_bass}")

        print()


def demonstrate_applied_chords() -> None:
    """Show applied chord analysis and notation."""
    print("=== Applied Chord Analysis ===")

    major_scale = composer.ScaleFingerprint.major()

    # Common applied chords
    applied_chords = [
        (composer.Chord(5, 7, applied=5), "V7/V", "Secondary dominant of V"),
        (composer.Chord(5, 7, applied=6), "V7/vi", "Secondary dominant of vi"),
        (composer.Chord(5, 7, applied=2), "V7/ii", "Secondary dominant of ii"),
        (composer.Chord(5, 7, applied=4), "V7/IV", "Secondary dominant of IV"),
    ]

    print("Applied Chord Notation:")
    for chord, expected_notation, description in applied_chords:
        print(f"  {description}:")
        print(f"    Chord: {chord}")
        print(f"    Applied to: {chord.applied}")
        print(f"    Expected notation: {expected_notation}")

        try:
            graphic = composer.get_relative_chord_graphic(chord, major_scale)
            print(f"    Roman numeral: {graphic.symbol}")
            print(f"    Applied notation: {graphic.applied}")

        except AttributeError:
            # Check if chord is applied
            try:
                is_applied = composer.is_valid_tri_sub(chord, "major")
                print(f"    Valid tritone sub: {is_applied}")
            except AttributeError:
                pass

        print()


def demonstrate_borrowed_chords() -> None:
    """Show borrowed chord analysis from parallel modes."""
    print("=== Borrowed Chord Analysis ===")

    major_scale = composer.ScaleFingerprint.major()
    minor_scale = composer.ScaleFingerprint.minor()

    # Common borrowed chords in major from parallel minor
    borrowed_chords = [
        (composer.Chord(6, 5, borrowed="minor"), "♭VI", "Borrowed from minor"),
        (composer.Chord(7, 5, borrowed="minor"), "♭VII", "Borrowed from minor"),
        (composer.Chord(3, 5, borrowed="minor"), "♭III", "Borrowed from minor"),
        (composer.Chord(2, 5, borrowed="minor"), "ii°", "Borrowed from minor"),
    ]

    print("Borrowed Chords in Major:")
    for chord, expected_notation, description in borrowed_chords:
        print(f"  {description}:")
        print(f"    Chord: {chord}")
        print(f"    Borrowed from: {chord.borrowed}")
        print(f"    Expected notation: {expected_notation}")

        try:
            graphic = composer.get_relative_chord_graphic(chord, major_scale)
            print(f"    Roman numeral: {graphic.symbol}")
            print(f"    Borrowed indication: {graphic.borrowed}")

        except AttributeError:
            # Alternative analysis using scale comparison
            print(f"    In major scale: {major_scale}")
            print(f"    In minor scale: {minor_scale}")

        print()


def demonstrate_altered_chords() -> None:
    """Show analysis of chords with alterations."""
    print("=== Altered Chord Analysis ===")

    major_scale = composer.ScaleFingerprint.major()

    # Create chords with alterations
    altered_chords = [
        ("Neapolitan sixth", composer.Chord(2, 5)),  # bII6
        ("Augmented sixth", composer.Chord(6, 5)),  # Augmented sixth chord
        ("Diminished seventh", composer.Chord(7, 7)),  # vii°7
    ]

    # Add alterations manually since we may not have direct alteration support
    print("Altered and Special Chords:")
    for name, base_chord in altered_chords:
        print(f"  {name}:")
        print(f"    Base chord: {base_chord}")

        # Show alterations if available
        try:
            print(f"    Alterations: {base_chord.alterations}")
            print(f"    Suspensions: {base_chord.suspensions}")

            graphic = composer.get_relative_chord_graphic(base_chord, major_scale)
            print(f"    Roman numeral: {graphic.symbol}")
            print(f"    Alterations display: {graphic.alterations}")
            print(f"    Suspensions display: {graphic.suspensions}")

        except AttributeError:
            complexity = composer.get_chord_complexity(base_chord)
            print(f"    Complexity: {complexity:.2f}")

        print()


def demonstrate_mode_analysis() -> None:
    """Show Roman numeral analysis in different modes."""
    print("=== Modal Roman Numeral Analysis ===")

    # Test different modes
    modes = [
        ("Major (Ionian)", composer.ScaleFingerprint.major()),
        ("Minor (Aeolian)", composer.ScaleFingerprint.minor()),
        ("Dorian", composer.ScaleFingerprint.dorian()),
        ("Mixolydian", composer.ScaleFingerprint.mixolydian()),
    ]

    test_chord = composer.Chord(5, 5)  # V chord

    print(f"Analyzing {test_chord} in different modes:")
    print()

    for mode_name, scale in modes:
        print(f"  In {mode_name}:")
        print(f"    Scale: {scale}")

        try:
            graphic = composer.get_relative_chord_graphic(test_chord, scale)
            print(f"    Roman numeral: {graphic.symbol}")
            print(f"    Quality: {graphic.quality}")

        except AttributeError:
            # Fallback analysis
            try:
                stable_degrees = composer.get_stable_scale_degrees(test_chord, scale)
                print(f"    Scale degrees: {stable_degrees}")
            except AttributeError:
                complexity = composer.get_chord_complexity(test_chord)
                print(f"    Complexity: {complexity:.2f}")

        print()


def demonstrate_scale_degree_functions() -> None:
    """Show scale degree functional analysis."""
    print("=== Scale Degree Functional Analysis ===")

    major_scale = composer.ScaleFingerprint.major()

    # Scale degree functions
    functions = {
        1: "Tonic",
        2: "Supertonic",
        3: "Mediant",
        4: "Subdominant",
        5: "Dominant",
        6: "Submediant",
        7: "Leading Tone",
    }

    print("Scale Degree Functions in Major:")
    for degree, function_name in functions.items():
        chord = composer.Chord(degree, 5)  # Build triad on each degree

        print(f"  {degree}. {function_name}:")
        print(f"    Chord: {chord}")
        print(f"    Function: {function_name}")

        try:
            # Get stable scale degrees
            stable_degrees = composer.get_stable_scale_degrees(chord, major_scale)
            print(f"    Scale degrees: {stable_degrees}")

            # Get relative scale degrees
            relative_degrees = composer.get_relative_scale_degrees(chord)
            print(f"    Relative degrees: {relative_degrees}")

        except AttributeError:
            # Use available analysis
            complexity = composer.get_chord_complexity(chord)
            print(f"    Complexity: {complexity:.2f}")

        print()


def main() -> None:
    """Run all Roman numeral analysis demonstration functions."""
    print("Composer Library - Roman Numeral Analysis Examples")
    print("=" * 60)
    print()

    demonstrate_basic_roman_numerals()
    demonstrate_seventh_chords()
    demonstrate_chord_inversions()
    demonstrate_applied_chords()
    demonstrate_borrowed_chords()
    demonstrate_altered_chords()
    demonstrate_mode_analysis()
    demonstrate_scale_degree_functions()

    print("All Roman numeral analysis examples completed!")
    print()
    print("Note: Some advanced features may require additional Python bindings")
    print(
        "to be fully functional. This example demonstrates the intended usage patterns."
    )


if __name__ == "__main__":
    main()
