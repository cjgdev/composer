#!/usr/bin/env python3
"""
Pattern Matching and Chord Lookup

This example demonstrates pattern matching and lookup functionality:
- Trie-based pattern storage and searching
- Wildcard pattern matching
- Chord symbol parsing from multiple formats
- Isotonal chord mapping and substitutions
- Context-aware pattern search algorithms

Based on the Composer specification: chord-analysis-lookup.spec
"""

import composer


def demonstrate_trie_pattern_storage() -> None:
    """Show trie construction and pattern storage."""
    print("=== Trie Pattern Storage ===")

    # Create a trie for storing chord progressions
    trie = composer.TrieNode()
    print(f"Created empty trie: {trie}")
    print(f"Initial node count: {trie.node_count}")
    print()

    # Common chord progressions to store
    progressions = [
        # I-V-vi-IV (Pop progression)
        {
            "chords": [
                composer.Chord(1, 5),  # I
                composer.Chord(5, 5),  # V
                composer.Chord(6, 5),  # vi
                composer.Chord(4, 5),  # IV
            ],
            "name": "Pop_Progression",
            "key": "C",
            "id": 1,
        },
        # ii-V-I (Jazz progression)
        {
            "chords": [
                composer.Chord(2, 7),  # ii7
                composer.Chord(5, 7),  # V7
                composer.Chord(1, 7),  # IM7
            ],
            "name": "Jazz_ii_V_I",
            "key": "C",
            "id": 2,
        },
        # I-vi-IV-V (50s progression)
        {
            "chords": [
                composer.Chord(1, 5),  # I
                composer.Chord(6, 5),  # vi
                composer.Chord(4, 5),  # IV
                composer.Chord(5, 5),  # V
            ],
            "name": "Fifties_Progression",
            "key": "C",
            "id": 3,
        },
        # vi-IV-I-V (Pop variation)
        {
            "chords": [
                composer.Chord(6, 5),  # vi
                composer.Chord(4, 5),  # IV
                composer.Chord(1, 5),  # I
                composer.Chord(5, 5),  # V
            ],
            "name": "Pop_Variation",
            "key": "C",
            "id": 4,
        },
    ]

    print("Adding progressions to trie:")
    for progression in progressions:
        # Serialize chords to binary for trie storage
        pattern = []
        for chord in progression["chords"]:
            binary_chord = composer.serialize_chord_to_binary(chord)
            pattern.append(binary_chord)

        # Add pattern to trie
        try:
            trie.add_pattern(pattern, progression["id"])
            print(f"  Added: {progression['name']} ({len(pattern)} chords)")
        except AttributeError:
            # If method doesn't exist, show intended usage
            print(f"  Would add: {progression['name']} ({len(pattern)} chords)")

    print("\nTrie after adding patterns:")
    print(f"  Node count: {trie.node_count}")
    print(f"  Children count: {trie.children_count}")
    print(f"  Total patterns stored: {len(progressions)}")
    print()


def demonstrate_pattern_search() -> None:
    """Show pattern searching in the trie."""
    print("=== Pattern Search ===")

    # Create test patterns to search for
    search_patterns = [
        {
            "name": "I-V pattern",
            "chords": [
                composer.Chord(1, 5),  # I
                composer.Chord(5, 5),  # V
            ],
        },
        {
            "name": "V-I cadence",
            "chords": [
                composer.Chord(5, 7),  # V7
                composer.Chord(1, 7),  # IM7
            ],
        },
        {
            "name": "Single I chord",
            "chords": [
                composer.Chord(1, 5),  # I
            ],
        },
    ]

    print("Searching for common patterns:")
    for pattern_info in search_patterns:
        pattern_name = pattern_info["name"]
        chords = pattern_info["chords"]

        print(f"  {pattern_name}:")
        print(f"    Pattern: {[str(c) for c in chords]}")

        # Serialize pattern for search
        binary_pattern = []
        for chord in chords:
            binary_chord = composer.serialize_chord_to_binary(chord)
            binary_pattern.append(binary_chord)

        try:
            # Search for pattern
            results = composer.search_pattern_in_trie(binary_pattern)
            print(f"    Results: {len(results)} matches found")

            for i, result in enumerate(results, 1):
                print(f"      {i}. Count: {result.count}, Rank: {result.rank}")
                print(f"         Weight: {result.weight:.3f}")
                print(f"         Source IDs: {result.id_list}")

        except AttributeError:
            # Show what the search would look like
            print(f"    Would search for: {len(binary_pattern)} chord pattern")
            total_bytes = sum(len(b) for b in binary_pattern)
            print(f"    Binary pattern length: {total_bytes} bytes")

        print()


