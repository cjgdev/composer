"""
Composer: High-performance music theory and composition library with
AI-powered features.

This library provides sophisticated music theory analysis, chord progression generation,
and AI-powered musical intelligence capabilities.
"""

# Import the compiled Rust extension
from .composer import *

__version__ = "2.35.2"
__author__ = "Composer Contributors"

__all__ = [
    # Core functionality will be exposed by the Rust extension
    # This will be populated based on what's exported from the FFI bindings
]
