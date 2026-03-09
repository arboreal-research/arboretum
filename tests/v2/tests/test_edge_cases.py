#!/usr/bin/env python3
"""Tests for edge cases and robustness."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler, CompilationError


pytestmark = pytest.mark.edge_cases


class TestEmptyFiles:
    """Tests for handling empty files."""
    
    def test_empty_file_no_crash(self, postgres_url: str, plugin_path: str) -> None:
        """Test that empty files don't crash the plugin."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "empty.cc"
            test_file.write_text("")
            
            # Should not crash
            compiler.compile(str(test_file))
    
    def test_file_with_only_whitespace(self, postgres_url: str, plugin_path: str) -> None:
        """Test file with only whitespace."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "whitespace.cc"
            test_file.write_text("   \n\n   \t  ")
            
            # Should not crash
            compiler.compile(str(test_file))


class TestLargeFiles:
    """Tests for handling large files."""
    
    def test_large_function(self, postgres_url: str, plugin_path: str) -> None:
        """Test large function with many statements."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "large_func.cc"
            
            # Generate a large function with many statements
            body = "\n    ".join([f"int var{i} = {i};" for i in range(100)])
            test_file.write_text(f"""
void test() {{
    {body}
}}
""")
            
            # Should handle without issues
            compiler.compile(str(test_file))
            
            count = db.get_record_count("VarDecl")
            assert count >= 100, f"Expected at least 100 VarDecl, got {count}"
    
    def test_file_with_many_functions(self, postgres_url: str, plugin_path: str) -> None:
        """Test file with many function declarations."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "many_funcs.cc"
            
            # Generate many function declarations
            functions = "\n".join([f"int func{i}() {{ return {i}; }}" for i in range(50)])
            test_file.write_text(functions)
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("FunctionDecl")
            assert count >= 50, f"Expected at least 50 FunctionDecl, got {count}"


class TestMacros:
    """Tests for macro handling."""
    
    def test_simple_macro(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple macro expansion."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "macro.cc"
            test_file.write_text("""
#define ADD(a, b) ((a) + (b))

int test() {
    return ADD(1, 2);
}
""")
            
            compiler.compile(str(test_file))
    
    def test_macro_in_function(self, postgres_url: str, plugin_path: str) -> None:
        """Test macro used in function."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "macro_func.cc"
            test_file.write_text("""
#define MAX(x, y) ((x) > (y) ? (x) : (y))

int test(int a, int b) {
    return MAX(a, b);
}
""")
            
            compiler.compile(str(test_file))


class TestUnicode:
    """Tests for Unicode identifier handling."""
    
    def test_unicode_identifiers(self, postgres_url: str, plugin_path: str) -> None:
        """Test Unicode identifiers in code."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "unicode.cc"
            test_file.write_text("""
// Test Unicode identifiers
int α = 1;
int β = 2;
int γ = α + β;

void αβγ() {
    int δ = 4;
}
""")
            
            # Should handle Unicode
            compiler.compile(str(test_file))
    
    def test_utf8_strings(self, postgres_url: str, plugin_path: str) -> None:
        """Test UTF-8 encoded strings."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "utf8.cc"
            test_file.write_text("""
const char* greeting = "Hello, 世界!";
const char* emoji = "😀🎉";
""")
            
            compiler.compile(str(test_file))


class TestNestedTemplates:
    """Tests for nested template handling."""
    
    def test_nested_template_classes(self, postgres_url: str, plugin_path: str) -> None:
        """Test deeply nested template classes."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "nested_templates.cc"
            test_file.write_text("""
template<typename T>
struct Outer {
    template<typename U>
    struct Inner {
        T value;
        U data;
    };
};

int test() {
    Outer<int>::Inner<double> obj;
    return 0;
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("ClassTemplateDecl")
            assert count >= 2, f"Expected at least 2 ClassTemplateDecl, got {count}"
    
    def test_template_specialization(self, postgres_url: str, plugin_path: str) -> None:
        """Test template specialization."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "specialization.cc"
            test_file.write_text("""
template<typename T>
struct Container {
    T value;
};

template<>
struct Container<int> {
    int value;
    int extra;
};
""")
            
            compiler.compile(str(test_file))


class TestExceptionHandling:
    """Tests for exception handling code."""
    
    def test_try_catch(self, postgres_url: str, plugin_path: str) -> None:
        """Test try-catch statement handling."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "exceptions.cc"
            test_file.write_text("""
void test() {
    try {
        throw 42;
    } catch (int e) {
        // handle
    }
}
""")
            
            compiler.compile(str(test_file))
    
    def test_noexcept(self, postgres_url: str, plugin_path: str) -> None:
        """Test noexcept specifier."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "noexcept.cc"
            test_file.write_text("""
int test() noexcept {
    return 42;
}
""")
            
            compiler.compile(str(test_file))


class TestLambdaExpressions:
    """Tests for lambda expression handling."""
    
    def test_simple_lambda(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple lambda expression."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "lambda.cc"
            test_file.write_text("""
int test() {
    auto f = [](int x) { return x * 2; };
    return f(5);
}
""")
            
            compiler.compile(str(test_file))
    
    def test_lambda_capture(self, postgres_url: str, plugin_path: str) -> None:
        """Test lambda with capture."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "lambda_capture.cc"
            test_file.write_text("""
int test() {
    int x = 10;
    auto f = [x]() { return x; };
    return f();
}
""")
            
            compiler.compile(str(test_file))


class TestAttributes:
    """Tests for attribute handling."""
    
    def test_decl_attributes(self, postgres_url: str, plugin_path: str) -> None:
        """Test declaration attributes."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "attributes.cc"
            test_file.write_text("""
[[nodiscard]] int test() {
    return 42;
}
""")
            
            compiler.compile(str(test_file))
    
    def test_pragma_directives(self, postgres_url: str, plugin_path: str) -> None:
        """Test pragma directives."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "pragma.cc"
            test_file.write_text("""
#pragma once

void test() {
}
""")
            
            compiler.compile(str(test_file))