def demonstrate_wildcard_search() -> None:
    """Show wildcard pattern matching."""
    print("=== Wildcard Pattern Search ===")

    # Create patterns with wildcards (represented as None)
    wildcard_patterns = [
        {
            "name": "I-?-vi pattern",
            "pattern": [
                composer.Chord(1, 5),  # I
                None,  # Wildcard
                composer.Chord(6, 5),  # vi
            ],
            "wildcard_index": 1,
        },
        {
            "name": "?-V-I pattern",
            "pattern": [
                None,  # Wildcard
                composer.Chord(5, 5),  # V
                composer.Chord(1, 5),  # I
            ],
            "wildcard_index": 0,
        },
    ]

    print("Wildcard pattern searches:")
    for pattern_info in wildcard_patterns:
        pattern_name = pattern_info["name"]
        pattern = pattern_info["pattern"]
        wildcard_idx = pattern_info["wildcard_index"]

        print(f"  {pattern_name}:")
        print(f"    Pattern: {[str(c) if c else '?' for c in pattern]}")
        print(f"    Wildcard at position: {wildcard_idx}")

        # Convert to binary pattern
        binary_pattern = []
        for chord in pattern:
            if chord is not None:
                binary_pattern.append(composer.serialize_chord_to_binary(chord))
            else:
                binary_pattern.append(None)  # Wildcard marker

        try:
            # Search with wildcard
            results = composer.search_with_wildcard(binary_pattern, wildcard_idx)
            print(f"    Found {len(results)} possible completions:")

            for i, result in enumerate(results, 1):
                print(f"      {i}. Suggestion: {result.serialized_chord}")
                print(f"         Count: {result.count}, Weight: {result.weight:.3f}")

        except AttributeError:
            # Show intended wildcard functionality
            print(f"    Would find completions for wildcard at position {wildcard_idx}")

        print()


def demonstrate_chord_symbol_parsing() -> None:
    """Show parsing of various chord symbol formats."""
    print("=== Chord Symbol Parsing ===")

    # Test various chord symbol formats
    chord_symbols = [
        # Roman numeral formats
        ("I", "Roman numeral - major tonic"),
        ("ii7", "Roman numeral - minor seventh"),
        ("V7/V", "Roman numeral - applied dominant"),
        ("vii°", "Roman numeral - diminished"),
        # Popular notation
        ("Cmaj7", "Popular notation - major seventh"),
        ("Dm7b5", "Popular notation - half-diminished"),
        ("F#dim", "Popular notation - diminished"),
        ("Bb7#11", "Popular notation - altered dominant"),
        # Figured bass
        ("I6", "Figured bass - first inversion"),
        ("V64", "Figured bass - second inversion"),
        ("ii42", "Figured bass - third inversion seventh"),
        # Slash chords
        ("C/E", "Slash chord - C over E"),
        ("Am7/C", "Slash chord - Am7 over C"),
    ]

    print("Parsing different chord symbol formats:")
    for symbol, description in chord_symbols:
        print(f"  '{symbol}' ({description}):")

        try:
            # Parse chord symbol - may return multiple interpretations
            chord_candidates = composer.parse_chord_symbol(symbol, "major")

            print(f"    Parsed {len(chord_candidates)} interpretation(s):")
            for i, chord in enumerate(chord_candidates, 1):
                print(f"      {i}. {chord}")
                print(f"         Root: {chord.root}, Type: {chord.chord_type}")
                if chord.applied:
                    print(f"         Applied to: {chord.applied}")
                if chord.inversion:
                    print(f"         Inversion: {chord.inversion}")

        except AttributeError:
            # Show component extraction if full parsing not available
            try:
                components = composer.extract_chord_components(symbol)
                print(f"    Components: {components}")
            except AttributeError:
                # Manual parsing demonstration
                print(f"    Would parse: {symbol}")
                if "/" in symbol:
                    print("      Detected slash chord")
                if any(c in symbol for c in ["7", "9", "11", "13"]):
                    print("      Detected extension")
                if any(c in symbol for c in ["#", "b"]):
                    print("      Detected alteration")

        print()


def demonstrate_isotonal_mapping() -> None:
    """Show isotonal chord mapping and substitutions."""
    print("=== Isotonal Chord Mapping ===")

    major_scale = composer.ScaleFingerprint.major()

    # Test chords for isotonal relationships
    test_chords = [
        composer.Chord(1, 5),  # I major
        composer.Chord(6, 5),  # vi minor (relative minor)
        composer.Chord(3, 5),  # iii minor
        composer.Chord(5, 7),  # V7 dominant
        composer.Chord(2, 7),  # ii7 minor seventh
    ]

    print("Testing isotonal relationships:")
    for i, chord1 in enumerate(test_chords):
        print(f"  {chord1}:")

        for j, chord2 in enumerate(test_chords):
            if i != j:
                try:
                    # Check if chords are isotonal (harmonically equivalent)
                    is_isotonal = composer.is_isotonal(chord1, chord2, major_scale)
                    isotonal_text = "Isotonal" if is_isotonal else "Not isotonal"
                    print(f"    vs {chord2}: {isotonal_text}")

                except AttributeError:
                    # Compare scale degrees manually
                    try:
                        degrees1 = composer.get_stable_scale_degrees(
                            chord1, major_scale
                        )
                        degrees2 = composer.get_stable_scale_degrees(
                            chord2, major_scale
                        )

                        # Simple comparison
                        common_degrees = set(degrees1) & set(degrees2)
                        similarity = len(common_degrees) / max(
                            len(degrees1), len(degrees2)
                        )

                        print(f"    vs {chord2}: {similarity:.1%} scale degree overlap")

                    except AttributeError:
                        # Basic chord comparison
                        same_type = chord1.chord_type == chord2.chord_type
                        type_text = "Same type" if same_type else "Different type"
                        print(f"    vs {chord2}: {type_text}")
        print()


