import nox

PYTHON_VERSIONS = ["3.8", "3.9", "3.10", "3.11", "3.12"]
# By default, run linting and the tests for the default Python version.
nox.options.sessions = ["lint", "tests"]


@nox.session(python=False)
def lint(session: nox.Session) -> None:
    """Runs ruff linting and formatting checks."""
    session.run("uv", "pip", "install", "ruff")
    session.run("ruff", "check", ".")
    session.run("ruff", "format", "--check", ".")


@nox.session(python=False)
def format(session: nox.Session) -> None:
    """Runs ruff formatting."""
    session.run("uv", "pip", "install", "ruff")
    session.run("ruff", "format", ".")
    session.run("ruff", "check", "--fix", ".")


@nox.session(python=False)
def types(session: nox.Session) -> None:
    """Runs the ty type checker."""
    # Install project in editable mode so ty can resolve its imports.
    session.run("uv", "pip", "install", "-e", ".")
    # Use uvx to run ty in an ephemeral environment.
    session.run("uvx", "ty", "check", "src/composer")


@nox.session(python=PYTHON_VERSIONS)
def tests(session: nox.Session) -> None:
    """Runs the test suite against multiple Python versions."""
    # Use uv to install dependencies into the nox-managed venv.
    session.run("uv", "pip", "install", "-e", ".[test]")
    session.run("pytest", *session.posargs)


@nox.session(python=False)
def build(session: nox.Session) -> None:
    """Build the package using maturin."""
    session.run("uv", "pip", "install", "maturin")
    session.run("maturin", "build", "--release")


@nox.session(python=False)
def develop(session: nox.Session) -> None:
    """Install the package in development mode."""
    session.run("uv", "pip", "install", "-e", ".")


@nox.session(python=False)
def docs(session: nox.Session) -> None:
    """Build the documentation using Sphinx."""
    session.run("uv", "pip", "install", "-e", ".[docs]")
    session.chdir("docs")
    session.run("sphinx-build", "-b", "html", ".", "_build/html")


@nox.session(python=False)
def docs_live(session: nox.Session) -> None:
    """Build documentation with live reload for development."""
    session.run("uv", "pip", "install", "-e", ".[docs]", "sphinx-autobuild")
    session.chdir("docs")
    session.run("sphinx-autobuild", ".", "_build/html", "--open-browser")


@nox.session(python=False)
def docs_linkcheck(session: nox.Session) -> None:
    """Check documentation for broken links."""
    session.run("uv", "pip", "install", "-e", ".[docs]")
    session.chdir("docs")
    session.run("sphinx-build", "-b", "linkcheck", ".", "_build/linkcheck")
