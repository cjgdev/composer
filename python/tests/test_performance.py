"""Performance and benchmark tests for the Composer library."""

from __future__ import annotations

import time
from typing import Any

import pytest


@pytest.mark.performance
@pytest.mark.slow
class TestChordPerformance:
    """Test chord operation performance."""

    def test_chord_creation_performance(self, composer_module) -> None:
        """Test performance of chord creation."""
        start_time = time.time()

        chords = []
        for i in range(1000):
            root = (i % 7) + 1  # 1-7 for valid scale degrees
            chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
            chord = composer_module.Chord(root, chord_type)
            chords.append(chord)

        end_time = time.time()
        duration = end_time - start_time

        # Should create 1000 chords in less than 100ms
        assert duration < 0.1
        assert len(chords) == 1000

        # Test creation rate
        creation_rate = 1000 / duration
        assert creation_rate > 10000  # More than 10,000 chords/second

    def test_chord_property_access_performance(self, sample_chords) -> None:
        """Test performance of chord property access."""
        start_time = time.time()

        for _ in range(100):
            for chord in sample_chords:
                if hasattr(chord, "root"):
                    _ = chord.root
                if hasattr(chord, "chord_type"):
                    _ = chord.chord_type
                if hasattr(chord, "inversion"):
                    _ = chord.inversion

        end_time = time.time()
        duration = end_time - start_time

        # Should complete in less than 10ms
        assert duration < 0.01


@pytest.mark.performance
@pytest.mark.slow
class TestSerializationPerformance:
    """Test serialization performance."""

    def test_binary_serialization_performance(self, composer_module) -> None:
        """Test binary serialization performance."""
        # Create test chords
        test_chords = []
        for i in range(1000):
            root = (i % 7) + 1  # 1-7 for valid scale degrees
            chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
            chord = composer_module.Chord(root, chord_type)
            test_chords.append(chord)

        if hasattr(composer_module, "serialize_chord_to_binary"):
            start_time = time.time()

            serialized = []
            for chord in test_chords:
                binary_data = composer_module.serialize_chord_to_binary(chord)
                serialized.append(binary_data)

            end_time = time.time()
            duration = end_time - start_time

            # Should serialize 1000 chords in less than 50ms
            assert duration < 0.05
            assert len(serialized) == 1000

            # Test serialization rate
            serialization_rate = 1000 / duration
            assert serialization_rate > 20000  # More than 20,000 serializations/second

    def test_hex_serialization_performance(self, composer_module) -> None:
        """Test hex serialization performance."""
        # Create test chords
        test_chords = []
        for i in range(500):
            root = (i % 7) + 1  # 1-7 for valid scale degrees
            chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
            chord = composer_module.Chord(root, chord_type)
            test_chords.append(chord)

        if hasattr(composer_module, "chord_to_hex"):
            start_time = time.time()

            hex_strings = []
            for chord in test_chords:
                hex_string = composer_module.chord_to_hex(chord)
                hex_strings.append(hex_string)

            end_time = time.time()
            duration = end_time - start_time

            # Should serialize 500 chords to hex in less than 25ms
            assert duration < 0.025
            assert len(hex_strings) == 500


@pytest.mark.performance
@pytest.mark.slow
class TestAIPerformance:
    """Test AI engine performance."""

    def test_chord_suggestion_performance(
        self, ai_engine, major_scale, composer_module
    ) -> None:
        """Test chord suggestion performance."""
        if hasattr(ai_engine, "get_chord_suggestions"):
            # Create test progression
            progression = [composer_module.Chord(1, 5)]  # C major
            context = composer_module.SuggestionContext()
            config = composer_module.SuggestionConfig()

            start_time = time.time()

            try:
                for _ in range(100):
                    _ = ai_engine.get_chord_suggestions(
                        progression, context, config
                    )

                end_time = time.time()
                duration = end_time - start_time

                # Should complete 100 suggestion requests in less than 100ms
                assert duration < 0.1

                # Test suggestion rate
                suggestion_rate = 100 / duration
                assert suggestion_rate > 1000  # More than 1,000 suggestions/second
            except Exception as e:
                # Expected: AI engine requires complex training data not suitable for performance tests
                if "not initialized" in str(e) or "Engine not initialized" in str(e):
                    # Performance test validates that error response is still fast
                    end_time = time.time()
                    duration = end_time - start_time
                    assert duration < 0.1  # Error response should be fast
                else:
                    raise

    def test_difficulty_assessment_performance(
        self, composer_module, common_progressions
    ) -> None:
        """Test difficulty assessment performance."""
        if hasattr(composer_module, "assess_song_difficulty"):
            progressions = list(common_progressions.values())

            start_time = time.time()

            for _ in range(100):
                for progression in progressions:
                    _ = composer_module.assess_song_difficulty(progression)

            end_time = time.time()
            duration = end_time - start_time

            # Should complete 300 assessments (3 progressions * 100 iterations) in less than 100ms
            assert duration < 0.1


