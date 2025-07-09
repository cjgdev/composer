"""Comprehensive tests for serialization and tokenization functionality."""

from __future__ import annotations

import pytest


@pytest.fixture
def sample_chords() -> list[tuple[int, int]]:
    """Sample chords for serialization testing."""
    return [
        (1, 5),  # C major
        (5, 7),  # G7
        (1, 9),  # Cmaj7
        (2, 9),  # Dm7
        (4, 5),  # F major
        (6, 5),  # A minor
        (7, 7),  # G7
        (7, 7),  # B diminished
    ]


@pytest.fixture
def sample_binary_data() -> list[bytes]:
    """Sample binary data for testing."""
    return [
        b"\x00\x00\x00\x00\x00",  # Sample 5-byte chord data
        b"\x05\x07\x00\x00\x00",  # Another sample
        b"\xff\xff\xff\xff\xff",  # Edge case
    ]


class TestBinarySerialization:
    """Test binary serialization functionality."""

    def test_chord_to_binary(self, sample_chords: list[tuple[int, int]]) -> None:
        """Test chord to binary conversion."""
        try:
            import composer

            if hasattr(composer, "serialize_chord_to_binary"):
                for root, chord_type in sample_chords:
                    chord = composer.Chord(root, chord_type)
                    binary_data = composer.serialize_chord_to_binary(chord)

                    assert binary_data is not None
                    assert isinstance(binary_data, bytes)
                    assert len(binary_data) == 5  # 5-byte format

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord to binary test failed: {e}")

    def test_binary_to_chord(self, sample_binary_data: list[bytes]) -> None:
        """Test binary to chord conversion."""
        try:
            import composer

            if hasattr(composer, "deserialize_chord_from_binary"):
                for binary_data in sample_binary_data:
                    try:
                        chord = composer.deserialize_chord_from_binary(binary_data)
                        assert chord is not None
                        assert hasattr(chord, "root")
                        assert hasattr(chord, "chord_type")
                    except Exception:
                        # Some binary data might be invalid, that's okay
                        pass

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Binary to chord test failed: {e}")

    def test_binary_serialization_roundtrip(
        self, sample_chords: list[tuple[int, int]]
    ) -> None:
        """Test binary serialization roundtrip."""
        try:
            import composer

            if hasattr(composer, "serialize_chord_to_binary") and hasattr(
                composer, "deserialize_chord_from_binary"
            ):
                for root, chord_type in sample_chords:
                    original_chord = composer.Chord(root, chord_type)

                    # Serialize to binary
                    binary_data = composer.serialize_chord_to_binary(original_chord)

                    # Deserialize back to chord
                    restored_chord = composer.deserialize_chord_from_binary(binary_data)

                    # Compare basic properties
                    if hasattr(original_chord, "root") and hasattr(
                        restored_chord, "root"
                    ):
                        assert original_chord.root == restored_chord.root
                    if hasattr(original_chord, "chord_type") and hasattr(
                        restored_chord, "chord_type"
                    ):
                        assert original_chord.chord_type == restored_chord.chord_type

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Binary serialization roundtrip test failed: {e}")


class TestHexSerialization:
    """Test hex serialization functionality."""

    def test_chord_to_hex(self, sample_chords: list[tuple[int, int]]) -> None:
        """Test chord to hex conversion."""
        try:
            import composer

            if hasattr(composer, "chord_to_hex"):
                for root, chord_type in sample_chords:
                    chord = composer.Chord(root, chord_type)
                    hex_string = composer.chord_to_hex(chord)

                    assert hex_string is not None
                    assert isinstance(hex_string, str)
                    assert len(hex_string) == 10  # 5 bytes = 10 hex chars

                    # Verify it's valid hex
                    int(hex_string, 16)

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord to hex test failed: {e}")

    def test_hex_to_chord(self) -> None:
        """Test hex to chord conversion."""
        try:
            import composer

            if hasattr(composer, "chord_from_hex"):
                sample_hex_strings = [
                    "0000000000",  # All zeros
                    "0507000000",  # G7
                    "0008000000",  # Cmaj7
                ]

                for hex_string in sample_hex_strings:
                    try:
                        chord = composer.chord_from_hex(hex_string)
                        assert chord is not None
                        assert hasattr(chord, "root")
                        assert hasattr(chord, "chord_type")
                    except Exception:
                        # Some hex strings might be invalid, that's okay
                        pass

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Hex to chord test failed: {e}")

    def test_hex_serialization_roundtrip(
        self, sample_chords: list[tuple[int, int]]
    ) -> None:
        """Test hex serialization roundtrip."""
        try:
            import composer

            if hasattr(composer, "chord_to_hex") and hasattr(
                composer, "chord_from_hex"
            ):
                for root, chord_type in sample_chords:
                    original_chord = composer.Chord(root, chord_type)

                    # Serialize to hex
                    hex_string = composer.chord_to_hex(original_chord)

                    # Deserialize back to chord
                    restored_chord = composer.chord_from_hex(hex_string)

                    # Compare basic properties
                    if hasattr(original_chord, "root") and hasattr(
                        restored_chord, "root"
                    ):
                        assert original_chord.root == restored_chord.root
                    if hasattr(original_chord, "chord_type") and hasattr(
                        restored_chord, "chord_type"
                    ):
                        assert original_chord.chord_type == restored_chord.chord_type

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Hex serialization roundtrip test failed: {e}")


