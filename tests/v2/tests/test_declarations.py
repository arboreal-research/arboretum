#!/usr/bin/env python3
"""Tests for declaration AST node handling."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.declarations


class TestFunctionDeclarations:
    """Tests for function declaration extraction."""
    
    def test_simple_function(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple function declaration extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "func_simple.cc"
            test_file.write_text("""
int add(int a, int b) {
    return a + b;
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("FunctionDecl")
            assert count >= 1, f"Expected at least 1 FunctionDecl, got {count}"
    
    def test_function_with_parameters(self, postgres_url: str, plugin_path: str) -> None:
        """Test function with parameters."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "func_params.cc"
            test_file.write_text("""
int multiply(int a, int b, int c) {
    return a * b * c;
}

void no_params() {}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("FunctionDecl")
            assert count >= 2, f"Expected at least 2 FunctionDecl, got {count}"
    
    def test_constexpr_function(self, postgres_url: str, plugin_path: str) -> None:
        """Test constexpr function."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "constexpr.cc"
            test_file.write_text("""
constexpr int square(int x) {
    return x * x;
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("FunctionDecl")
            assert count >= 1, f"Expected at least 1 FunctionDecl, got {count}"


class TestVariableDeclarations:
    """Tests for variable declaration extraction."""
    
    def test_simple_variable(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple variable declaration."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "var_simple.cc"
            test_file.write_text("""
int x = 42;
const char* msg = "Hello";
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("VarDecl")
            assert count >= 2, f"Expected at least 2 VarDecl, got {count}"
    
    def test_static_variable(self, postgres_url: str, plugin_path: str) -> None:
        """Test static variable declaration."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "var_static.cc"
            test_file.write_text("""
static int counter = 0;
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("VarDecl")
            assert count >= 1, f"Expected at least 1 VarDecl, got {count}"


class TestTypeDeclarations:
    """Tests for type declaration extraction."""
    
    def test_struct_declaration(self, postgres_url: str, plugin_path: str) -> None:
        """Test struct declaration."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "struct.cc"
            test_file.write_text("""
struct Point {
    int x;
    int y;
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("CXXRecordDecl")
            assert count >= 1, f"Expected at least 1 CXXRecordDecl, got {count}"
    
    def test_class_declaration(self, postgres_url: str, plugin_path: str) -> None:
        """Test class declaration."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "class.cc"
            test_file.write_text("""
class MyClass {
public:
    int value;
    void method();
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("CXXRecordDecl")
            assert count >= 1, f"Expected at least 1 CXXRecordDecl, got {count}"


class TestEnumDeclarations:
    """Tests for enum declaration extraction."""
    
    def test_simple_enum(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple enum."""
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
    
    def test_enum_class(self, postgres_url: str, plugin_path: str) -> None:
        """Test enum class (scoped enum)."""
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


class TestNamespaceDeclarations:
    """Tests for namespace declaration extraction."""
    
    def test_simple_namespace(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple namespace."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "namespace.cc"
            test_file.write_text("""
namespace mylib {
    int value = 42;
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("NamespaceDecl")
            assert count >= 1, f"Expected at least 1 NamespaceDecl, got {count}"


class TestUsingDeclarations:
    """Tests for using declaration extraction."""
    
    def test_using_namespace(self, postgres_url: str, plugin_path: str) -> None:
        """Test using namespace declaration."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "using.cc"
            test_file.write_text("""
namespace foo {
    int x = 1;
}
using namespace foo;
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("UsingDirectiveDecl")
            assert count >= 1, f"Expected at least 1 UsingDirectiveDecl, got {count}"
