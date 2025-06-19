#!/usr/bin/env python3
"""
Advanced Serialization and Tokenization

This example demonstrates the serialization and tokenization features:
- Binary chord serialization (5-byte format)
- Musical tokenization for ML applications
- Timeline reconstruction and MIDI-like tokens
- Trie serialization for pattern storage
- Hash functions and data compression

Based on the Composer specification: data-processing-serialization.spec
"""

import composer


def demonstrate_binary_serialization() -> None:
    """Show binary chord serialization and deserialization."""
    print("=== Binary Chord Serialization ===")

    # Create various chords to serialize
    chords = [
        composer.Chord(1, 5),  # C major
        composer.Chord(7, 7),  # G minor 7
        composer.Chord(4, 9),  # E dominant 7
        composer.Chord(2, 11),  # D major 7
    ]

    print("Serializing chords to 5-byte binary format:")
    serialized_data = []

    for i, chord in enumerate(chords, 1):
        # Serialize to binary
        binary_data = composer.serialize_chord_to_binary(chord)
        hex_string = composer.chord_to_hex(chord)

        print(f"  {i}. {chord}")
        print(f"     Binary length: {len(binary_data)} bytes")
        print(f"     Hex: {hex_string}")

        serialized_data.append((chord, binary_data, hex_string))

    print()

    # Test deserialization
    print("Deserializing chords:")
    for i, (original, binary_data, hex_string) in enumerate(serialized_data, 1):
        # Deserialize from binary
        restored_binary = composer.deserialize_chord_from_binary(binary_data)
        restored_hex = composer.chord_from_hex(hex_string)

        print(f"  {i}. Original: {original}")
        print(f"     From binary: {restored_binary}")
        print(f"     From hex: {restored_hex}")

        # Verify roundtrip
        binary_match = (
            original.root == restored_binary.root
            and original.chord_type == restored_binary.chord_type
        )
        hex_match = (
            original.root == restored_hex.root
            and original.chord_type == restored_hex.chord_type
        )

        print(f"     Binary roundtrip: {'✓' if binary_match else '✗'}")
        print(f"     Hex roundtrip: {'✓' if hex_match else '✗'}")
        print()


def demonstrate_tokenization() -> None:
    """Show musical tokenization for ML applications."""
    print("=== Musical Tokenization ===")

    # Create a scale for tokenization context
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

    print(f"Using scale: {scale}")
    print()

    # Tokenize durations
    durations = [1.0, 0.5, 0.25, 0.75, 2.0]
    print("Duration tokenization:")

    for duration in durations:
        token = composer.py_tokenize_duration(duration)
        parsed = composer.py_parse_duration_token(token)

        print(f"  {duration} -> '{token}' -> {parsed}")
        roundtrip_ok = abs(duration - parsed) < 0.001
        print(f"    Roundtrip: {'✓' if roundtrip_ok else '✗'}")

    print()

    # Tokenize chords as raw chromatic clusters
    print("Chord tokenization as raw clusters:")
    chords = [
        composer.Chord(1, 5),  # C major
        composer.Chord(7, 7),  # G minor 7
        composer.Chord(4, 9),  # E dominant 7
    ]

    for chord in chords:
        try:
            token = composer.py_tokenize_chord_as_raw(chord, scale)
            print(f"  {chord} -> '{token}'")
        except Exception as e:
            print(f"  {chord} -> Error: {e}")

    print()


def demonstrate_note_structures() -> None:
    """Show Note and TokenLibrary structures."""
    print("=== Note Structures and Token Library ===")

    # Create various notes
    notes = [
        composer.Note(0, 4),  # C4
        composer.Note(4, 4),  # E4
        composer.Note(7, 4),  # G4
        composer.Note(0, 0, True),  # Rest
    ]

    print("Created notes:")
    for note in notes:
        print(f"  {note}")
        if not note.is_rest:
            print(f"    Scale degree: {note.scale_degree}, Octave: {note.octave}")

    print()

    # Create and use token library
    print("Token Library operations:")
    library = composer.TokenLibrary()
    print(f"Initial library size: {len(library)}")

    # Add some chord tokens
    test_chords = [
        (b"\x00\x05\x00\x00\x00", "C_MAJOR"),
        (b"\x07\x07\x00\x00\x00", "G_MINOR7"),
        (b"\x04\x09\x00\x00\x00", "E_DOM7"),
    ]

    for binary, token_name in test_chords:
        library.add_chord_token(token_name, binary)
        print(f"Added token: {token_name}")

    print(f"Library size after adding tokens: {len(library)}")

    # Resolve tokens back to binary
    for _, token_name in test_chords:
        try:
            resolved = library.resolve_chord_token(token_name)
            print(f"Resolved {token_name}: {len(resolved)} bytes")
        except Exception as e:
            print(f"Error resolving {token_name}: {e}")

    print()