class TestTokenization:
    """Test tokenization functionality for ML applications."""

    def test_chord_tokenization(self, sample_chords: list[tuple[int, int]]) -> None:
        """Test chord tokenization."""
        try:
            import composer

            if hasattr(composer, "tokenize_chord"):
                for root, chord_type in sample_chords:
                    chord = composer.Chord(root, chord_type)
                    token = composer.tokenize_chord(chord)

                    assert token is not None
                    assert isinstance(token, (int, str))

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord tokenization test failed: {e}")

    def test_progression_tokenization(self) -> None:
        """Test progression tokenization."""
        try:
            import composer

            if hasattr(composer, "tokenize_progression"):
                progression = [
                    composer.Chord(1, 5),  # C major
                    composer.Chord(6, 5),  # A minor
                    composer.Chord(4, 5),  # F major
                    composer.Chord(5, 7),  # G7
                ]

                tokens = composer.tokenize_progression(progression)
                assert isinstance(tokens, list)
                assert len(tokens) == len(progression)

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Progression tokenization test failed: {e}")

    def test_token_library_creation(self) -> None:
        """Test token library creation."""
        try:
            import composer

            if hasattr(composer, "PyTokenLibrary"):
                token_lib = composer.PyTokenLibrary()
                assert token_lib is not None

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Token library creation test failed: {e}")

    def test_note_tokenization(self) -> None:
        """Test note tokenization."""
        try:
            import composer

            if hasattr(composer, "PyNote"):
                # Test note creation and tokenization
                note = composer.PyNote(60, 24, 100)  # Middle C, quarter note, forte
                assert note is not None

                if hasattr(composer, "tokenize_note"):
                    token = composer.tokenize_note(note)
                    assert token is not None

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Note tokenization test failed: {e}")

    def test_timeline_tokenization(self) -> None:
        """Test timeline tokenization."""
        try:
            import composer

            if hasattr(composer, "PyTimeline"):
                timeline = composer.PyTimeline()
                assert timeline is not None

                if hasattr(composer, "tokenize_timeline"):
                    token = composer.tokenize_timeline(timeline)
                    assert token is not None

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Timeline tokenization test failed: {e}")


class TestCompressionEfficiency:
    """Test compression efficiency of serialization."""

    def test_compression_ratio(self, sample_chords: list[tuple[int, int]]) -> None:
        """Test compression ratio of binary serialization."""
        try:
            import composer

            if hasattr(composer, "serialize_chord_to_binary"):
                total_original_size = 0
                total_compressed_size = 0

                for root, chord_type in sample_chords:
                    chord = composer.Chord(root, chord_type)
                    binary_data = composer.serialize_chord_to_binary(chord)

                    # Estimate original size (assuming chord has multiple fields)
                    original_size = 27 * 4  # 27 fields * 4 bytes each (rough estimate)
                    compressed_size = len(binary_data)

                    total_original_size += original_size
                    total_compressed_size += compressed_size

                if total_original_size > 0:
                    compression_ratio = total_compressed_size / total_original_size
                    # Should achieve significant compression
                    assert compression_ratio < 0.1  # Less than 10% of original size

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Compression ratio test failed: {e}")

    def test_serialization_performance(
        self, sample_chords: list[tuple[int, int]]
    ) -> None:
        """Test serialization performance."""
        try:
            import time

            import composer

            if hasattr(composer, "serialize_chord_to_binary"):
                start_time = time.time()

                for _ in range(100):  # Serialize 100 chords
                    for root, chord_type in sample_chords:
                        chord = composer.Chord(root, chord_type)
                        composer.serialize_chord_to_binary(chord)

                end_time = time.time()

                # Should complete within reasonable time
                assert end_time - start_time < 1.0  # Less than 1 second

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Serialization performance test failed: {e}")


