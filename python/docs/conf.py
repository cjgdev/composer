# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os
import sys

# Add the src directory to the Python path so Sphinx can find the modules
sys.path.insert(0, os.path.abspath('../src'))

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'Composer'
copyright = '2024, Composer Contributors'
author = 'Composer Contributors'
release = '2.35.2'
version = '2.35.2'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.autodoc',        # Automatic documentation from docstrings
    'sphinx.ext.autosummary',    # Generate autodoc summaries
    'sphinx.ext.viewcode',       # Add source code links
    'sphinx.ext.napoleon',       # Support for Google and NumPy style docstrings
    'sphinx.ext.intersphinx',    # Link to other project's documentation
    'sphinx.ext.githubpages',    # Generate .nojekyll file for GitHub Pages
    'myst_parser',              # Support for Markdown files
    # 'sphinxcontrib_rust',       # Support for Rust documentation integration (disabled temporarily)
]

# MyST Parser configuration
myst_enable_extensions = [
    "colon_fence",
    "deflist",
    "fieldlist",
    "html_admonition",
    "html_image",
    "linkify",
    "replacements",
    "smartquotes",
    "strikethrough",
    "substitution",
    "tasklist",
]

# Rust crates for documentation integration (disabled temporarily)
# rust_crates = {
#     'composer_core': '../../rust/crates/composer-core',
#     'composer_ai': '../../rust/crates/composer-ai',
#     'composer_serialization': '../../rust/crates/composer-serialization',
#     'composer_config': '../../rust/crates/composer-config',
# }

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# Theme options
html_theme_options = {
    'canonical_url': '',
    'analytics_id': '',
    'logo_only': False,
    'display_version': True,
    'prev_next_buttons_location': 'bottom',
    'style_external_links': False,
    'vcs_pageview_mode': '',
    'style_nav_header_background': '#2980B9',
    # Toc options
    'collapse_navigation': False,
    'sticky_navigation': True,
    'navigation_depth': 4,
    'includehidden': True,
    'titles_only': False
}

html_title = "Composer Documentation"
html_short_title = "Composer"
html_logo = None
html_favicon = None

# Custom CSS
html_css_files = [
    'custom.css',
]

# -- Extension configuration -------------------------------------------------

# autodoc configuration
autodoc_default_options = {
    'members': True,
    'member-order': 'bysource',
    'special-members': '__init__',
    'undoc-members': True,
    'exclude-members': '__weakref__'
}

autodoc_class_signature = 'mixed'
autodoc_member_order = 'bysource'
autodoc_typehints = 'description'
autodoc_typehints_description_target = 'documented'

# autosummary configuration
autosummary_generate = True
autosummary_generate_overwrite = True

# napoleon configuration
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = False
napoleon_use_admonition_for_notes = False
napoleon_use_admonition_for_references = False
napoleon_use_ivar = False
napoleon_use_param = True
napoleon_use_rtype = True
napoleon_preprocess_types = False
napoleon_type_aliases = None
napoleon_attr_annotations = True

# intersphinx mapping
intersphinx_mapping = {
    'python': ('https://docs.python.org/3', None),
    'numpy': ('https://numpy.org/doc/stable/', None),
}

# -- Custom configuration for music theory documentation --------------------

# Add custom roles for musical notation
rst_prolog = """
.. role:: chord
   :class: chord

.. role:: degree
   :class: scale-degree

.. role:: roman
   :class: roman-numeral
"""

# Add version and build info
html_context = {
    "display_github": True,
    "github_user": "cjgdev",
    "github_repo": "composer",
    "github_version": "main",
    "conf_py_path": "/python/docs/",
}

# Custom domain for musical terms
def setup(app):
    """Custom setup for musical documentation."""
    app.add_css_file('custom.css')
    
    # Add custom directives for musical examples
    from docutils.parsers.rst import Directive
    from docutils.statemachine import ViewList
    from sphinx.util.docutils import SphinxDirective
    
    class MusicalExample(SphinxDirective):
        """Custom directive for musical examples."""
        has_content = True
        required_arguments = 0
        optional_arguments = 1
        
        def run(self):
            """Process the musical example."""
            content = '\n'.join(self.content)
            
            # Create a code block with special styling
            from docutils.nodes import container, literal_block
            node = container()
            node['classes'] = ['musical-example']
            
            code_block = literal_block(content, content)
            code_block['language'] = 'python'
            code_block['classes'] = ['musical-code']
            
            node.append(code_block)
            return [node]
    
    app.add_directive('musical-example', MusicalExample)