def demonstrate_hash_functions() -> None:
    """Show hash and compression functions."""
    print("=== Hash Functions and Compression ===")

    # Test fast hash function
    test_strings = [
        "C major chord",
        "G minor seventh",
        "A complex jazz progression",
        "The same string",
        "The same string",  # Duplicate to test consistency
    ]

    print("Fast hash function:")
    hashes = []
    for string in test_strings:
        hash_value = composer.py_fast_hash(string)
        hashes.append(hash_value)
        print(f"  '{string}' -> {hash_value}")

    # Verify duplicate strings have same hash
    if hashes[-1] == hashes[-2]:
        print("  ✓ Duplicate strings have identical hashes")
    else:
        print("  ✗ Duplicate strings have different hashes")

    print()

    # Test fold hash function
    print("Fold hash function (combining hashes):")
    base_hash = composer.py_fast_hash("initial")
    print(f"Base hash: {base_hash}")

    additional_data = ["chord1", "chord2", "chord3"]
    combined_hash = base_hash

    for data in additional_data:
        combined_hash = composer.py_fold_hash(combined_hash, data)
        print(f"  + '{data}' -> {combined_hash}")

    print()

    # Test scale fingerprint encoding
    print("Scale fingerprint encoding:")
    test_scales = [
        (
            [
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
            ],
            "Major",
        ),
        (
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
                True,
                False,
            ],
            "Minor",
        ),
        (
            [
                True,
                True,
                False,
                True,
                True,
                False,
                True,
                True,
                True,
                False,
                True,
                False,
            ],
            "Chromatic subset",
        ),
    ]

    for pattern, name in test_scales:
        try:
            encoded = composer.py_scale40_encode(pattern)
            decoded = composer.py_scale40_decode(encoded)

            print(f"  {name}:")
            print(f"    Original:  {pattern}")
            print(f"    Encoded:   '{encoded}'")
            print(f"    Decoded:   {decoded}")
            print(f"    Roundtrip: {'✓' if pattern == decoded else '✗'}")
            print()
        except Exception as e:
            print(f"  {name}: Error - {e}")


def demonstrate_trie_operations() -> None:
    """Show trie serialization and pattern storage."""
    print("=== Trie Operations and Pattern Storage ===")

    # Create a trie and add patterns
    trie = composer.TrieNode()
    print(f"Created empty trie: {trie}")

    # Add some chord progression patterns
    patterns = [
        ([b"\x00\x05", b"\x07\x05", b"\x00\x05"], 1),  # I-V-I
        ([b"\x00\x05", b"\x05\x05", b"\x07\x05"], 2),  # I-IV-V
        ([b"\x09\x07", b"\x00\x05"], 3),  # vi-I
    ]

    print("Adding patterns to trie:")
    for pattern, pattern_id in patterns:
        trie.add_pattern(pattern, pattern_id)
        print(f"  Added pattern {pattern_id}: {len(pattern)} chords")

    print(f"Trie after adding patterns: {trie}")
    print(f"  Node count: {trie.node_count}")
    print(f"  Children: {trie.children_count}")
    print(f"  ID list: {trie.id_list}")
    print()

    # Search for patterns
    print("Searching for patterns:")
    search_patterns = [
        [b"\x00\x05", b"\x07\x05"],  # I-V (partial)
        [b"\x00\x05", b"\x05\x05"],  # I-IV (partial)
        [b"\x02\x07"],  # Unknown pattern
    ]

    for search_pattern in search_patterns:
        results = trie.search_patterns(search_pattern)
        print(f"  Pattern {search_pattern}: found {results}")

    print()

    # Test trie serialization
    print("Trie serialization:")
    try:
        serialized = composer.py_serialize_trie(trie)
        print(f"Serialized trie: {len(serialized)} bytes")

        # Validate binary format
        is_valid = composer.py_validate_binary_format(serialized)
        print(f"Binary format valid: {'✓' if is_valid else '✗'}")

        # Deserialize
        deserialized = composer.py_deserialize_trie(serialized, include_key_tonic=False)
        print(f"Deserialized trie: {deserialized}")

        # Compare basic properties
        print(f"Original node count: {trie.node_count}")
        print(f"Deserialized node count: {deserialized.node_count}")

    except Exception as e:
        print(f"Trie serialization error: {e}")


