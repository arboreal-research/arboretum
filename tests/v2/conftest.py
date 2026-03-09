#!/usr/bin/env python3
"""Pytest configuration for Arboretum tests."""

import os
import pytest


def pytest_addoption(parser):
    """Add command-line options for pytest."""
    parser.addoption(
        "--postgres-url",
        action="store",
        default="postgres://localhost:5432",
        help="PostgreSQL connection URL",
    )
    parser.addoption(
        "--plugin-path",
        action="store",
        default=None,
        help="Path to libarboretum.so plugin",
    )
    parser.addoption(
        "--clean",
        action="store_true",
        default=False,
        help="Clean test databases after each test",
    )


@pytest.fixture(scope="session")
def postgres_url(request) -> str:
    """Get PostgreSQL URL from command line option."""
    return request.config.getoption("--postgres-url")


@pytest.fixture(scope="session")
def plugin_path(request) -> str:
    """Get plugin path from command line option."""
    path = request.config.getoption("--plugin-path")
    if path is None:
        # Try to find default plugin location
        default_paths = [
            "/workspace/build/libarboretum.so",
            "/workspace/tests/../build/libarboretum.so",
        ]
        for p in default_paths:
            if os.path.exists(p):
                path = p
                break
        
        if path is None:
            pytest.fail(
                "Plugin path not specified and could not be found. "
                "Run 'make arboretum' first or use --plugin-path option."
            )
    return path


@pytest.fixture(scope="function")
def test_db(request, postgres_url) -> "TestDatabase":
    """Create a test database for each test function.
    
    This fixture creates a database before each test and drops it after.
    Set --clean=false to keep databases for debugging.
    """
    from .postgres import TestDatabase
    
    db = TestDatabase(postgres_url)
    db.create()
    
    yield db
    
    # Cleanup after test
    cleanup = not request.config.getoption("--clean", default=True)
    if cleanup:
        db.drop()


@pytest.fixture(scope="function")
def test_compiler(request, postgres_url, plugin_path) -> "TestCompiler":
    """Create a compiler fixture for each test.
    
    The compiler is configured to use the test database URL.
    """
    from .compiler import TestCompiler
    
    db = TestDatabase(postgres_url)
    db.create()
    
    compiler = TestCompiler(
        plugin_path=plugin_path,
        postgres_url=db.connection_url,
    )
    
    yield compiler
    
    # Cleanup after test
    cleanup = not request.config.getoption("--clean", default=True)
    if cleanup:
        db.drop()


def pytest_configure(config):
    """Register custom markers."""
    config.addinivalue_line("markers", "expressions: tests for expression handling")
    config.addinivalue_line("markers", "statements: tests for statement handling")
    config.addinivalue_line("markers", "declarations: tests for declaration handling")
    config.addinivalue_line("markers", "types: tests for type handling")
    config.addinivalue_line("markers", "classes: tests for class handling")
    config.addinivalue_line("markers", "templates: tests for template handling")
    config.addinivalue_line("markers", "enums: tests for enum handling")
    config.addinivalue_line("markers", "namespaces: tests for namespace handling")
    config.addinivalue_line("markers", "schema: tests for schema generation")
    config.addinivalue_line("markers", "integrations: end-to-end integration tests")
    config.addinivalue_line("markers", "edge_cases: tests for edge cases and robustness")
    config.addinivalue_line("markers", "slow: tests that take longer to run")
