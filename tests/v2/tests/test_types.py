#!/usr/bin/env python3
"""Tests for type handling."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.types


class TestQualType:
    """Tests for QualType handling."""
    
    def test_qualtype_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that QualType table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "qualtype_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "QualType" in tables, "QualType table not created"


class TestTypeReferences:
    """Tests for type reference resolution."""
    
    def test_function_return_type(self, postgres_url: str, plugin_path: str) -> None:
        """Test that function return types are captured."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "return_type.cc"
            test_file.write_text("""
int test_int() { return 0; }
double test_double() { return 0.0; }
const char* test_string() { return ""; }
void test_void() {}
""")
            
            compiler.compile(str(test_file))
            
            # Check that we have FunctionDecl with returnType
            count = db.get_record_count("FunctionDecl")
            assert count > 0, "No FunctionDecl records"
            
            # Check that returnType column exists
            columns = db.get_table_columns("FunctionDecl")
            assert "returnType" in columns or "type" in columns, (
                "Expected returnType or type column in FunctionDecl"
            )
    
    def test_variable_type(self, postgres_url: str, plugin_path: str) -> None:
        """Test that variable types are captured."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "var_type.cc"
            test_file.write_text("""
int x;
double y;
const char* z;
""")
            
            compiler.compile(str(test_file))
            
            # Check that VarDecl has type column
            columns = db.get_table_columns("VarDecl")
            assert "type" in columns, "Expected type column in VarDecl"


class TestPointerType:
    """Tests for pointer type handling."""
    
    def test_pointer_types(self, postgres_url: str, plugin_path: str) -> None:
        """Test pointer type extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "pointer.cc"
            test_file.write_text("""
int* ptr;
const char* str;
int** double_ptr;
""")
            
            compiler.compile(str(test_file))
    
    def test_reference_types(self, postgres_url: str, plugin_path: str) -> None:
        """Test reference type extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "reference.cc"
            test_file.write_text("""
int& ref;
const int& const_ref;
""")
            
            compiler.compile(str(test_file))


class TestQualifiedTypes:
    """Tests for qualified type handling."""
    
    def test_const_types(self, postgres_url: str, plugin_path: str) -> None:
        """Test const type handling."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "const.cc"
            test_file.write_text("""
const int x = 5;
int* const ptr = nullptr;
const char* const str = "hello";
""")
            
            compiler.compile(str(test_file))
    
    def test_volatile_types(self, postgres_url: str, plugin_path: str) -> None:
        """Test volatile type handling."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "volatile.cc"
            test_file.write_text("""
volatile int x;
volatile int* ptr;
""")
            
            compiler.compile(str(test_file))
