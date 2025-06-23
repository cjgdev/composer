# Composer

A high-performance music theory and composition library with AI-powered musical intelligence, implemented in Rust with comprehensive cross-platform bindings.

## Features

- **Advanced Music Theory**: Complete chord analysis, Roman numeral notation, scale theory, and voice leading.
- **Chord Progression Suggestions**: ML-driven recommendations using trie-based pattern matching.
- **Difficulty Assessment**: Statistical models for musical complexity analysis.
- **Bass Line Harmonization**: Intelligent harmonization with multiple styles.
- **Scale Degree Harmonization**: Advanced harmonic analysis and suggestion.
- **Cross-Platform**: Native Rust, Python bindings, and WebAssembly for web applications.
- **Thread-Safe**: Built-in concurrency support throughout the library.

## Quick Start

### Rust

```rust
use composer_core::{Chord, ScaleFingerprint, get_stable_scale_degrees};
use composer_ai::AiEngine;

// Create and analyze a G7 chord
let chord = Chord::new(5, 7)?;
let scale = ScaleFingerprint::major_scale();
let degrees = get_stable_scale_degrees(&chord, &scale)?;

// AI-powered suggestions
let ai_engine = AiEngine::new(Default::default());
let suggestions = ai_engine.get_chord_suggestions(&pattern, &context, &config)?;
```

### Python

Install from PyPI:
```bash
pip install composer
```

```python
import composer

# Create and analyze chords
chord = composer.PyChord(5, 7)  # G7 chord
scale = composer.PyScaleFingerprint([True, False, True, False, True, True, False, True, False, True, False, True])

# Binary serialization for ML applications
binary_data = composer.serialize_chord_to_binary(chord)
hex_string = composer.chord_to_hex(chord)

# AI-powered suggestions
engine = composer.PyAiEngine()
suggestions = engine.get_chord_suggestions(progression, context, config)
```

### WebAssembly (JavaScript/TypeScript)

Install from NPM:
```bash
npm install @composer/composer-wasm
```

**Node.js:**
```javascript
const composer = require('@composer/composer-wasm');

const chord = new composer.WasmChord(5, 7);
const scale = composer.WasmScaleFingerprint.major();
const roman = composer.getRomanNumeral(chord, scale);

console.log(`${chord.toString()} = ${roman}`);
```

**Browser:**
```javascript
import init, * as composer from '@composer/composer-wasm/web';

await init(); // Initialize WASM module

const chord = new composer.WasmChord(6, 5); // vi chord
const degrees = composer.getStableScaleDegrees(chord, scale);
```

## Architecture

The project is organized as a cross-platform monorepo:

```
composer/
├── rust/                   # Rust workspace
│   └── crates/
│       ├── composer-core/          # Core music theory algorithms
│       ├── composer-config/        # Configuration constants
│       ├── composer-serialization/ # Binary data formats
│       ├── composer-ai/            # AI-powered features
│       ├── composer-ffi/           # Python FFI bindings
│       └── composer-wasm/          # WebAssembly bindings
├── python/                 # Python package with PyO3 bindings
├── wasm/                   # WebAssembly NPM package
├── examples/               # Cross-platform usage examples
└── docs/                   # Documentation
```

## Development

### Prerequisites

- Rust 1.70+ with Cargo
- Python 3.8+ (for Python bindings)
- Node.js 14+ and npm (for WebAssembly)
- [just](https://github.com/casey/just) command runner

### Setup

Install development dependencies and set up the environment:

```bash
just setup
```

### Building

Build all packages:
```bash
just build
```

Build specific packages:
```bash
just build-rust    # Rust workspace
just build-python  # Python package
just build-wasm    # WebAssembly package
```

### Testing

Run all tests:
```bash
just test
```

Run platform-specific tests:
```bash
just test-rust     # Rust tests
just test-python   # Python tests (multi-version)
just test-wasm     # WebAssembly tests
```

### Code Quality

Format and lint all code:
```bash
just fmt     # Format all code
just lint    # Lint all code
```

### Examples

Run comprehensive examples:
```bash
just examples                    # All examples
just examples-python            # Python examples
just examples-wasm              # WebAssembly examples
```

### Documentation

Generate documentation:
```bash
just docs                # All documentation
just docs-rust           # Rust API docs
just docs-python         # Python docs
just docs-wasm           # WebAssembly/TypeScript docs
```

## Contributing

Contributions are welcome! The core implementation is complete, but we appreciate:

- Additional examples and tutorials
- Performance optimizations
- New AI algorithms
- Integration with other music software
- Documentation improvements

Please run the full development cycle before submitting:

```bash
just full  # clean + build + test + lint
```

## Development Utilities

```bash
just status          # Check development status
just dev             # Quick build + test cycle
just clean           # Clean all build artifacts
just deps            # Show dependency information
just dev-server      # Start WASM development server
just release-check   # Validate ready for release
```

## License

MIT OR Apache-2.0
