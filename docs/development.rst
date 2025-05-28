#############
 Development
#############

This guide covers setting up a development environment and contributing
to ``read_aconity_layers``.

*******************
 Development Setup
*******************

Prerequisites
=============

-  Rust 1.70+ with Cargo
-  Python 3.11+
-  Poetry
-  Git

Environment Setup
=================

#. **Clone and setup the repository:**

   .. code:: bash

      git clone https://github.com/Cian-H/read_aconity_layers.git
      cd read_aconity_layers

#. **Install dependencies:**

   .. code:: bash

      poetry install --with dev,docs

#. **Setup pre-commit hooks:**

   .. code:: bash

      poetry run pre-commit install

#. **Build the Rust extension:**

   .. code:: bash

      poetry run maturin develop

************************
 Code Style and Quality
************************

This project uses several tools to maintain code quality:

Python Code
===========

-  **Ruff**: For linting and formatting
-  **MyPy**: For type checking
-  **Pytest**: For testing

Run quality checks:

.. code:: bash

   # Format and lint Python code
   poetry run ruff format .
   poetry run ruff check .

   # Type checking
   poetry run mypy .

   # Run tests
   poetry run pytest

Rust Code
=========

-  **rustfmt**: For formatting
-  **clippy**: For linting
-  **cargo test**: For testing

Run quality checks:

.. code:: bash

   # Format Rust code
   cargo fmt

   # Lint Rust code
   cargo clippy

   # Run Rust tests
   cargo test

*********
 Testing
*********

The project includes comprehensive tests for both Python and Rust
components.

Running Tests
=============

.. code:: bash

   # Run all tests
   poetry run pytest

   # Run with coverage
   poetry run pytest --cov=read_aconity_layers

   # Run Rust tests
   cargo test

Test Structure
==============

-  **Python tests**: Located in ``tests/`` directory
-  **Rust tests**: Integrated into ``src/rust_fn/mod.rs``
-  **Property-based tests**: Uses ``arbtest`` for Rust property testing
-  **Regression tests**: Validates against known good outputs

Adding Tests
============

When adding new functionality:

#. **Add Rust tests** in the appropriate module
#. **Add Python integration tests** in ``tests/``
#. **Update regression tests** if output format changes
#. **Add property tests** for mathematical functions

***************
 Documentation
***************

Building Documentation
======================

**Prerequisites**: You need the Rust toolchain installed for
``sphinxcontrib-rust`` to work.

.. code:: bash

   # Install documentation dependencies
   poetry install --with docs

   # Build documentation
   cd docs
   make html

   # Or build manually
   poetry run sphinx-build -b html . _build/html

   # Serve locally (optional)
   make serve

Documentation Structure
=======================

-  **docs/conf.py**: Sphinx configuration
-  **docs/index.rst**: Main documentation page
-  **docs/python/**: Python API documentation
-  **docs/rust/**: Rust API documentation
-  **docs/\*.rst**: User guides and tutorials

The documentation automatically generates API references from:

-  Python docstrings and type hints
-  Rust documentation comments (``///`` and ``//!``)
-  Type stub files (``*.pyi``)

**Note**: For Rust API documentation to work properly, you need:

#. Rust toolchain installed (cargo, rustfmt)
#. Proper Rust doc comments in your source code
#. The ``sphinxcontrib-rust`` extension configured correctly

**************
 Contributing
**************

Workflow
========

#. **Fork the repository** on GitHub
#. **Create a feature branch** from ``main``
#. **Make your changes** following the coding standards
#. **Add tests** for new functionality
#. **Update documentation** as needed
#. **Run the full test suite** to ensure everything works
#. **Submit a pull request**

Pre-commit Checks
=================

The project uses pre-commit hooks that run automatically:

-  Code formatting (Ruff, rustfmt)
-  Linting (Ruff, Clippy)
-  Type checking (MyPy)
-  Version bump validation
-  Poetry validation

These checks must pass before commits are accepted.

Release Process
===============

#. **Update version** in ``Cargo.toml`` (triggers version validation)
#. **Update changelog** if applicable
#. **Ensure all tests pass**
#. **Create a release** on GitHub
#. **CI automatically builds and publishes** wheels to PyPI

********************
 Architecture Notes
********************

The library is structured in two main components:

Rust Core (``src/rust_fn/``)
============================

-  **High-performance file I/O** using CSV reader
-  **Parallel processing** with Rayon
-  **Memory-efficient array operations** with ndarray
-  **Coordinate correction algorithms**

Python Bindings (``src/lib.rs``)
================================

-  **PyO3 integration** for seamless Python interop
-  **Error handling** conversion from Rust to Python exceptions
-  **NumPy integration** for zero-copy array passing
-  **Type annotations** via stub files

Performance Considerations
==========================

-  File I/O is the primary bottleneck
-  Parallel processing scales well with core count
-  Memory usage is proportional to dataset size
-  Coordinate corrections use vectorized operations

**************************
 Common Development Tasks
**************************

Adding a New Function
=====================

#. **Implement in Rust** (``src/rust_fn/mod.rs``)
#. **Add Python binding** (``src/lib.rs``)
#. **Update type stubs** (``read_layers.pyi``)
#. **Add tests** for both Rust and Python
#. **Update documentation**

Debugging Build Issues
======================

-  **Check Rust version**: Must be 1.70+
-  **Verify PyO3 compatibility**: Should match Python version
-  **Clear build cache**: ``cargo clean`` and ``poetry env remove
   --all``
-  **Check dependencies**: Ensure all dev dependencies are installed

Profiling Performance
=====================

For Rust code:

.. code:: bash

   # Profile with perf (Linux)
   cargo build --release
   perf record --call-graph=dwarf ./target/release/your_binary
   perf report

For Python integration:

.. code:: bash

   # Profile with py-spy
   pip install py-spy
   py-spy record -o profile.svg -- python your_script.py
