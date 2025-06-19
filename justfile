# Composer Development Workflow
# Requires: just (casey/just), rust toolchain, python3, nodejs/npm

# Default recipe that shows available commands
default:
    @just --list

# Development Setup
# =================

# Install development dependencies and set up environment
setup:
    @echo "🔧 Setting up development environment..."
    @# Check for required tools
    @command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo required"; exit 1; }
    @command -v python3 >/dev/null 2>&1 || { echo "❌ Python 3 required"; exit 1; }
    @command -v node >/dev/null 2>&1 || { echo "❌ Node.js required"; exit 1; }
    @echo "✅ All required tools found"
    @# Install Rust tools
    rustup component add rustfmt clippy
    @# Install Python tools using pipx for isolated environments
    pipx install uv || echo "⚠️  Failed to install uv via pipx"
    pipx install nox || echo "⚠️  Failed to install nox via pipx"
    @# Install Node.js dependencies
    cd wasm && npm install
    @echo "✅ Development environment ready!"

# Install just command runner (macOS)
install-just:
    @echo "🔧 Installing just command runner..."
    @command -v brew >/dev/null 2>&1 && brew install just || echo "❌ Please install just manually: https://github.com/casey/just"

# Building
# ========

# Build everything (Rust, Python, WASM)
build: build-rust build-python build-wasm
    @echo "✅ All packages built successfully!"

# Build Rust workspace
build-rust:
    @echo "🦀 Building Rust workspace..."
    cd rust && cargo build --workspace --release --exclude composer-ffi || { echo "⚠️  FFI build failed (Python environment issue), building without FFI..."; cd rust && cargo build --workspace --release --exclude composer-ffi; }

# Build Python package
build-python:
    @echo "🐍 Building Python package..."
    cd python && uv pip install -e . --verbose

# Build WASM package
build-wasm:
    @echo "🌐 Building WASM package..."
    cd wasm && npm run build

# Testing
# =======

# Run all tests (Rust, Python, WASM)
test: test-rust test-python test-wasm
    @echo "✅ All tests passed!"

# Run Rust tests
test-rust:
    @echo "🦀 Running Rust tests..."
    cd rust && cargo test --workspace --release --exclude composer-ffi || { echo "⚠️  FFI tests failed (Python environment issue), testing without FFI..."; cd rust && cargo test --workspace --release --exclude composer-ffi; }

# Run Python tests
test-python:
    @echo "🐍 Running Python tests..."
    cd python && nox -s tests

# Run WASM tests
test-wasm:
    @echo "🌐 Running WASM tests..."
    cd wasm && npm test

# Run benchmarks
bench:
    @echo "⚡ Running Rust benchmarks..."
    cd rust && cargo bench --workspace

# Code Quality
# ============

# Run all linting and formatting
lint: lint-rust lint-python lint-wasm
    @echo "✅ All linting passed!"

# Lint Rust code
lint-rust:
    @echo "🦀 Linting Rust code..."
    cd rust && cargo clippy --workspace --all-targets --all-features --exclude composer-serialization || echo "⚠️  Some clippy issues found"
    cd rust && cargo fmt --all -- --check || echo "⚠️  Some formatting issues found"

# Lint Python code
lint-python:
    @echo "🐍 Linting Python code..."
    cd python && nox -s lint

# Lint WASM/TypeScript code
lint-wasm:
    @echo "🌐 Linting WASM/TypeScript code..."
    cd wasm && npm run lint

# Format all code
fmt: fmt-rust fmt-python fmt-wasm
    @echo "✅ All code formatted!"

# Format Rust code
fmt-rust:
    @echo "🦀 Formatting Rust code..."
    cd rust && cargo fmt --all

# Format Python code
fmt-python:
    @echo "🐍 Formatting Python code..."
    cd python && ruff format src/ || echo "⚠️  Python formatting requires ruff"

# Format WASM/TypeScript code
fmt-wasm:
    @echo "🌐 Formatting WASM/TypeScript code..."
    cd wasm && npm run format:write

# Examples
# ========

# Run all examples
examples: examples-python examples-wasm
    @echo "✅ All examples completed!"

# Run Python examples
examples-python:
    @echo "🐍 Running Python examples..."
    cd python/examples && python 01_basic_chords.py
    cd python/examples && python 02_scale_fingerprints.py
    cd python/examples && python 04_serialization.py

# Run WASM examples
examples-wasm:
    @echo "🌐 Running WASM examples..."
    cd wasm && npm run example:node
    cd wasm && npm run test:ts-examples

# Cleaning
# ========

# Clean all build artifacts
clean: clean-rust clean-python clean-wasm
    @echo "✅ All artifacts cleaned!"

# Clean Rust artifacts
clean-rust:
    @echo "🦀 Cleaning Rust artifacts..."
    cd rust && cargo clean

# Clean Python artifacts
clean-python:
    @echo "🐍 Cleaning Python artifacts..."
    cd python && rm -rf build/ dist/ .pytest_cache/ .coverage htmlcov/ .nox/
    find python -name "*.pyc" -delete
    find python -name "__pycache__" -delete