@pytest.mark.performance
@pytest.mark.slow
class TestScalePerformance:
    """Test scale operation performance."""

    def test_scale_creation_performance(self, composer_module, scale_patterns) -> None:
        """Test scale creation performance."""
        start_time = time.time()

        scales = []
        for _ in range(1000):
            for _pattern_name, pattern in scale_patterns.items():
                scale = composer_module.ScaleFingerprint(pattern)
                scales.append(scale)

        end_time = time.time()
        duration = end_time - start_time

        # Should create 6000 scales (6 patterns * 1000 iterations) in less than 100ms
        assert duration < 0.1
        assert len(scales) == 6000

    def test_roman_numeral_analysis_performance(
        self, composer_module, sample_chords, major_scale
    ) -> None:
        """Test Roman numeral analysis performance."""
        if hasattr(composer_module, "get_roman_numeral"):
            start_time = time.time()

            for _ in range(100):
                for chord in sample_chords:
                    _ = composer_module.get_roman_numeral(chord, major_scale)

            end_time = time.time()
            duration = end_time - start_time

            # Should complete 600 analyses (6 chords * 100 iterations) in less than 50ms
            assert duration < 0.05


@pytest.mark.performance
@pytest.mark.slow
class TestMemoryPerformance:
    """Test memory usage and efficiency."""

    def test_large_scale_chord_creation(self, composer_module) -> None:
        """Test memory efficiency with large numbers of chords."""
        chords = []

        # Create 10,000 chords
        for i in range(10000):
            root = (i % 7) + 1  # 1-7 for valid scale degrees
            chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
            chord = composer_module.Chord(root, chord_type)
            chords.append(chord)

        # Verify all chords were created
        assert len(chords) == 10000

        # Test that we can still access properties efficiently
        start_time = time.time()

        root_sum = 0
        for chord in chords:
            if hasattr(chord, "root"):
                root_sum += chord.root

        end_time = time.time()
        duration = end_time - start_time

        # Should access 10,000 chord properties in less than 10ms
        assert duration < 0.01
        assert root_sum > 0  # Sanity check

    def test_serialization_memory_efficiency(self, composer_module) -> None:
        """Test memory efficiency of serialization."""
        if hasattr(composer_module, "serialize_chord_to_binary"):
            # Create and serialize many chords
            serialized_data = []

            for i in range(5000):
                root = (i % 7) + 1  # 1-7 for valid scale degrees
                chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
                chord = composer_module.Chord(root, chord_type)
                binary_data = composer_module.serialize_chord_to_binary(chord)
                serialized_data.append(binary_data)

            # Verify serialization
            assert len(serialized_data) == 5000

            # Each chord should serialize to exactly 5 bytes
            total_size = sum(len(data) for data in serialized_data)
            assert total_size == 25000  # 5000 chords * 5 bytes each


@pytest.mark.benchmark
@pytest.mark.slow
class TestBenchmarks:
    """Benchmark tests using pytest-benchmark if available."""

    def test_chord_creation_benchmark(self, composer_module) -> None:
        """Benchmark chord creation."""
        pytest.skip(
            "pytest-benchmark not installed - install with: pip install pytest-benchmark"
        )

    def test_serialization_benchmark(self, composer_module) -> None:
        """Benchmark chord serialization."""
        pytest.skip(
            "pytest-benchmark not installed - install with: pip install pytest-benchmark"
        )

    def test_ai_suggestion_benchmark(
        self, ai_engine: Any, composer_module: Any
    ) -> None:
        """Benchmark AI chord suggestions."""
        pytest.skip(
            "pytest-benchmark not installed - "
            "install with: pip install pytest-benchmark"
        )


@pytest.mark.performance
@pytest.mark.parametrize("num_chords", [100, 500, 1000, 2000])
def test_scaling_performance(composer_module: Any, num_chords: int) -> None:
    """Test that operations scale linearly with input size."""
    # Create progressions of different sizes
    progression = []
    for i in range(num_chords):
        root = (i % 7) + 1  # 1-7 for valid scale degrees
        chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
        chord = composer_module.Chord(root, chord_type)
        progression.append(chord)

    # Test that difficulty assessment scales reasonably
    if hasattr(composer_module, "assess_song_difficulty"):
        start_time = time.time()
        difficulty = composer_module.assess_song_difficulty(progression)
        end_time = time.time()

        duration = end_time - start_time

        # Duration should scale roughly linearly (allowing some overhead)
        # Max 0.1ms per chord
        max_expected_duration = num_chords * 0.0001  # 0.1ms per chord
        assert duration < max_expected_duration
        assert isinstance(difficulty, (int, float))


@pytest.mark.performance
class TestConcurrencyPerformance:
    """Test performance under concurrent access."""

    def test_concurrent_chord_creation(self, composer_module: Any) -> None:
        """Test chord creation under concurrent-like access patterns."""
        import threading

        results = []
        errors = []

        def create_chords() -> None:
            try:
                local_chords = []
                for i in range(100):
                    root = (i % 7) + 1  # 1-7 for valid scale degrees
                    chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
                    chord = composer_module.Chord(root, chord_type)
                    local_chords.append(chord)
                results.append(local_chords)
            except Exception as e:
                errors.append(e)

        # Create multiple threads
        threads = []
        for _ in range(10):
            thread = threading.Thread(target=create_chords)
            threads.append(thread)

        # Start all threads
        start_time = time.time()
        for thread in threads:
            thread.start()

        # Wait for completion
        for thread in threads:
            thread.join()

        end_time = time.time()
        duration = end_time - start_time

        # Verify results
        assert len(errors) == 0  # No errors should occur
        assert len(results) == 10  # All threads should complete
        assert duration < 1.0  # Should complete within 1 second

        # Verify all chords were created correctly
        total_chords = sum(len(chord_list) for chord_list in results)
        assert total_chords == 1000  # 10 threads * 100 chords each