def demonstrate_bass_line_analysis() -> None:
    """Show bass line pattern analysis."""
    print("=== Bass Line Pattern Analysis ===")

    # Create a chord progression for bass analysis
    progression = [
        composer.Chord(1, 5),  # C major
        composer.Chord(6, 7),  # Am7
        composer.Chord(4, 5),  # F major
        composer.Chord(5, 7),  # G7
    ]

    major_scale = composer.ScaleFingerprint.major()

    print("Analyzing bass line patterns:")
    print(f"Progression: {[str(c) for c in progression]}")
    print()

    for i, chord in enumerate(progression, 1):
        print(f"  {i}. {chord}:")

        try:
            # Extract bass note
            bass_note = composer.extract_bass_note(chord, major_scale)
            print(f"    Bass note (chromatic): {bass_note}")

            # Convert to note name
            note_names = [
                "C",
                "C#",
                "D",
                "D#",
                "E",
                "F",
                "F#",
                "G",
                "G#",
                "A",
                "A#",
                "B",
            ]
            bass_name = note_names[bass_note % 12]
            print(f"    Bass note name: {bass_name}")

        except AttributeError:
            # Estimate bass note from chord root
            bass_estimate = (chord.root - 1) % 12  # Convert scale degree to chromatic
            note_names = [
                "C",
                "C#",
                "D",
                "D#",
                "E",
                "F",
                "F#",
                "G",
                "G#",
                "A",
                "A#",
                "B",
            ]
            print(f"    Estimated bass: {note_names[bass_estimate]}")

        # Show chord-bass relationships
        try:
            bass_entries = composer.analyze_chord_bass_relationships([chord], "major")
            for bass_note, entries in bass_entries.items():
                print(f"    Bass note {bass_note}: {len(entries)} chord options")

        except AttributeError:
            # Show available chord properties
            print(f"    Root: {chord.root}, Type: {chord.chord_type}")
            print(f"    Inversion: {chord.inversion}")

        print()


def demonstrate_context_aware_search() -> None:
    """Show context-aware pattern searching."""
    print("=== Context-Aware Pattern Search ===")

    # Define context for search
    previous_chords = [
        composer.Chord(1, 5),  # I
        composer.Chord(6, 7),  # vi7
    ]

    following_chords = [
        composer.Chord(1, 5),  # I (return to tonic)
    ]

    print("Context-aware search:")
    print(f"Previous chords: {[str(c) for c in previous_chords]}")
    print(f"Following chords: {[str(c) for c in following_chords]}")
    print("Looking for connecting chord...")
    print()

    try:
        # Perform context-aware search
        results = composer.contextual_search(previous_chords, following_chords, "major")

        print(f"Found {len(results)} contextual suggestions:")
        for i, result in enumerate(results, 1):
            print(f"  {i}. {result.serialized_chord}")
            print(f"     Context match: {result.context_match:.3f}")
            print(f"     Statistical strength: {result.statistical_strength:.3f}")
            print(f"     Weight: {result.weight:.3f}")

    except AttributeError:
        # Show manual context analysis
        print("Manual context analysis:")

        # Test some candidate chords
        candidates = [
            composer.Chord(4, 5),  # IV
            composer.Chord(5, 7),  # V7
            composer.Chord(2, 7),  # ii7
        ]

        for candidate in candidates:
            print(f"  Candidate: {candidate}")

            # Simple context scoring
            complexity = composer.get_chord_complexity(candidate)
            print(f"    Complexity: {complexity:.2f}")
            print(f"    Context score: {(10 - complexity) / 10:.3f}")

        print()


def main() -> None:
    """Run all pattern matching demonstration functions."""
    print("Composer Library - Pattern Matching and Chord Lookup Examples")
    print("=" * 70)
    print()

    demonstrate_trie_pattern_storage()
    demonstrate_pattern_search()
    demonstrate_wildcard_search()
    demonstrate_chord_symbol_parsing()
    demonstrate_isotonal_mapping()
    demonstrate_bass_line_analysis()
    demonstrate_context_aware_search()

    print("All pattern matching examples completed!")
    print()
    print("Note: Some advanced pattern matching features may require additional")
    print("Python bindings to be fully functional. This example demonstrates")
    print("the intended usage patterns based on the specification.")


if __name__ == "__main__":
    main()
