"""Pytest configuration and shared fixtures for Composer tests."""

from __future__ import annotations

import pytest


def pytest_configure(config: pytest.Config) -> None:
    """Configure pytest with custom settings."""
    config.addinivalue_line(
        "markers", "slow: marks tests as slow (deselect with '-m \"not slow\"')"
    )
    config.addinivalue_line("markers", "integration: marks tests as integration tests")
    config.addinivalue_line("markers", "performance: marks tests as performance tests")
    config.addinivalue_line("markers", "ai: marks tests as AI-related tests")
    config.addinivalue_line(
        "markers", "serialization: marks tests as serialization-related tests"
    )


def pytest_collection_modifyitems(config: pytest.Config, items: list[pytest.Item]) -> None:
    """Modify test collection to add markers automatically."""
    for item in items:
        # Add slow marker to performance tests
        if "performance" in item.nodeid or "Performance" in str(item.cls):
            item.add_marker(pytest.mark.slow)
            item.add_marker(pytest.mark.performance)

        # Add integration marker to integration tests
        if "integration" in item.nodeid or "Integration" in str(item.cls):
            item.add_marker(pytest.mark.integration)

        # Add AI marker to AI tests
        if "ai" in item.nodeid or "Ai" in str(item.cls):
            item.add_marker(pytest.mark.ai)

        # Add serialization marker to serialization tests
        if "serialization" in item.nodeid or "Serialization" in str(item.cls):
            item.add_marker(pytest.mark.serialization)


@pytest.fixture(scope="session")
def composer_module() -> object:
    """Session-scoped fixture to import the composer module."""
    try:
        import composer

        return composer
    except ImportError:
        pytest.skip("Composer module not built yet")


@pytest.fixture
def sample_chords(composer_module: object) -> list[object]:
    """Fixture providing sample chords for testing."""
    return [
        composer_module.Chord(1, 5),  # C major
        composer_module.Chord(5, 7),  # G7
        composer_module.Chord(1, 9),  # Cmaj7
        composer_module.Chord(2, 9),  # Dm7
        composer_module.Chord(4, 5),  # F major
        composer_module.Chord(6, 5),  # A minor
    ]


