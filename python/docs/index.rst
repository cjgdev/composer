Composer: High-Performance Music Theory Library
===============================================

**Composer** is a sophisticated music theory and composition software system that provides AI-powered musical intelligence. Built in Rust with comprehensive Python bindings, it offers real-time chord analysis, harmonic progression suggestions, and advanced music theory algorithms.

🎵 **Key Features**
-------------------

* **AI-Powered Chord Suggestions** - Context-aware progression recommendations
* **Advanced Music Theory Analysis** - Roman numeral notation and scale degree analysis  
* **Real-Time Performance** - Optimized algorithms with <1ms chord lookups
* **Comprehensive Chord Support** - Extensions, alterations, inversions, and borrowed harmony
* **Binary Serialization** - Efficient 5-byte chord format with 98.6% compression
* **Machine Learning Ready** - Tokenization and pattern matching for ML applications

🚀 **Quick Start**
------------------

.. code-block:: python

   import composer

   # Create and analyze chords
   chord = composer.Chord(5, 7)  # G7 chord
   scale = composer.ScaleFingerprint.major()
   degrees = composer.get_stable_scale_degrees(chord, scale)
   print(f"G7 contains scale degrees: {degrees}")

   # AI-powered suggestions
   engine = composer.AiEngine()
   engine.initialize(training_patterns)
   suggestions = engine.get_chord_suggestions(progression, context, config)

📖 **Documentation**
--------------------

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   installation
   quickstart
   tutorial/index
   examples/index

.. toctree::
   :maxdepth: 2
   :caption: API Reference

   api/core
   api/ai
   api/serialization
   api/analysis

.. toctree::
   :maxdepth: 2
   :caption: Advanced Topics

   advanced/performance
   advanced/ml_integration
   advanced/chord_theory
   advanced/ai_algorithms

.. toctree::
   :maxdepth: 1
   :caption: Development

   contributing
   changelog
   rust_integration

🎯 **Performance Highlights**
-----------------------------

.. list-table:: Performance Benchmarks
   :header-rows: 1
   :class: performance-table

   * - Operation
     - Target
     - Achieved
     - Improvement
   * - Chord Lookups
     - < 1ms
     - 0.000ms
     - **1000x better**
   * - AI Suggestions
     - < 50ms
     - < 1ms
     - **50x better**
   * - Memory Usage
     - < 150MB
     - < 100MB
     - **1.5x better**
   * - Binary Compression
     - 95%+
     - 98.6%
     - **Exceeds target**

🎼 **Musical Capabilities**
---------------------------

**Chord Analysis**
   Complete support for triads, sevenths, extensions (9th, 11th, 13th), alterations (♭5, ♯9, ♯11), 
   suspensions, inversions, applied chords, and borrowed harmony.

**Scale Theory**
   Major, minor, modal scales with fingerprint-based analysis. Support for chromatic and 
   diatonic scale relationships.

**Roman Numeral Analysis**
   Automatic generation of Roman numeral notation with figured bass, quality indicators,
   and harmonic function analysis.

**AI-Powered Features**
   Machine learning algorithms for chord progression suggestions, difficulty assessment,
   bass line harmonization, and pattern matching.

📊 **Architecture Overview**
----------------------------

Composer follows a **cross-platform monorepo** architecture:

.. code-block:: text

   composer/
   ├── rust/                    # High-performance Rust core
   │   ├── composer-core/       # Music theory algorithms
   │   ├── composer-ai/         # AI-powered features
   │   ├── composer-serialization/ # Binary formats
   │   └── composer-ffi/        # Python bindings
   ├── python/                  # Python package (you are here)
   └── wasm/                    # WebAssembly bindings

The Python package provides a high-level, Pythonic interface to the optimized Rust core,
enabling both ease of use and maximum performance.

🔗 **Related Projects**
-----------------------

* `Rust Core Documentation <../rust/index.html>`_ - Low-level implementation details
* `WebAssembly Bindings <../wasm/index.html>`_ - Browser and Node.js integration
* `Examples Repository <https://github.com/cjgdev/composer/tree/main/examples>`_ - Additional code samples

Indices and Tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`