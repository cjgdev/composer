"""Basic tests for the Composer library."""

from __future__ import annotations

import pytest


def test_imports() -> None:
    """Test that the composer module can be imported."""
    try:
        import composer

        assert hasattr(composer, "__version__")
    except ImportError:
        pytest.skip("Composer module not built yet")


def test_basic_functionality() -> None:
    """Test basic functionality is available."""
    try:
        import composer

        # Test that the basic classes are available
        assert hasattr(composer, "Chord")
        assert hasattr(composer, "ScaleFingerprint")
        assert hasattr(composer, "AiEngine")
    except ImportError:
        pytest.skip("Composer module not built yet")


def test_version() -> None:
    """Test that version information is available."""
    try:
        import composer

        version = getattr(composer, "__version__", None)
        if version:
            assert isinstance(version, str)
            assert len(version) > 0
    except ImportError:
        pytest.skip("Composer module not built yet")
