#!/usr/bin/env python3
"""Tests for template handling."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.templates


class TestClassTemplate:
    """Tests for class template extraction."""
    
    def test_class_template(self, postgres_url: str, plugin_path: str) -> None:
        """Test class template extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "class_template.cc"
            test_file.write_text("""
template<typename T>
class Container {
public:
    T value;
    void set(T v) { value = v; }
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("ClassTemplateDecl")
            assert count >= 1, f"Expected at least 1 ClassTemplateDecl, got {count}"
    
    def test_class_template_with_multiple_params(self, postgres_url: str, plugin_path: str) -> None:
        """Test class template with multiple parameters."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "class_template_multi.cc"
            test_file.write_text("""
template<typename T, typename U>
class Pair {
public:
    T first;
    U second;
};
""")
            
            compiler.compile(str(test_file))


class TestFunctionTemplate:
    """Tests for function template extraction."""
    
    def test_function_template(self, postgres_url: str, plugin_path: str) -> None:
        """Test function template extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "function_template.cc"
            test_file.write_text("""
template<typename T>
T max_value(T a, T b) {
    return a > b ? a : b;
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("FunctionTemplateDecl")
            assert count >= 1, f"Expected at least 1 FunctionTemplateDecl, got {count}"
    
    def test_function_template_with_multiple_params(self, postgres_url: str, plugin_path: str) -> None:
        """Test function template with multiple parameters."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "function_template_multi.cc"
            test_file.write_text("""
template<typename T, typename U>
auto add(T a, U b) -> decltype(a + b) {
    return a + b;
}
""")
            
            compiler.compile(str(test_file))


class TestTemplateSpecialization:
    """Tests for template specialization extraction."""
    
    def test_full_specialization(self, postgres_url: str, plugin_path: str) -> None:
        """Test full template specialization."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "specialization_full.cc"
            test_file.write_text("""
template<typename T>
class Container {
public:
    T value;
};

template<>
class Container<int> {
public:
    int value;
    int extra;
};
""")
            
            compiler.compile(str(test_file))
    
    def test_partial_specialization(self, postgres_url: str, plugin_path: str) -> None:
        """Test partial template specialization."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "specialization_partial.cc"
            test_file.write_text("""
template<typename T, typename U>
class Container {
public:
    T first;
    U second;
};

template<typename T>
class Container<T, T> {
public:
    T value;
};
""")
            
            compiler.compile(str(test_file))


class TestTemplateInstantiation:
    """Tests for template instantiation extraction."""
    
    def test_explicit_instantiation(self, postgres_url: str, plugin_path: str) -> None:
        """Test explicit template instantiation."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "instantiation_explicit.cc"
            test_file.write_text("""
template<typename T>
class Container {
public:
    T value;
};

template class Container<int>;
""")
            
            compiler.compile(str(test_file))
    
    def test_implicit_instantiation(self, postgres_url: str, plugin_path: str) -> None:
        """Test implicit template instantiation."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "instantiation_implicit.cc"
            test_file.write_text("""
template<typename T>
class Container {
public:
    T value;
};

void test() {
    Container<int> c;
    c.value = 42;
}
""")
            
            compiler.compile(str(test_file))


class TestNestedTemplate:
    """Tests for nested template extraction."""
    
    def test_nested_class_template(self, postgres_url: str, plugin_path: str) -> None:
        """Test nested class template."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "nested_class.cc"
            test_file.write_text("""
template<typename T>
class Outer {
public:
    template<typename U>
    class Inner {
    public:
        T outer_value;
        U inner_value;
    };
};
""")
            
            compiler.compile(str(test_file))
    
    def test_template_template_parameter(self, postgres_url: str, plugin_path: str) -> None:
        """Test template template parameter."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "template_template.cc"
            test_file.write_text("""
template<template<typename> class Container, typename T>
class Wrapper {
public:
    Container<T> value;
};
""")
            
            compiler.compile(str(test_file))
