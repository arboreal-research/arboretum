#!/usr/bin/env python3
"""Tests for expression AST node handling."""

import pytest

from ..postgres import postgres_session, TestDatabase
from ..compiler import TestCompiler, CompilationError
from ..verifier import ResultVerifier, verify_all_tables_have_records

# Mark all tests in this module
pytestmark = pytest.mark.expressions


class TestLiteralExpressions:
    """Tests for literal expression extraction."""
    
    @pytest.fixture
    def test_compiler(self, postgres_url: str, plugin_path: str) -> TestCompiler:
        """Create compiler fixture."""
        return TestCompiler(plugin_path=plugin_path, postgres_url=postgres_url)
    
    def test_bool_literals(self, postgres_url: str, plugin_path: str) -> None:
        """Test boolean literal expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            # Create test file with bool literals
            test_file = db.build_dir / "bool_literals.cc"
            test_file.write_text("""
// Test bool literals
bool true_val = true;
bool false_val = false;
""")
            
            compiler.compile(str(test_file))
            
            # Verify bool literal table has records
            count = db.get_record_count("CXXBoolLiteralExpr")
            assert count >= 2, f"Expected at least 2 bool literals, got {count}"
    
    def test_integer_literals(self, postgres_url: str, plugin_path: str) -> None:
        """Test integer literal expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "int_literals.cc"
            test_file.write_text("""
// Test integer literals
int a = 42;
unsigned int b = 100U;
long c = 1000L;
long long d = 10000LL;
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("IntegerLiteral")
            assert count > 0, "Expected integer literal records"
    
    def test_floating_point_literals(self, postgres_url: str, plugin_path: str) -> None:
        """Test floating point literal expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "float_literals.cc"
            test_file.write_text("""
// Test floating point literals
float pi = 3.14f;
double e = 2.71828;
long double phi = 1.618L;
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("FloatingLiteral")
            assert count > 0, "Expected floating literal records"
    
    def test_string_literals(self, postgres_url: str, plugin_path: str) -> None:
        """Test string literal expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "string_literals.cc"
            test_file.write_text("""
// Test string literals
const char* greeting = "Hello, World!";
const char* empty = "";
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("StringLiteral")
            assert count > 0, "Expected string literal records"


class TestArithmeticExpressions:
    """Tests for arithmetic expression extraction."""
    
    @pytest.fixture
    def test_compiler(self, postgres_url: str, plugin_path: str) -> TestCompiler:
        """Create compiler fixture."""
        return TestCompiler(plugin_path=plugin_path, postgres_url=postgres_url)
    
    def test_binary_operators(self, postgres_url: str, plugin_path: str) -> None:
        """Test binary operator expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "binary_ops.cc"
            test_file.write_text("""
// Test binary operators
int add(int a, int b) { return a + b; }
int subtract(int a, int b) { return a - b; }
int multiply(int a, int b) { return a * b; }
int divide(int a, int b) { return a / b; }
int modulo(int a, int b) { return a % b; }
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("BinaryOperator")
            assert count > 0, "Expected BinaryOperator records"
    
    def test_unary_operators(self, postgres_url: str, plugin_path: str) -> None:
        """Test unary operator expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "unary_ops.cc"
            test_file.write_text("""
// Test unary operators
int negate(int x) { return -x; }
int increment(int x) { return ++x; }
int decrement(int x) { return --x; }
bool not_op(bool x) { return !x; }
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("UnaryOperator")
            assert count > 0, "Expected UnaryOperator records"
    
    def test_logical_operators(self, postgres_url: str, plugin_path: str) -> None:
        """Test logical operator expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "logical_ops.cc"
            test_file.write_text("""
// Test logical operators
bool and_op(bool a, bool b) { return a && b; }
bool or_op(bool a, bool b) { return a || b; }
bool xor_op(bool a, bool b) { return (a && !b) || (!a && b); }
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("BinaryOperator")
            # Should have AND and OR operators
            assert count > 0, "Expected BinaryOperator records for logical ops"


class TestConditionalExpressions:
    """Tests for conditional expression extraction."""
    
    def test_ternary_operator(self, postgres_url: str, plugin_path: str) -> None:
        """Test ternary operator expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "ternary.cc"
            test_file.write_text("""
// Test ternary operator
int max(int a, int b) { return a > b ? a : b; }
int abs_val(int x) { return x < 0 ? -x : x; }
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("ConditionalOperator")
            assert count > 0, "Expected ConditionalOperator records"


class TestExpressionTypes:
    """Tests for various expression type handling."""
    
    def test_decl_ref_expressions(self, postgres_url: str, plugin_path: str) -> None:
        """Test declaration reference expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "decl_ref.cc"
            test_file.write_text("""
// Test declaration references
int x = 10;
int y = x;
int z = x + y;
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("DeclRefExpr")
            assert count > 0, "Expected DeclRefExpr records"
    
    def test_member_expressions(self, postgres_url: str, plugin_path: str) -> None:
        """Test member access expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "member.cc"
            test_file.write_text("""
// Test member expressions
struct Point { int x; int y; };
Point p;
int x = p.x;
int y = p.y;
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("MemberExpr")
            assert count > 0, "Expected MemberExpr records"
