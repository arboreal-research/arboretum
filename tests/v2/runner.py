#!/usr/bin/env python3
"""Test runner for Arboretum test suite."""

import argparse
import os
import subprocess
import sys
from pathlib import Path
from typing import List, Optional, Tuple

import pytest

from .postgres import TestDatabase
from .compiler import TestCompiler


class TestRunner:
    """Runs Arboretum tests with PostgreSQL isolation."""
    
    def __init__(
        self,
        postgres_url: str = "postgres://localhost:5432",
        verbose: bool = False,
        cleanup: bool = True,
        plugin_path: Optional[str] = None,
    ):
        self.postgres_url = postgres_url
        self.verbose = verbose
        self.cleanup = cleanup
        self.plugin_path = plugin_path or self._find_plugin()
        
        # Find test data directory
        self.testdata_dir = Path(__file__).parent.parent / "testdata"
        self.expected_dir = Path(__file__).parent.parent / "expected"
        
        if not self.testdata_dir.exists():
            raise RuntimeError(f"Test data directory not found: {self.testdata_dir}")
    
    def _find_plugin(self) -> str:
        """Find the Arboretum plugin."""
        # Try common build locations
        possible_paths = [
            Path(__file__).parent.parent.parent / "build" / "libarboretum.so",
            Path("/workspace/build/libarboretum.so"),
            Path(os.getcwd()) / "build" / "libarboretum.so",
        ]
        
        for path in possible_paths:
            if path.exists():
                return str(path)
        
        raise RuntimeError(
            "Could not find libarboretum.so. Build the project first with: make arboretum"
        )
    
    def run(self, test_path: Optional[str] = None) -> int:
        """Run tests using pytest."""
        # Build pytest arguments
        pytest_args = [__file__]  # This file for self-test
        
        if test_path:
            # Run specific test
            pytest_args.append(test_path)
        else:
            # Run all tests in tests/v2/tests
            pytest_args.append(str(Path(__file__).parent / "tests"))
        
        # Add verbosity if requested
        if self.verbose:
            pytest_args.extend(["-v", "-s"])
        
        # Add markers
        pytest_args.extend(["-m", "not slow"])  # Skip slow tests by default
        
        # Run pytest
        exit_code = pytest.main(pytest_args)
        return exit_code


def main():
    """Main entry point for the test runner."""
    parser = argparse.ArgumentParser(
        description="Run Arboretum tests with PostgreSQL isolation"
    )
    parser.add_argument(
        "test",
        nargs="?",
        help="Specific test file or directory to run",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="Enable verbose output",
    )
    parser.add_argument(
        "--postgres-url",
        default="postgres://localhost:5432",
        help="PostgreSQL connection URL",
    )
    parser.add_argument(
        "--plugin-path",
        default=None,
        help="Path to libarboretum.so plugin",
    )
    parser.add_argument(
        "--cleanup",
        action="store_true",
        default=True,
        help="Clean up test databases after tests",
    )
    parser.add_argument(
        "--no-cleanup",
        action="store_false",
        dest="cleanup",
        help="Keep test databases after tests (for debugging)",
    )
    parser.add_argument(
        "--version",
        action="store_true",
        help="Print version information",
    )
    
    args = parser.parse_args()
    
    if args.version:
        print("Arboretum Test Suite v2.0.0")
        return 0
    
    try:
        runner = TestRunner(
            postgres_url=args.postgres_url,
            verbose=args.verbose,
            cleanup=args.cleanup,
            plugin_path=args.plugin_path,
        )
        return runner.run(args.test)
    except RuntimeError as e:
        print(f"ERROR: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