def demonstrate_token_validation() -> None:
    """Show token validation functions."""
    print("=== Token Validation ===")

    # Test various token formats
    test_tokens = [
        ("D1A2", "duration token"),
        ("R4C3", "raw note token"),
        ("O05", "octave token"),
        ("C1F2A3", "chord cluster token"),
        ("INVALID", "invalid token"),
        ("", "empty token"),
    ]

    print("Token validation tests:")
    for token, description in test_tokens:
        general_valid = composer.py_validate_token(token)
        duration_valid = composer.py_validate_duration_token(token)
        note_valid = composer.py_validate_raw_note_token(token)
        octave_valid = composer.py_validate_octave_token(token)
        cluster_valid = composer.py_validate_chord_cluster_token(token)

        print(f"  '{token}' ({description}):")
        print(f"    General: {'✓' if general_valid else '✗'}")
        print(f"    Duration: {'✓' if duration_valid else '✗'}")
        print(f"    Raw note: {'✓' if note_valid else '✗'}")
        print(f"    Octave: {'✓' if octave_valid else '✗'}")
        print(f"    Cluster: {'✓' if cluster_valid else '✗'}")
        print()


def demonstrate_ml_utilities() -> None:
    """Show ML-related utility functions."""
    print("=== ML Utility Functions ===")

    # Test chord vocabulary reduction
    print("Chord vocabulary reduction:")
    sample_chords = [
        b"\x00\x05\x00\x00\x00",  # C major
        b"\x07\x05\x00\x00\x00",  # G major
        b"\x05\x05\x00\x00\x00",  # F major
        b"\x09\x07\x00\x00\x00",  # A minor 7
        b"\x02\x07\x00\x00\x00",  # D minor 7
        b"\x04\x09\x00\x00\x00",  # E dominant 7
    ]

    print(f"Original vocabulary: {len(sample_chords)} chords")

    try:
        reduced = composer.py_reduce_chord_vocab(sample_chords, max_vocab=4)
        print(f"Reduced vocabulary: {len(reduced)} chords")

        for i, chord_bytes in enumerate(reduced, 1):
            print(f"  {i}. {len(chord_bytes)} bytes")

    except Exception as e:
        print(f"Vocabulary reduction error: {e}")

    print()

    # Test sequence augmentation
    print("Sequence augmentation with repetition:")
    original_sequence = ["C", "F", "G", "C"]
    min_tokens = 10

    augmented = composer.py_augment_with_repeated(original_sequence, min_tokens)
    print(f"Original sequence: {original_sequence} ({len(original_sequence)} tokens)")
    print(f"Augmented sequence: {augmented} ({len(augmented)} tokens)")
    print(f"Target minimum: {min_tokens} tokens")
    print()


def demonstrate_constants() -> None:
    """Show serialization constants and configuration."""
    print("=== Serialization Constants ===")

    constants = composer.get_serialization_constants()
    print("Available constants:")

    for key, value in constants.items():
        print(f"  {key}: {value}")

    print()


def main() -> None:
    """Run all serialization demonstration functions."""
    print("Composer Library - Advanced Serialization and Tokenization")
    print("=" * 70)
    print()

    demonstrate_binary_serialization()
    demonstrate_tokenization()
    demonstrate_note_structures()
    demonstrate_hash_functions()
    demonstrate_trie_operations()
    demonstrate_token_validation()
    demonstrate_ml_utilities()
    demonstrate_constants()

    print("All serialization examples completed successfully!")


if __name__ == "__main__":
    main()