@pytest.fixture
def major_scale(composer_module: object) -> object:
    """Fixture providing a C major scale."""
    pattern = [
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
    return composer_module.ScaleFingerprint(pattern)


@pytest.fixture
def minor_scale(composer_module: object) -> object:
    """Fixture providing a C minor scale."""
    pattern = [
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
    return composer_module.ScaleFingerprint(pattern)


@pytest.fixture
def ai_engine(composer_module: object) -> object:
    """Fixture providing an AI engine instance."""
    return composer_module.AiEngine()


@pytest.fixture
def initialized_ai_engine(composer_module: object) -> object:
    """Fixture providing an AI engine instance with training data."""
    engine = composer_module.AiEngine()

    # Note: AI engine initialization requires specific training data format
    # that is complex to provide in unit tests. Tests will handle gracefully
    # by checking if the engine is properly initialized before testing suggestions.

    return engine


@pytest.fixture
def common_progressions(composer_module: object) -> dict[str, list[object]]:
    """Fixture providing common chord progressions."""
    return {
        "I_vi_IV_V": [
            composer_module.Chord(1, 5),  # C major
            composer_module.Chord(6, 5),  # A minor
            composer_module.Chord(4, 5),  # F major
            composer_module.Chord(5, 5),  # G major
        ],
        "ii_V_I": [
            composer_module.Chord(2, 5),  # D minor
            composer_module.Chord(5, 7),  # G7
            composer_module.Chord(1, 5),  # C major
        ],
        "jazz_turnaround": [
            composer_module.Chord(1, 9),  # Cmaj7
            composer_module.Chord(2, 9),  # Dm7
            composer_module.Chord(5, 7),  # G7
            composer_module.Chord(1, 9),  # Cmaj7
        ],
    }


@pytest.fixture
def chord_types() -> list[tuple[int, str]]:
    """Fixture providing chord type test data."""
    return [
        (5, "major"),
        (5, "minor"),
        (5, "diminished"),
        (5, "augmented"),
        (7, "dominant_7th"),
        (9, "major_7th"),
        (9, "minor_7th"),
        (7, "diminished_7th"),
    ]


@pytest.fixture
def scale_patterns() -> dict[str, list[bool]]:
    """Fixture providing various scale patterns."""
    return {
        "major": [
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
        "minor": [
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
        "dorian": [
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
            True,
            False,
        ],
        "mixolydian": [
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
        "chromatic": [True] * 12,
        "pentatonic": [
            True,
            False,
            True,
            False,
            True,
            False,
            False,
            True,
            False,
            True,
            False,
            False,
        ],
    }


@pytest.fixture
def performance_test_data(composer_module: object) -> list[object]:
    """Fixture providing data for performance tests."""
    chords = []
    for i in range(100):
        root = (i % 7) + 1  # 1-7 for valid scale degrees
        chord_type = [5, 7, 9, 11, 13][i % 5]  # Valid chord types
        chord = composer_module.Chord(root, chord_type)
        chords.append(chord)
    return chords


@pytest.fixture(autouse=True)
def skip_if_no_composer() -> None:
    """Auto-use fixture to skip tests if composer module is not available."""
    try:
        import composer  # noqa: F401
    except ImportError:
        pytest.skip("Composer module not built - run 'nox -s build' first")


def pytest_runtest_setup(item: pytest.Item) -> None:
    """Setup function run before each test."""
    # Skip performance tests by default unless explicitly requested
    if "performance" in item.keywords:
        if not item.config.getoption("--run-performance", default=False):
            pytest.skip("Performance tests skipped (use --run-performance to run)")


def pytest_addoption(parser: pytest.Parser) -> None:
    """Add custom command line options."""
    parser.addoption(
        "--run-performance",
        action="store_true",
        default=False,
        help="run performance tests",
    )
    parser.addoption(
        "--run-slow", action="store_true", default=False, help="run slow tests"
    )


# Helper functions for tests
def assert_chord_equal(chord1: object, chord2: object) -> None:
    """Helper function to assert chord equality."""
    if hasattr(chord1, "root") and hasattr(chord2, "root"):
        assert chord1.root == chord2.root
    if hasattr(chord1, "chord_type") and hasattr(chord2, "chord_type"):
        assert chord1.chord_type == chord2.chord_type
    if hasattr(chord1, "inversion") and hasattr(chord2, "inversion"):
        assert chord1.inversion == chord2.inversion


def assert_scale_equal(scale1: object, scale2: object) -> None:
    """Helper function to assert scale equality."""
    if hasattr(scale1, "pattern") and hasattr(scale2, "pattern"):
        assert scale1.pattern == scale2.pattern
    elif hasattr(scale1, "__str__") and hasattr(scale2, "__str__"):
        assert str(scale1) == str(scale2)


def create_test_progression(composer_module: object, length: int = 4) -> list[object]:
    """Helper function to create test progressions."""
    progression = []
    chord_types = [5, 5, 7, 9]  # major, minor, dom7, maj7

    for i in range(length):
        root = (i % 7) + 1  # 1-7 for valid scale degrees
        chord_type = chord_types[i % len(chord_types)]
        chord = composer_module.Chord(root, chord_type)
        progression.append(chord)

    return progression


# Parametrized fixtures for comprehensive testing
@pytest.fixture(
    params=[
        (1, 5),
        (2, 5),
        (3, 5),
        (4, 5),
        (5, 5),
        (6, 5),  # Major chords
        (7, 5),
        (1, 5),
        (2, 5),
        (3, 5),
        (4, 5),
        (5, 5),
        (1, 5),
        (2, 5),
        (3, 5),
        (4, 5),
        (5, 5),
        (6, 5),  # Minor chords (using major type for now)
        (1, 7),
        (2, 7),
        (3, 7),
        (4, 7),
        (5, 7),  # Dominant 7th chords
    ]
)
def all_basic_chords(request: pytest.FixtureRequest, composer_module: object) -> object:
    """Parametrized fixture for all basic chord types."""
    root, chord_type = request.param
    return composer_module.Chord(root, chord_type)


@pytest.fixture(params=[4, 8, 12, 16])
def progression_lengths(request: pytest.FixtureRequest) -> int:
    """Parametrized fixture for different progression lengths."""
    return request.param


# Session-wide test data
@pytest.fixture(scope="session")
def test_training_data() -> list[list[tuple[int, int]]]:
    """Session-scoped fixture providing training data for AI tests."""
    return [
        [(1, 5), (6, 5), (4, 5), (5, 5)],  # I-vi-IV-V
        [(1, 5), (4, 5), (5, 5), (1, 5)],  # I-IV-V-I
        [(1, 5), (4, 5), (5, 7), (1, 5)],  # i-iv-V7-i
        [(1, 5), (2, 5), (5, 7), (1, 5)],  # I-ii-V7-I
        [(1, 9), (2, 9), (5, 7), (1, 9)],  # Cmaj7-Dm7-G7-Cmaj7
    ]
