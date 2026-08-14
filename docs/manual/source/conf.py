# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "grs-digital-clock"
copyright = "2026, GRS Digital Clock Contributors"
author = "SpaceLab"
title = "GRS Digital Clock"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

numfig = True
extensions = ["sphinxcontrib.bibtex"]

templates_path = ["_templates"]
exclude_patterns = []

# -- Options for sphinxcontrib-bibtex -----------------------------------------
# https://sphinxcontrib-bibtex.readthedocs.io/en/latest/usage.html

bibtex_bibfiles = ["references.bib"]


# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "sphinx_rtd_theme"
html_static_path = ["_static"]
