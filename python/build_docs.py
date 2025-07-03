#!/usr/bin/env python3
"""
Build script for Composer Python documentation.

This script provides a convenient way to build the documentation with various options
and configurations. It's designed to work both locally and in CI/CD environments.
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path


def run_command(cmd: list[str], cwd: Path = None) -> bool:
    """Run a command and return True if successful."""
    print(f"Running: {' '.join(cmd)}")
    try:
        result = subprocess.run(
            cmd, cwd=cwd, check=True, capture_output=True, text=True
        )
        if result.stdout:
            print(result.stdout)
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error: {e}")
        if e.stdout:
            print("STDOUT:", e.stdout)
        if e.stderr:
            print("STDERR:", e.stderr)
        return False


def install_dependencies(use_uv: bool = True) -> bool:
    """Install documentation dependencies."""
    print("📦 Installing documentation dependencies...")

    if use_uv:
        cmd = ["uv", "pip", "install", "-e", ".[docs]"]
    else:
        cmd = ["pip", "install", "-e", ".[docs]"]

    return run_command(cmd)


def build_docs(
    source_dir: Path,
    build_dir: Path,
    format: str = "html",
    strict: bool = False,
    clean: bool = False,
) -> bool:
    """Build documentation using Sphinx."""

    if clean and build_dir.exists():
        print("🧹 Cleaning build directory...")
        import shutil

        shutil.rmtree(build_dir)

    print(f"📚 Building {format} documentation...")

    # Ensure build directory exists
    build_dir.mkdir(parents=True, exist_ok=True)

    # Build sphinx command
    cmd = ["sphinx-build", "-b", format, str(source_dir), str(build_dir)]

    if strict:
        cmd.extend(["-W", "--keep-going"])  # Treat warnings as errors

    cmd.extend(["-v"])  # Verbose output

    return run_command(cmd)


def check_links(source_dir: Path, build_dir: Path) -> bool:
    """Check documentation for broken links."""
    print("🔗 Checking for broken links...")

    linkcheck_dir = build_dir / "linkcheck"
    cmd = ["sphinx-build", "-b", "linkcheck", str(source_dir), str(linkcheck_dir)]

    return run_command(cmd)


def serve_docs(build_dir: Path, port: int = 8000) -> bool:
    """Serve documentation locally."""
    print(f"🌐 Serving documentation at http://localhost:{port}")

    html_dir = build_dir / "html"
    if not html_dir.exists():
        print("Error: HTML documentation not found. Build it first with --build")
        return False

    try:
        import http.server
        import socketserver
        import webbrowser

        os.chdir(html_dir)

        handler = http.server.SimpleHTTPRequestHandler
        httpd = socketserver.TCPServer(("", port), handler)

        # Open browser
        webbrowser.open(f"http://localhost:{port}")

        print("Press Ctrl+C to stop the server")
        httpd.serve_forever()

    except KeyboardInterrupt:
        print("\n👋 Stopping documentation server")
        return True
    except Exception as e:
        print(f"Error serving documentation: {e}")
        return False


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Build Composer Python documentation",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python build_docs.py --build                    # Build HTML docs
  python build_docs.py --build --strict           # Build with warnings as errors
  python build_docs.py --build --serve            # Build and serve locally
  python build_docs.py --check-links              # Check for broken links
  python build_docs.py --clean --build --strict   # Clean build with strict checking
        """,
    )

    # Action arguments
    parser.add_argument(
        "--install", action="store_true", help="Install documentation dependencies"
    )
    parser.add_argument("--build", action="store_true", help="Build HTML documentation")
    parser.add_argument(
        "--serve",
        action="store_true",
        help="Serve documentation locally after building",
    )
    parser.add_argument(
        "--check-links",
        action="store_true",
        help="Check documentation for broken links",
    )
    parser.add_argument(
        "--clean", action="store_true", help="Clean build directory before building"
    )

    # Build options
    parser.add_argument(
        "--strict", action="store_true", help="Treat warnings as errors"
    )
    parser.add_argument(
        "--format",
        default="html",
        choices=["html", "latex", "epub", "pdf"],
        help="Documentation format (default: html)",
    )
    parser.add_argument(
        "--port", type=int, default=8000, help="Port for local server (default: 8000)"
    )

    # Utility options
    parser.add_argument(
        "--no-uv", action="store_true", help="Use pip instead of uv for installation"
    )
    parser.add_argument(
        "--source-dir",
        type=Path,
        default=Path("docs"),
        help="Source directory (default: docs)",
    )
    parser.add_argument(
        "--build-dir",
        type=Path,
        default=Path("docs/_build"),
        help="Build directory (default: docs/_build)",
    )

    args = parser.parse_args()

    # If no action specified, show help
    if not any([args.install, args.build, args.serve, args.check_links]):
        parser.print_help()
        return 1

    # Ensure we're in the right directory
    script_dir = Path(__file__).parent
    os.chdir(script_dir)

    success = True

    # Install dependencies if requested
    if args.install:
        if not install_dependencies(use_uv=not args.no_uv):
            success = False

    # Build documentation if requested
    if args.build and success:
        if not build_docs(
            args.source_dir,
            args.build_dir / args.format,
            format=args.format,
            strict=args.strict,
            clean=args.clean,
        ):
            success = False

    # Check links if requested
    if args.check_links and success:
        if not check_links(args.source_dir, args.build_dir):
            success = False

    # Serve documentation if requested
    if args.serve and success:
        if not serve_docs(args.build_dir, args.port):
            success = False

    if success:
        print("✅ Documentation operations completed successfully!")

        if args.build:
            build_path = args.build_dir / args.format / "index.html"
            print(f"📖 Documentation available at: {build_path.absolute()}")

        return 0
    else:
        print("❌ Some operations failed. Check the output above.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
