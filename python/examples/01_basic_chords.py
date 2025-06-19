#!/usr/bin/env python3
"""
Basic Chord Creation and Manipulation

This example demonstrates fundamental chord operations in the Composer library:
- Creating chords with different roots, types, and inversions
- Examining chord properties
- Converting chords to different representations
- Working with chord characteristics

Based on the Composer specification: chord-theory-core.spec
"""

import composer


def demonstrate_chord_creation() -> None:
    """Show basic chord creation with different parameters."""
    print("=== Basic Chord Creation ===")

    # Create a simple C major chord (root=0, type=5)
    c_major = composer.Chord(1, 5)  # C major triad
    print(f"C Major: {c_major}")
    print(f"  Root: {c_major.root}")
    print(f"  Type: {c_major.chord_type}")
    print(f"  Inversion: {c_major.inversion}")
    print()

    # Create chord with inversion
    c_major_first = composer.Chord(1, 5, inversion=1)
    print(f"C Major (1st inversion): {c_major_first}")
    print(f"  Inversion: {c_major_first.inversion}")
    print()

    # Create more complex chords
    d_minor7 = composer.Chord(2, 7)  # D minor 7
    print(f"D Minor 7: {d_minor7}")
    print()

    # Create chord with extensions
    g7_sus4 = composer.Chord(5, 7)  # G7 (using valid chord type)
    print(f"G7: {g7_sus4}")
    print()


def demonstrate_chord_properties() -> None:
    """Explore various chord properties and methods."""
    print("=== Chord Properties ===")

    chord = composer.Chord(4, 5)  # E major
    print(f"Chord: {chord}")

    # Check various properties
    print(f"  Root note: {chord.root}")
    print(f"  Chord type: {chord.chord_type}")
    print(f"  Inversion: {chord.inversion}")
    print(f"  Alterations: {chord.alterations}")
    print(f"  Suspensions: {chord.suspensions}")
    print(f"  Adds: {chord.adds}")
    print(f"  Omits: {chord.omits}")
    print(f"  Applied: {chord.applied}")
    print(f"  Is rest: {chord.is_rest}")
    print()

    # Create chord with alterations
    altered_chord = composer.Chord(1, 5)
    altered_chord.add_suspension(4)
    print(f"C Major with sus4: {altered_chord}")
    print()


def demonstrate_chord_serialization() -> None:
    """Show chord serialization to binary and hex formats."""
    print("=== Chord Serialization ===")

    chord = composer.Chord(3, 7)  # Eb minor 7
    print(f"Original chord: {chord}")

    # Serialize to binary
    binary_data = composer.serialize_chord_to_binary(chord)
    print(f"Binary data length: {len(binary_data)} bytes")

    # Convert to hex for readable representation
    hex_string = composer.chord_to_hex(chord)
    print(f"Hex representation: {hex_string}")

    # Deserialize back
    restored_chord = composer.chord_from_hex(hex_string)
    print(f"Restored chord: {restored_chord}")

    # Verify they match
    if (
        chord.root == restored_chord.root
        and chord.chord_type == restored_chord.chord_type
        and chord.inversion == restored_chord.inversion
    ):
        print("✓ Serialization roundtrip successful")
    else:
        print("✗ Serialization roundtrip failed")
    print()


def demonstrate_chord_comparison() -> None:
    """Show chord comparison and equality operations."""
    print("=== Chord Comparison ===")

    chord1 = composer.Chord(1, 5)  # C major
    chord2 = composer.Chord(1, 5)  # C major (identical)
    chord3 = composer.Chord(1, 5, inversion=1)  # C major first inversion
    chord4 = composer.Chord(2, 5)  # D major

    print(f"Chord 1: {chord1}")
    print(f"Chord 2: {chord2}")
    print(f"Chord 3: {chord3}")
    print(f"Chord 4: {chord4}")
    print()

    print(f"Chord 1 == Chord 2: {chord1 == chord2}")
    print(f"Chord 1 == Chord 3: {chord1 == chord3}")
    print(f"Chord 1 == Chord 4: {chord1 == chord4}")
    print()


def demonstrate_chord_theory() -> None:
    """Show music theory related chord operations."""
    print("=== Chord Theory ===")

    # Create a progression
    progression = [
        composer.Chord(1, 5),  # I major (C)
        composer.Chord(4, 5),  # IV major (F)
        composer.Chord(5, 5),  # V major (G)
        composer.Chord(1, 5),  # I major (C)
    ]

    print("Chord progression:")
    for i, chord in enumerate(progression, 1):
        complexity = composer.get_chord_complexity(chord)
        print(f"  {i}. {chord} (complexity: {complexity:.2f})")

    print()


def main() -> None:
    """Run all chord demonstration functions."""
    print("Composer Library - Basic Chord Examples")
    print("=" * 50)
    print()

    demonstrate_chord_creation()
    demonstrate_chord_properties()
    demonstrate_chord_serialization()
    demonstrate_chord_comparison()
    demonstrate_chord_theory()

    print("All examples completed successfully!")


if __name__ == "__main__":
    main()
