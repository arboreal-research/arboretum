#!/usr/bin/env python3
"""Tests for class handling."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.classes


class TestClassDeclaration:
    """Tests for class declaration extraction."""
    
    def test_simple_class(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple class extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "class_simple.cc"
            test_file.write_text("""
class MyClass {
public:
    int value;
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("CXXRecordDecl")
            assert count >= 1, f"Expected at least 1 CXXRecordDecl, got {count}"
    
    def test_class_with_methods(self, postgres_url: str, plugin_path: str) -> None:
        """Test class with methods."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "class_methods.cc"
            test_file.write_text("""
class MyClass {
public:
    int getValue() const { return value; }
    void setValue(int v) { value = v; }
private:
    int value;
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("CXXRecordDecl")
            assert count >= 1, f"Expected at least 1 CXXRecordDecl, got {count}"


class TestInheritance:
    """Tests for inheritance extraction."""
    
    def test_public_inheritance(self, postgres_url: str, plugin_path: str) -> None:
        """Test public inheritance."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "inheritance_public.cc"
            test_file.write_text("""
class Base {
public:
    int base_value;
};

class Derived : public Base {
public:
    int derived_value;
};
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("CXXRecordDecl")
            assert count >= 2, f"Expected at least 2 CXXRecordDecl, got {count}"
    
    def test_private_inheritance(self, postgres_url: str, plugin_path: str) -> None:
        """Test private inheritance."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "inheritance_private.cc"
            test_file.write_text("""
class Base {
public:
    int base_value;
};

class Derived : private Base {
public:
    int derived_value;
};
""")
            
            compiler.compile(str(test_file))


class TestVirtualMethods:
    """Tests for virtual method extraction."""
    
    def test_virtual_function(self, postgres_url: str, plugin_path: str) -> None:
        """Test virtual function extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "virtual.cc"
            test_file.write_text("""
class Base {
public:
    virtual void method() {}
};

class Derived : public Base {
public:
    void method() override {}
};
""")
            
            compiler.compile(str(test_file))
    
    def test_pure_virtual_function(self, postgres_url: str, plugin_path: str) -> None:
        """Test pure virtual function extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "pure_virtual.cc"
            test_file.write_text("""
class Abstract {
public:
    virtual void method() = 0;
};
""")
            
            compiler.compile(str(test_file))


class TestConstructor:
    """Tests for constructor extraction."""
    
    def test_default_constructor(self, postgres_url: str, plugin_path: str) -> None:
        """Test default constructor extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "constructor_default.cc"
            test_file.write_text("""
class MyClass {
public:
    MyClass() = default;
};
""")
            
            compiler.compile(str(test_file))
    
    def test_constructor_with_initialization(self, postgres_url: str, plugin_path: str) -> None:
        """Test constructor with member initialization."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "constructor_init.cc"
            test_file.write_text("""
class MyClass {
public:
    MyClass(int v) : value(v) {}
private:
    int value;
};
""")
            
            compiler.compile(str(test_file))


class TestDestructor:
    """Tests for destructor extraction."""
    
    def test_default_destructor(self, postgres_url: str, plugin_path: str) -> None:
        """Test default destructor extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "destructor.cc"
            test_file.write_text("""
class MyClass {
public:
    ~MyClass() = default;
};
""")
            
            compiler.compile(str(test_file))
    
    def test_virtual_destructor(self, postgres_url: str, plugin_path: str) -> None:
        """Test virtual destructor extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "virtual_destructor.cc"
            test_file.write_text("""
class Base {
public:
    virtual ~Base() = default;
};
""")
            
            compiler.compile(str(test_file))