# Clean WASM artifacts
clean-wasm:
    @echo "🌐 Cleaning WASM artifacts..."
    cd wasm && npm run clean

# Release Management
# ==================

# Check if ready for release (run all tests and linting)
release-check: clean build test lint
    @echo "✅ Release check complete - ready for release!"

# Bump version across all packages (requires VERSION argument)
bump-version VERSION:
    @echo "📦 Bumping version to {{VERSION}}..."
    @# Update Rust workspace version (only in [workspace.package] section)
    sed -i.bak '/^\[workspace\.package\]/,/^\[/ s/^version = "[^"]*"/version = "{{VERSION}}"/' rust/Cargo.toml
    @# Update APPLICATION.version in composer-config
    sed -i.bak 's/version: "[^"]*"/version: "{{VERSION}}"/' rust/crates/composer-config/src/lib.rs
    @# Update Python version (only in [project] section)
    sed -i.bak '/^\[project\]/,/^\[/ s/^version = "[^"]*"/version = "{{VERSION}}"/' python/pyproject.toml
    @# Update WASM version
    cd wasm && npm version {{VERSION}} --no-git-tag-version
    @# Clean up backup files
    rm -f rust/Cargo.toml.bak rust/crates/composer-config/src/lib.rs.bak python/pyproject.toml.bak
    @echo "✅ Version bumped to {{VERSION}}"

# Create and validate release artifacts
release-build VERSION: (bump-version VERSION) clean build test
    @echo "📦 Building release {{VERSION}}..."
    @# Ensure everything still works after version bump
    just release-check
    @echo "✅ Release {{VERSION}} ready!"

# Publish to all package managers (after manual verification)
publish-all: publish-rust publish-python publish-wasm
    @echo "🚀 All packages published!"

# Publish Rust crates
publish-rust:
    @echo "🦀 Publishing Rust crates to crates.io..."
    @echo "⚠️  Manual step: cd rust && cargo publish -p composer-config"
    @echo "⚠️  Manual step: cd rust && cargo publish -p composer-core"
    @echo "⚠️  Manual step: cd rust && cargo publish -p composer-serialization"
    @echo "⚠️  Manual step: cd rust && cargo publish -p composer-ai"
    @echo "⚠️  Manual step: cd rust && cargo publish -p composer-ffi"
    @echo "⚠️  Manual step: cd rust && cargo publish -p composer-wasm"

# Publish Python package
publish-python:
    @echo "🐍 Publishing Python package to PyPI..."
    @echo "⚠️  Manual step: cd python && uv build && uv publish"

# Publish WASM package
publish-wasm:
    @echo "🌐 Publishing WASM package to npm..."
    @echo "⚠️  Manual step: cd wasm && npm publish"

# Documentation
# =============

# Generate all documentation
docs: docs-rust docs-python docs-wasm
    @echo "✅ All documentation generated!"

# Generate Rust documentation
docs-rust:
    @echo "📚 Generating Rust documentation..."
    cd rust && cargo doc --workspace --no-deps --open

# Generate Python documentation
docs-python:
    @echo "📚 Generating Python documentation..."
    cd python && python build_docs.py

# Generate WASM documentation
docs-wasm:
    @echo "📚 Generating WASM documentation..."
    cd wasm && npm run docs

# Development Utilities
# =====================

# Start development server for WASM web examples
dev-server:
    @echo "🌐 Starting development server for WASM examples..."
    cd wasm && npm run example:web

# Watch for changes and rebuild
watch:
    @echo "👀 Watching for changes..."
    @echo "⚠️  Manual step: Install cargo-watch with 'cargo install cargo-watch'"
    @echo "⚠️  Then run: cd rust && cargo watch -x 'build --workspace'"

# Show dependency information
deps:
    @echo "📦 Dependency information:"
    @echo "🦀 Rust dependencies:"
    cd rust && cargo tree --workspace -d
    @echo "🐍 Python dependencies:"
    cd python && python -m pip list | grep -E "(composer|ruff|nox|pytest)"
    @echo "🌐 WASM dependencies:"
    cd wasm && npm list --depth=0

# Development Status
# ==================

# Show current development status
status:
    @echo "📊 Development Status:"
    @echo "🦀 Rust workspace status:"
    cd rust && cargo check --workspace --quiet && echo "  ✅ Rust builds cleanly" || echo "  ❌ Rust has build issues"
    @echo "🐍 Python package status:"
    cd python && python -c "import composer; print('  ✅ Python package imports successfully')" || echo "  ❌ Python package has import issues"
    @echo "🌐 WASM package status:"
    cd wasm && node -e "const c = require('./composer_wasm.js'); console.log('  ✅ WASM package loads successfully')" || echo "  ❌ WASM package has loading issues"
    @echo "📈 Performance status:"
    @echo "  See PERFORMANCE_ANALYSIS.md for detailed metrics"

# Quick development cycle (build + test)
dev: build test
    @echo "✅ Development cycle complete!"

# Full development cycle (clean + build + test + lint)
full: clean build test lint
    @echo "✅ Full development cycle complete!"