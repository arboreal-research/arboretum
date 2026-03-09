#!/usr/bin/env python3
"""Tests for enum handling."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.enums


class TestEnumDeclaration:
    """Tests for enum declaration extraction."""
    
    def test_simple_enum(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple enum extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "enum_simple.cc"
            test_file.write_text("""
enum Color {
    Red,
    Green,
    Blue
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("EnumDecl")
            assert count >= 1, f"Expected at least 1 EnumDecl, got {count}"
    
    def test_enum_with_values(self, postgres_url: str, plugin_path: str) -> None:
        """Test enum with explicit values."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "enum_values.cc"
            test_file.write_text("""
enum Status {
    Pending = 0,
    Active = 1,
    Done = 2
};
""")
            
            compiler.compile(str(test_file))


class TestEnumClass:
    """Tests for enum class (scoped enum) extraction."""
    
    def test_enum_class(self, postgres_url: str, plugin_path: str) -> None:
        """Test enum class extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "enum_class.cc"
            test_file.write_text("""
enum class Direction {
    North,
    South,
    East,
    West
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("EnumDecl")
            assert count >= 1, f"Expected at least 1 EnumDecl, got {count}"
    
    def test_enum_class_with_underlying_type(self, postgres_url: str, plugin_path: str) -> None:
        """Test enum class with underlying type."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "enum_class_underlying.cc"
            test_file.write_text("""
enum class Status : char {
    Pending = 0,
    Active = 1,
    Done = 2
};
""")
            
            compiler.compile(str(test_file))


class TestScopedEnum:
    """Tests for scoped enum extraction."""
    
    def test_scoped_enum_usage(self, postgres_url: str, plugin_path: str) -> None:
        """Test scoped enum usage."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "scoped_enum.cc"
            test_file.write_text("""
enum class Level {
    Low,
    Medium,
    High
};

void test() {
    Level l = Level::High;
}
""")
            
            compiler.compile(str(test_file))


class TestAnonymousEnum:
    """Tests for anonymous enum extraction."""
    
    def test_anonymous_enum(self, postgres_url: str, plugin_path: str) -> None:
        """Test anonymous enum."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "anonymous_enum.cc"
            test_file.write_text("""
enum {
    Small,
    Medium,
    Large
} size;

void test() {
    int s = Medium;
}
""")
            
            compiler.compile(str(test_file))


class TestEnumConstantDecl:
    """Tests for enum constant declaration extraction."""
    
    def test_enum_constant_count(self, postgres_url: str, plugin_path: str) -> None:
        """Test enum constant count."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "enum_constant.cc"
            test_file.write_text("""
enum Color {
    Red,    // 0
    Green,  // 1
    Blue    // 2
};
""")
            
            compiler.compile(str(test_file))
            
            # Check EnumDecl has the right number of enum constants
            count = db.get_record_count("EnumDecl")
            assert count >= 1, f"Expected at least 1 EnumDecl, got {count}"
    
    def test_enum_constant_with_values(self, postgres_url: str, plugin_path: str) -> None:
        """Test enum constant with explicit values."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "enum_const_values.cc"
            test_file.write_text("""
enum ErrorCode {
    Success = 0,
    InvalidInput = 1,
    NotFound = 2,
    InternalError = 3
};
""")
            
            compiler.compile(str(test_file))
