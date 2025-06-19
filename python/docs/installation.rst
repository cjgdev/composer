Installation Guide
==================

This guide covers different ways to install the Composer Python package.

📦 **Quick Install**
--------------------

Install from PyPI (recommended):

.. code-block:: bash

   pip install composer

For development features:

.. code-block:: bash

   pip install composer[dev]

🛠️ **Development Installation**
-------------------------------

For contributing to Composer or using the latest features:

.. code-block:: bash

   # Clone the repository
   git clone https://github.com/cjgdev/composer.git
   cd composer/python

   # Create virtual environment
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate

   # Install in development mode
   pip install -e .

🔧 **Build from Source**
------------------------

Requirements:
   * Python 3.8 or higher
   * Rust toolchain (1.70.0 or higher)
   * `maturin <https://github.com/PyO3/maturin>`_ for building

.. code-block:: bash

   # Install Rust if not already installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env

   # Install maturin
   pip install maturin

   # Build and install
   cd python
   maturin develop --release

🌍 **Platform Support**
-----------------------

Composer supports the following platforms:

.. list-table:: Platform Compatibility
   :header-rows: 1

   * - Platform
     - Architecture
     - Python Versions
     - Status
   * - Linux
     - x86_64, aarch64
     - 3.8 - 3.12
     - ✅ Fully Supported
   * - macOS
     - x86_64, arm64 (M1/M2)
     - 3.8 - 3.12
     - ✅ Fully Supported
   * - Windows
     - x86_64
     - 3.8 - 3.12
     - ✅ Fully Supported

⚙️ **Optional Dependencies**
----------------------------

For enhanced functionality, install optional dependencies:

.. code-block:: bash

   # For testing
   pip install composer[test]

   # For development tools
   pip install composer[dev]

   # All optional dependencies
   pip install composer[all]

Available optional dependencies:

* **test**: pytest, pytest-cov, pytest-benchmark
* **dev**: ruff, nox, type checking tools
* **docs**: sphinx, sphinx-rtd-theme, myst-parser

🔍 **Verify Installation**
--------------------------

Test your installation:

.. code-block:: python

   import composer

   # Check version
   print(f"Composer version: {composer.__version__}")

   # Basic functionality test
   chord = composer.Chord.triad(1)
   print(f"C major triad: {chord}")

   # Performance test
   scale = composer.ScaleFingerprint.major()
   degrees = composer.get_stable_scale_degrees(chord, scale)
   print(f"Scale degrees: {degrees}")

Expected output:

.. code-block:: text

   Composer version: 2.35.2
   C major triad: 1
   Scale degrees: ['1', '3', '5']

🐛 **Troubleshooting**
---------------------

**Import Error**: ``ImportError: No module named 'composer'``
   * Ensure you're in the correct virtual environment
   * Try reinstalling: ``pip uninstall composer && pip install composer``

**Build Error**: ``error: Microsoft Visual C++ 14.0 is required``
   * On Windows, install `Microsoft C++ Build Tools <https://visualstudio.microsoft.com/visual-cpp-build-tools/>`_

**Performance Issues**: Slow chord operations
   * Ensure you installed the release build: ``maturin develop --release``
   * Check that binary wheels were used: ``pip show composer``

**Memory Errors**: High memory usage
   * Monitor with: ``composer.get_configuration_constants()['MEMORY_USAGE_MAX_MB']``
   * Consider using ``AiEngine`` with custom memory limits

🔄 **Updating**
---------------

Update to the latest version:

.. code-block:: bash

   pip install --upgrade composer

For development installations:

.. code-block:: bash

   cd composer/python
   git pull origin main
   maturin develop --release

💡 **Next Steps**
-----------------

* :doc:`quickstart` - Learn basic usage
* :doc:`tutorial/index` - Comprehensive tutorials
* :doc:`examples/index` - Practical examples
* :doc:`api/core` - Full API reference