# GitHub Actions CI/CD Workflows

This directory contains the automated workflows for the Composer monorepo project, implementing the architecture specified in `monorepo-structure.md`.

## 🔧 Workflows Overview

### `ci.yml` - Continuous Integration
**Triggers:** Push to `main`/`develop`, Pull Requests, Manual dispatch

**Key Features:**
- **Smart Path-Based Execution**: Uses `dorny/paths-filter` to run only affected components
- **Multi-Job Architecture**: Separate jobs for Rust, Python, WASM, docs, and security
- **Comprehensive Testing**: Cross-platform and multi-version testing
- **Performance Optimized**: Modern caching with `Swatinem/rust-cache` and uv

**Jobs:**
1. **`changes`**: Detects which components have changed
2. **`rust-ci`**: Rust workspace testing with cargo check, clippy, test, bench
3. **`python-ci`**: Multi-version Python testing (3.8-3.12) across platforms
4. **`wasm-ci`**: WebAssembly testing in Node.js and headless browsers
5. **`docs-ci`**: Documentation building and link checking
6. **`security`**: Rust security audit and dependency review

### `release.yml` - Release Pipeline
**Triggers:** Version tags (`v*`), Manual dispatch

**Key Features:**
- **Multi-Platform Builds**: Linux, macOS (x64/ARM64), Windows
- **Professional Release Notes**: Auto-generated with project details
- **Comprehensive Testing**: Full test suite before publishing
- **Secure Publishing**: Uses trusted publishing for PyPI and NPM

**Jobs:**
1. **`create-release`**: Creates GitHub release with generated notes
2. **`build-python-wheels`**: Builds Python wheels for all platforms/versions
3. **`publish-python`**: Publishes to PyPI with verification
4. **`build-wasm-package`**: Builds WebAssembly packages for multiple targets
5. **`publish-wasm`**: Publishes to NPM registry
6. **`build-docs`**: Generates and deploys documentation

## 🎯 Path-Based Conditional Execution

The workflows implement intelligent execution based on file changes:

| Changed Path | Triggers |
|--------------|----------|
| `rust/crates/composer-core/**` | All jobs (core affects everything) |
| `rust/crates/composer-ffi/**` | Rust + Python + Docs |
| `rust/crates/composer-wasm/**` | Rust + WASM |
| `python/**` | Python + Docs |
| `wasm/**` | WASM only |
| `.github/workflows/**` | All jobs |

## 🚀 Performance Optimizations

- **Modern Caching**: Uses `Swatinem/rust-cache` for superior Rust caching
- **Selective Testing**: Matrix exclusions for efficient multi-platform testing
- **Build Artifacts**: Efficient artifact sharing between jobs
- **Parallel Execution**: Independent jobs run concurrently

## 🔐 Security Features

- **Minimal Permissions**: Each job has only required permissions
- **Environment Protection**: Release jobs use GitHub environments
- **Dependency Scanning**: Automated security audits with `rustsec/audit-check`
- **Trusted Publishing**: Uses OIDC for secure PyPI/NPM publishing

## 📋 Requirements

### Repository Secrets
- `PYPI_API_TOKEN`: For PyPI publishing (or use trusted publishing)
- `NPM_TOKEN`: For NPM publishing

### Repository Settings
- Enable GitHub Pages for documentation deployment
- Configure release environment protection rules
- Set up trusted publishing for PyPI (optional but recommended)

## 🔍 Monitoring and Debugging

### CI Status Checks
- All jobs must pass for PR merging
- Security scans run on every PR
- Documentation builds verified before merge

### Release Validation
- Comprehensive testing across all platforms
- Package integrity verification
- Artifact upload and download validation
- Documentation deployment verification

## 📚 Architecture Alignment

These workflows implement the best practices from `monorepo-structure.md`:

✅ **Path-based conditional execution** (Part 4, Section 1)  
✅ **Multi-platform Python wheel building** (Part 2, Release section)  
✅ **WebAssembly browser + Node.js testing** (Part 3, Testing section)  
✅ **Intelligent dependency matrix** (Part 4, CI Matrix)  
✅ **Modern tooling integration** (uv, ruff, nox, wasm-pack)  
✅ **Security and quality gates** (clippy, audit, dependency review)

The implementation exceeds the specifications by adding security scanning, comprehensive documentation generation, and modern caching strategies for optimal performance.