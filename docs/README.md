# Composer Documentation

This directory contains unified documentation for the Composer music theory library.

## Structure

### Platform-Specific Documentation
- **[Rust Documentation](../rust/README.md)** - Core library implementation
- **[Python Package](../python/README.md)** - Python bindings and usage
- **[WebAssembly Package](../wasm/README.md)** - JavaScript/WASM bindings

### Technical Documentation
- **[Performance Analysis](../PERFORMANCE_ANALYSIS.md)** - Benchmark results and validation
- **[Monorepo Structure Guide](../monorepo-structure.md)** - Architecture design principles
- **[Specifications](../specs/)** - Original technical specifications

### Examples and Tutorials
- **[Python Examples](../python/examples/)** - Comprehensive Python usage examples
- **[WebAssembly Examples](../wasm/examples/)** - Browser and Node.js examples
- **[Cross-Platform Examples](../examples/)** - General usage patterns

## Building Documentation

### Rust API Documentation
```bash
cd rust
cargo doc --workspace --no-deps --open
```

### Python Documentation
```bash
cd python
nox -s build
# Documentation is generated as part of the build process
```

### WebAssembly Documentation
```bash
cd wasm
npm run build
# TypeScript definitions are generated automatically
```

## Contributing to Documentation

Documentation contributions are welcome! Please ensure:

1. **Accuracy**: All code examples should be tested and work with the current API
2. **Completeness**: Cover both basic usage and advanced features
3. **Cross-Platform**: Consider all three language bindings (Rust, Python, WASM)
4. **Performance**: Include performance considerations where relevant

## Architecture Overview

The Composer library is built as a monorepo with clear separation of concerns:

- **Rust Core**: All performance-critical algorithms implemented in Rust
- **Python Bindings**: High-level API for data science and composition
- **WebAssembly Bindings**: Browser and Node.js compatibility
- **Modern Tooling**: CI/CD, automated testing, and cross-platform builds

This architecture ensures maximum performance while providing familiar APIs for each target language.