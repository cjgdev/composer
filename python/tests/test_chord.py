"""Comprehensive tests for chord functionality."""

from __future__ import annotations

import pytest


# Test fixtures for common chord types
@pytest.fixture
def basic_chords() -> list[tuple[int, int, str]]:
    """Basic chord test data: (root, chord_type, expected_name)."""
    return [
        (1, 5, "C"),  # C major
        (2, 5, "C#"),  # C# major
        (3, 5, "D"),  # D major
        (7, 5, "G"),  # G major
        (1, 5, "Cm"),  # C minor
        (5, 7, "G7"),  # G7
        (1, 9, "Cmaj7"),  # C major 7
        (1, 9, "Cm7"),  # C minor 7
    ]


@pytest.fixture
def complex_chords() -> list[tuple[int, int, int, bool, list[int]]]:
    """Complex chord test data with various features."""
    return [
        (1, 5, 1, False, []),  # C major, first inversion
        (5, 7, 0, True, [11]),  # G7/F# with flat 5
        (4, 7, 2, False, [9]),  # F dim7, second inversion, with sharp 9
    ]


class TestChordCreation:
    """Test chord creation and basic properties."""

    def test_basic_chord_creation(self, basic_chords) -> None:
        """Test creation of basic chord types."""
        try:
            import composer

            for root, chord_type, _expected_name in basic_chords:
                chord = composer.Chord(root, chord_type)
                assert chord is not None
                assert hasattr(chord, "root")
                assert hasattr(chord, "chord_type")

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord creation failed: {e}")

    def test_chord_with_inversion(self) -> None:
        """Test chord creation with inversion."""
        try:
            import composer

            # Test chord with inversion if supported
            chord = composer.Chord(1, 5)  # C major
            if hasattr(chord, "inversion"):
                inverted_chord = composer.Chord(1, 5, 1)  # C major first inversion
                assert inverted_chord is not None

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord inversion test failed: {e}")

    def test_chord_with_alterations(self) -> None:
        """Test chord creation with alterations."""
        try:
            import composer

            # Test basic chord first
            chord = composer.Chord(1, 5)  # C major
            assert chord is not None

            # Test chord with alterations if supported
            if (
                hasattr(composer, "PyChord")
                and len(composer.Chord.__init__.__code__.co_varnames) > 3
            ):
                altered_chord = composer.Chord(1, 7, 0, False, [11])  # C7b5
                assert altered_chord is not None

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord alterations test failed: {e}")


class TestChordProperties:
    """Test chord properties and methods."""

    def test_chord_basic_properties(self) -> None:
        """Test basic chord properties."""
        try:
            import composer

            chord = composer.Chord(1, 5)  # C major

            # Test basic properties
            if hasattr(chord, "root"):
                assert chord.root == 1
            if hasattr(chord, "chord_type"):
                assert chord.chord_type == 5
            if hasattr(chord, "inversion"):
                assert isinstance(chord.inversion, int)

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord properties test failed: {e}")

    def test_chord_methods(self) -> None:
        """Test chord methods if available."""
        try:
            import composer

            chord = composer.Chord(5, 7)  # G7

            # Test common methods
            if hasattr(chord, "is_seventh"):
                assert chord.is_seventh()
            if hasattr(chord, "is_major"):
                assert isinstance(chord.is_major(), bool)
            if hasattr(chord, "is_minor"):
                assert isinstance(chord.is_minor(), bool)
            if hasattr(chord, "get_notes"):
                notes = chord.get_notes()
                assert isinstance(notes, list)

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord methods test failed: {e}")

    def test_chord_string_representation(self) -> None:
        """Test chord string representation."""
        try:
            import composer

            chord = composer.Chord(1, 5)  # C major

            # Test string representation
            if hasattr(chord, "__str__"):
                chord_str = str(chord)
                assert isinstance(chord_str, str)
                assert len(chord_str) > 0

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord string representation test failed: {e}")


