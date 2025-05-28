read_aconity_layers
==================================

.. image:: https://github.com/Cian-H/read_aconity_layers/workflows/CI/badge.svg
        :target: https://github.com/Cian-H/read_aconity_layers/actions

.. image:: https://github.com/Cian-H/read_aconity_layers/workflows/Python/badge.svg
        :target: https://github.com/Cian-H/read_aconity_layers/actions

.. image:: https://github.com/Cian-H/read_aconity_layers/workflows/Rust/badge.svg
        :target: https://github.com/Cian-H/read_aconity_layers/actions

.. image:: https://img.shields.io/pypi/dm/read-aconity-layers.svg
        :target: https://pypi.python.org/pypi/read-aconity-layers

.. image:: https://img.shields.io/github/tag/Cian-H/read_aconity_layers.svg
        :target: https://github.com/Cian-H/read_aconity_layers/releases

.. image:: https://img.shields.io/github/license/Cian-H/read_aconity_layers.svg
        :target: https://github.com/Cian-H/read_aconity_layers/blob/main/LICENSE

.. image:: https://readthedocs.org/projects/read-aconity-layers/badge/?version=latest
        :target: https://read-aconity-layers.readthedocs.io/en/latest/?badge=latest

.. image:: https://coveralls.io/repos/github/Cian-H/read-aconity-layers/badge.svg?branch=main
        :target: https://coveralls.io/github/Cian-H/read-aconity-layers?branch=main

.. image:: https://img.shields.io/badge/code%20style-Ruff-D7FF64.svg
        :target: https://github.com/astral-sh/ruff

==================================

A utility for fast reading of layer data from the aconity mini powder bed fusion machine.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   installation
   quickstart
   python/index
   rust/index
   development

Overview
--------

``read_aconity_layers`` is a high-performance Python library for reading and processing layer data from Aconity mini powder bed fusion machines. It's built with Rust for maximum performance and uses PyO3 for seamless Python integration.

Features
--------

* **Fast**: Built with Rust for high-performance data processing
* **Simple**: Easy-to-use Python API
* **Parallel**: Leverages Rayon for parallel processing of multiple files
* **Type-safe**: Full type annotations and stub files included

Quick Example
-------------

.. code-block:: python

   import read_aconity_layers as ral
   import numpy as np

   # Read all layers from a directory
   data = ral.read_layers("/path/to/layer/files/")
   
   # Read specific layer files
   files = ["/path/to/0.01.pcd", "/path/to/0.02.pcd"]
   data = ral.read_selected_layers(files)
   
   # Read a single layer
   layer = ral.read_layer("/path/to/0.01.pcd")

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