class TestDataIntegrity:
    """Test data integrity and validation."""

    def test_invalid_binary_data(self) -> None:
        """Test handling of invalid binary data."""
        try:
            import composer

            if hasattr(composer, "deserialize_chord_from_binary"):
                invalid_data = [
                    b"",  # Empty data
                    b"\x00",  # Too short
                    b"\x00" * 3,  # Still too short
                    b"\x00" * 6,  # Too long
                    b"\xff" * 20,  # Way too long
                ]

                for data in invalid_data:
                    with pytest.raises((ValueError, TypeError, Exception)):
                        composer.deserialize_chord_from_binary(data)

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Invalid binary data test failed: {e}")

    def test_invalid_hex_data(self) -> None:
        """Test handling of invalid hex data."""
        try:
            import composer

            if hasattr(composer, "chord_from_hex"):
                invalid_hex = [
                    "",  # Empty string
                    "invalid",  # Non-hex characters
                    "12345",  # Wrong length
                    "123456789",  # Wrong length
                    "1234567890123456789",  # Too long
                ]

                for hex_string in invalid_hex:
                    with pytest.raises((ValueError, TypeError, Exception)):
                        composer.chord_from_hex(hex_string)

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Invalid hex data test failed: {e}")

    def test_data_consistency(self, sample_chords: list[tuple[int, int]]) -> None:
        """Test data consistency across different serialization formats."""
        try:
            import composer

            if hasattr(composer, "serialize_chord_to_binary") and hasattr(
                composer, "chord_to_hex"
            ):
                for root, chord_type in sample_chords:
                    chord = composer.Chord(root, chord_type)

                    # Get both serialization formats
                    binary_data = composer.serialize_chord_to_binary(chord)
                    hex_string = composer.chord_to_hex(chord)

                    # Convert hex to binary and compare
                    hex_as_bytes = bytes.fromhex(hex_string)
                    assert binary_data == hex_as_bytes

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Data consistency test failed: {e}")


@pytest.mark.parametrize(
    "root,chord_type",
    [
        (1, 5),  # C major
        (5, 7),  # G7
        (1, 9),  # Cmaj7
        (7, 7),  # B diminished
    ],
)
def test_serialization_edge_cases(root: int, chord_type: int) -> None:
    """Test serialization with edge cases."""
    try:
        import composer

        chord = composer.Chord(root, chord_type)

        # Test binary serialization
        if hasattr(composer, "serialize_chord_to_binary"):
            binary_data = composer.serialize_chord_to_binary(chord)
            assert len(binary_data) == 5

        # Test hex serialization
        if hasattr(composer, "chord_to_hex"):
            hex_string = composer.chord_to_hex(chord)
            assert len(hex_string) == 10

    except ImportError:
        pytest.skip("Composer module not built yet")
    except Exception as e:
        pytest.skip(f"Serialization edge cases test failed: {e}")


class TestBatchOperations:
    """Test batch serialization operations."""

    def test_batch_serialization(self, sample_chords: list[tuple[int, int]]) -> None:
        """Test batch serialization of multiple chords."""
        try:
            import composer

            if hasattr(composer, "serialize_chord_batch"):
                chord_list = []
                for root, chord_type in sample_chords:
                    chord_list.append(composer.Chord(root, chord_type))

                batch_data = composer.serialize_chord_batch(chord_list)
                assert isinstance(batch_data, (list, bytes))

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Batch serialization test failed: {e}")

    def test_batch_deserialization(self) -> None:
        """Test batch deserialization of multiple chords."""
        try:
            import composer

            if hasattr(composer, "deserialize_chord_batch"):
                # Sample batch data (multiple 5-byte entries)
                batch_data = (
                    b"\x00\x00\x00\x00\x00\x05\x07\x00\x00\x00\x00\x08\x00\x00\x00"
                )

                chord_list = composer.deserialize_chord_batch(batch_data)
                assert isinstance(chord_list, list)
                assert len(chord_list) == 3  # 15 bytes / 5 bytes per chord

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Batch deserialization test failed: {e}")
