Development
===========

This guide covers setting up a development environment and contributing to ``read_aconity_layers``.

Development Setup
-----------------

Prerequisites
~~~~~~~~~~~~~

* Rust 1.70+ with Cargo
* Python 3.11+ 
* Poetry
* Git

Environment Setup
~~~~~~~~~~~~~~~~~

1. **Clone and setup the repository:**

   .. code-block:: bash

      git clone https://github.com/Cian-H/read_aconity_layers.git
      cd read_aconity_layers

2. **Install dependencies:**

   .. code-block:: bash

      poetry install --with dev,docs

3. **Setup pre-commit hooks:**

   .. code-block:: bash

      poetry run pre-commit install

4. **Build the Rust extension:**

   .. code-block:: bash

      poetry run maturin develop

Code Style and Quality
----------------------

This project uses several tools to maintain code quality:

Python Code
~~~~~~~~~~~

* **Ruff**: For linting and formatting
* **MyPy**: For type checking  
* **Pytest**: For testing

Run quality checks:

.. code-block:: bash

   # Format and lint Python code
   poetry run ruff format .
   poetry run ruff check .
   
   # Type checking
   poetry run mypy .
   
   # Run tests
   poetry run pytest

Rust Code
~~~~~~~~~

* **rustfmt**: For formatting
* **clippy**: For linting
* **cargo test**: For testing

Run quality checks:

.. code-block:: bash

   # Format Rust code
   cargo fmt
   
   # Lint Rust code 
   cargo clippy
   
   # Run Rust tests
   cargo test

Testing
-------

The project includes comprehensive tests for both Python and Rust components.

Running Tests
~~~~~~~~~~~~~

.. code-block:: bash

   # Run all tests
   poetry run pytest
   
   # Run with coverage
   poetry run pytest --cov=read_aconity_layers
   
   # Run Rust tests
   cargo test

Test Structure
~~~~~~~~~~~~~~

* **Python tests**: Located in ``tests/`` directory
* **Rust tests**: Integrated into ``src/rust_fn/mod.rs`` 
* **Property-based tests**: Uses ``arbtest`` for Rust property testing
* **Regression tests**: Validates against known good outputs

Adding Tests
~~~~~~~~~~~~

When adding new functionality:

1. **Add Rust tests** in the appropriate module
2. **Add Python integration tests** in ``tests/``
3. **Update regression tests** if output format changes
4. **Add property tests** for mathematical functions

Documentation
-------------

Building Documentation
~~~~~~~~~~~~~~~~~~~~~~~

**Prerequisites**: You need the Rust toolchain installed for ``sphinxcontrib-rust`` to work.

.. code-block:: bash

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
~~~~~~~~~~~~~~~~~~~~~~~

* **docs/conf.py**: Sphinx configuration
* **docs/index.rst**: Main documentation page
* **docs/python/**: Python API documentation
* **docs/rust/**: Rust API documentation
* **docs/*.rst**: User guides and tutorials

The documentation automatically generates API references from:

* Python docstrings and type hints
* Rust documentation comments (``///`` and ``//!``)
* Type stub files (``*.pyi``)

**Note**: For Rust API documentation to work properly, you need:

1. Rust toolchain installed (cargo, rustfmt)
2. Proper Rust doc comments in your source code
3. The ``sphinxcontrib-rust`` extension configured correctly

Contributing
------------

Workflow
~~~~~~~~

1. **Fork the repository** on GitHub
2. **Create a feature branch** from ``main``
3. **Make your changes** following the coding standards
4. **Add tests** for new functionality
5. **Update documentation** as needed
6. **Run the full test suite** to ensure everything works
7. **Submit a pull request**

Pre-commit Checks
~~~~~~~~~~~~~~~~~

The project uses pre-commit hooks that run automatically:

* Code formatting (Ruff, rustfmt)
* Linting (Ruff, Clippy)
* Type checking (MyPy)
* Version bump validation
* Poetry validation

These checks must pass before commits are accepted.

Release Process
~~~~~~~~~~~~~~~

1. **Update version** in ``Cargo.toml`` (triggers version validation)
2. **Update changelog** if applicable
3. **Ensure all tests pass**
4. **Create a release** on GitHub
5. **CI automatically builds and publishes** wheels to PyPI

Architecture Notes
------------------

The library is structured in two main components:

Rust Core (``src/rust_fn/``)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* **High-performance file I/O** using CSV reader
* **Parallel processing** with Rayon
* **Memory-efficient array operations** with ndarray
* **Coordinate correction algorithms**

Python Bindings (``src/lib.rs``)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* **PyO3 integration** for seamless Python interop
* **Error handling** conversion from Rust to Python exceptions
* **NumPy integration** for zero-copy array passing
* **Type annotations** via stub files

Performance Considerations
~~~~~~~~~~~~~~~~~~~~~~~~~~

* File I/O is the primary bottleneck
* Parallel processing scales well with core count
* Memory usage is proportional to dataset size
* Coordinate corrections use vectorized operations

Common Development Tasks
------------------------

Adding a New Function
~~~~~~~~~~~~~~~~~~~~~~

1. **Implement in Rust** (``src/rust_fn/mod.rs``)
2. **Add Python binding** (``src/lib.rs``)
3. **Update type stubs** (``read_layers.pyi``)
4. **Add tests** for both Rust and Python
5. **Update documentation**

Debugging Build Issues
~~~~~~~~~~~~~~~~~~~~~~

* **Check Rust version**: Must be 1.70+
* **Verify PyO3 compatibility**: Should match Python version
* **Clear build cache**: ``cargo clean`` and ``poetry env remove --all``
* **Check dependencies**: Ensure all dev dependencies are installed

Profiling Performance
~~~~~~~~~~~~~~~~~~~~~

For Rust code:

.. code-block:: bash

   # Profile with perf (Linux)
   cargo build --release
   perf record --call-graph=dwarf ./target/release/your_binary
   perf report

For Python integration:

.. code-block:: bash

   # Profile with py-spy
   pip install py-spy
   py-spy record -o profile.svg -- python your_script.py