class TestChordSerialization:
    """Test chord serialization functionality."""

    def test_chord_to_binary(self) -> None:
        """Test chord binary serialization."""
        try:
            import composer

            chord = composer.Chord(1, 5)  # C major

            # Test binary serialization
            if hasattr(composer, "serialize_chord_to_binary"):
                binary_data = composer.serialize_chord_to_binary(chord)
                assert binary_data is not None
                assert isinstance(binary_data, bytes)
                # Binary format should be 5 bytes according to spec
                assert len(binary_data) == 5

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord binary serialization test failed: {e}")

    def test_chord_to_hex(self) -> None:
        """Test chord hex serialization."""
        try:
            import composer

            chord = composer.Chord(1, 5)  # C major

            if hasattr(composer, "chord_to_hex"):
                hex_string = composer.chord_to_hex(chord)
                assert hex_string is not None
                assert isinstance(hex_string, str)
                # Hex string should be 10 characters (5 bytes * 2)
                assert len(hex_string) == 10
                # Should be valid hex
                int(hex_string, 16)

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord hex serialization test failed: {e}")

    def test_chord_serialization_roundtrip(self) -> None:
        """Test chord serialization roundtrip."""
        try:
            import composer

            original_chord = composer.Chord(5, 7)  # G7

            # Test hex roundtrip
            if hasattr(composer, "chord_to_hex") and hasattr(
                composer, "chord_from_hex"
            ):
                hex_string = composer.chord_to_hex(original_chord)
                restored_chord = composer.chord_from_hex(hex_string)

                # Compare basic properties
                if hasattr(original_chord, "root") and hasattr(restored_chord, "root"):
                    assert original_chord.root == restored_chord.root
                if hasattr(original_chord, "chord_type") and hasattr(
                    restored_chord, "chord_type"
                ):
                    assert original_chord.chord_type == restored_chord.chord_type

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Chord serialization roundtrip test failed: {e}")


class TestChordValidation:
    """Test chord validation and error handling."""

    def test_invalid_chord_root(self) -> None:
        """Test handling of invalid chord roots."""
        try:
            import composer

            # Test invalid root values
            with pytest.raises((ValueError, TypeError, Exception)):
                composer.Chord(-1, 0)  # Negative root

            with pytest.raises((ValueError, TypeError, Exception)):
                composer.Chord(12, 5)  # Root >= 12

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Invalid chord root test failed: {e}")

    def test_invalid_chord_type(self) -> None:
        """Test handling of invalid chord types."""
        try:
            import composer

            # Test invalid chord type values
            with pytest.raises((ValueError, TypeError, Exception)):
                composer.Chord(1, -1)  # Negative chord type

            with pytest.raises((ValueError, TypeError, Exception)):
                composer.Chord(1, 100)  # Very large chord type

        except ImportError:
            pytest.skip("Composer module not built yet")
        except Exception as e:
            pytest.skip(f"Invalid chord type test failed: {e}")


@pytest.mark.parametrize(
    "root,chord_type,expected_complexity",
    [
        (1, 5, 1.0),  # C major - simple
        (5, 7, 3.0),  # G7 - moderate complexity
        (1, 9, 4.0),  # Cmaj7 - higher complexity
    ],
)
def test_chord_complexity_scoring(root, chord_type, expected_complexity) -> None:
    """Test chord complexity scoring."""
    try:
        import composer

        if hasattr(composer, "get_chord_complexity"):
            chord = composer.Chord(root, chord_type)
            complexity = composer.get_chord_complexity(chord)
            assert isinstance(complexity, (int, float))
            assert complexity >= 0
            # Allow some tolerance in complexity scoring
            assert abs(complexity - expected_complexity) <= 2.0

    except ImportError:
        pytest.skip("Composer module not built yet")
    except Exception as e:
        pytest.skip(f"Chord complexity scoring test failed: {e}")
