#!/usr/bin/env python3
"""
Scale Fingerprints and Music Theory

This example demonstrates scale fingerprint operations:
- Creating and working with scale fingerprints
- Generating scales (major, minor, modes)
- Scale analysis and comparison
- Roman numeral analysis

Based on the Composer specification: chord-theory-core.spec
"""

import composer


def demonstrate_scale_creation() -> None:
    """Show basic scale fingerprint creation."""
    print("=== Scale Fingerprint Creation ===")

    # Create a C major scale using static method
    c_major = composer.ScaleFingerprint.major()

    print(f"C Major scale fingerprint: {c_major}")
    print(f"Scale degrees: {c_major.scale_degrees}")
    print(f"Chromatic notes: {c_major.chromatic_notes}")
    print()

    # Create a natural minor scale
    a_minor = composer.ScaleFingerprint.minor()

    print(f"A Minor scale fingerprint: {a_minor}")
    print(f"Scale degrees: {a_minor.scale_degrees}")
    print()


def demonstrate_borrowed_scales() -> None:
    """Show working with borrowed scale objects."""
    print("=== Borrowed Scale Operations ===")

    # Create borrowed scale objects
    dorian_scale = composer.BorrowedScale("dorian")
    print(f"Dorian borrowed scale: {dorian_scale}")

    # Create different scale types
    major_scale = composer.ScaleFingerprint.major()
    minor_scale = composer.ScaleFingerprint.minor()
    dorian_mode = composer.ScaleFingerprint.dorian()
    mixolydian_mode = composer.ScaleFingerprint.mixolydian()

    print(f"Major scale: {major_scale}")
    print(f"Minor scale: {minor_scale}")
    print(f"Dorian mode: {dorian_mode}")
    print(f"Mixolydian mode: {mixolydian_mode}")
    print()

    # Compare scales
    print("Scale Comparison:")
    print(f"  Major == Minor: {major_scale == minor_scale}")
    print(f"  Major == Dorian: {major_scale == dorian_mode}")
    print(f"  Dorian == Mixolydian: {dorian_mode == mixolydian_mode}")
    print()


def demonstrate_mode_generation() -> None:
    """Generate and analyze different musical modes."""
    print("=== Musical Mode Generation ===")

    # Generate available modes using static methods
    modes = [
        ("Major (Ionian)", composer.ScaleFingerprint.major()),
        ("Minor (Aeolian)", composer.ScaleFingerprint.minor()),
        ("Dorian", composer.ScaleFingerprint.dorian()),
        ("Mixolydian", composer.ScaleFingerprint.mixolydian()),
    ]

    for mode_name, scale in modes:
        print(f"{mode_name}:")
        print(f"  Scale: {scale}")
        print(f"  Scale degrees: {scale.scale_degrees}")
        print(f"  Chromatic notes: {scale.chromatic_notes}")
        print()


def demonstrate_scale_encoding() -> None:
    """Show scale fingerprint encoding and decoding."""
    print("=== Scale Encoding/Decoding ===")

    # Create a complex scale (harmonic minor)
    harmonic_minor = [
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
    scale = composer.ScaleFingerprint(harmonic_minor)

    print(f"Original scale: {scale}")
    print(f"Scale degrees: {scale.scale_degrees}")
    print(f"Chromatic notes: {scale.chromatic_notes}")

    # Encode the scale fingerprint
    try:
        encoded = composer.py_scale40_encode(harmonic_minor)
        print(f"Encoded representation: {encoded}")

        # Decode it back
        decoded = composer.py_scale40_decode(encoded)
        print(f"Decoded pattern: {decoded}")

        # Verify roundtrip
        if decoded == harmonic_minor:
            print("✓ Scale encoding roundtrip successful")
        else:
            print("✗ Scale encoding roundtrip failed")

    except Exception as e:
        print(f"Encoding error: {e}")

    print()


def demonstrate_scale_comparison() -> None:
    """Compare different scales and their relationships."""
    print("=== Scale Comparison ===")

    # Create several scales
    major = [
        True,
        False,
        True,
        False,
        True,
        True,
        False,
        True,
        False,
        True,
        False,
        True,
    ]
    minor = [
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
        True,
        False,
    ]
    harmonic_minor = [
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
    melodic_minor = [
        True,
        False,
        True,
        True,
        False,
        True,
        False,
        True,
        False,
        True,
        False,
        True,
    ]

    scales = [
        ("Major", major),
        ("Natural Minor", minor),
        ("Harmonic Minor", harmonic_minor),
        ("Melodic Minor", melodic_minor),
    ]

    scale_objects = []
    for name, pattern in scales:
        scale = composer.ScaleFingerprint(pattern)
        scale_objects.append((name, scale))
        print(f"{name}: {scale.scale_degrees}")

    print()

    # Compare scales
    print("Scale Relationships:")
    for i, (name1, scale1) in enumerate(scale_objects):
        for _j, (name2, scale2) in enumerate(scale_objects[i + 1 :], i + 1):
            # Count common tones
            common_tones = len(
                set(scale1.chromatic_notes) & set(scale2.chromatic_notes)
            )
            total_tones = len(set(scale1.chromatic_notes) | set(scale2.chromatic_notes))
            similarity = common_tones / total_tones if total_tones > 0 else 0

            print(
                f"  {name1} vs {name2}: "
                f"{common_tones}/{len(scale1.chromatic_notes)} common tones "
                f"({similarity:.1%} similarity)"
            )

    print()


def demonstrate_scale_theory() -> None:
    """Demonstrate advanced scale theory concepts."""
    print("=== Advanced Scale Theory ===")

    # Create scales for Roman numeral analysis
    major_scale = [
        True,
        False,
        True,
        False,
        True,
        True,
        False,
        True,
        False,
        True,
        False,
        True,
    ]
    scale = composer.ScaleFingerprint(major_scale)

    print("C Major Scale Analysis:")
    print(f"Scale: {scale}")

    # Create chords built on each scale degree
    scale_chords = []
    for degree in scale.scale_degrees:
        # Build a triad on each scale degree (simplified)
        chord = composer.Chord(degree, 5)  # Major triad on each degree
        scale_chords.append(chord)

    print("\nChords built on scale degrees:")
    roman_numerals = ["I", "ii", "iii", "IV", "V", "vi", "vii°"]

    for i, chord in enumerate(scale_chords[:7]):  # First 7 chords
        complexity = composer.get_chord_complexity(chord)
        print(f"  {roman_numerals[i]}: {chord} (complexity: {complexity:.2f})")

    print()


def main() -> None:
    """Run all scale fingerprint demonstration functions."""
    print("Composer Library - Scale Fingerprint Examples")
    print("=" * 55)
    print()

    demonstrate_scale_creation()
    demonstrate_borrowed_scales()
    demonstrate_mode_generation()
    demonstrate_scale_encoding()
    demonstrate_scale_comparison()
    demonstrate_scale_theory()

    print("All scale examples completed successfully!")


if __name__ == "__main__":
    main()
