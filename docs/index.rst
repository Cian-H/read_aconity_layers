read_aconity_layers
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
