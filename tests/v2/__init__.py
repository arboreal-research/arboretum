# Arboretum Test Suite v2
"""Next-generation testing infrastructure for Arboretum."""

from pathlib import Path

# Package root
ROOT = Path(__file__).parent

# Test data path
TESTDATA = ROOT.parent / "testdata"

# Expected results path
EXPECTED = ROOT.parent / "expected"
