#!/usr/bin/env python3
"""Tests for namespace handling."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.namespaces


class TestNamespaceDeclaration:
    """Tests for namespace declaration extraction."""
    
    def test_simple_namespace(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple namespace extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "namespace_simple.cc"
            test_file.write_text("""
namespace mylib {
    int value = 42;
    
    int function() {
        return 0;
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("NamespaceDecl")
            assert count >= 1, f"Expected at least 1 NamespaceDecl, got {count}"
    
    def test_nested_namespace(self, postgres_url: str, plugin_path: str) -> None:
        """Test nested namespace extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "namespace_nested.cc"
            test_file.write_text("""
namespace outer {
    namespace inner {
        int value = 42;
    }
}
""")
            
            compiler.compile(str(test_file))


class TestAnonymousNamespace:
    """Tests for anonymous namespace extraction."""
    
    def test_anonymous_namespace(self, postgres_url: str, plugin_path: str) -> None:
        """Test anonymous namespace extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "namespace_anon.cc"
            test_file.write_text("""
namespace {
    int internal_value = 42;
    
    int internal_function() {
        return 0;
    }
}
""")
            
            compiler.compile(str(test_file))


class TestUsingDirective:
    """Tests for using directive extraction."""
    
    def test_using_namespace(self, postgres_url: str, plugin_path: str) -> None:
        """Test using namespace directive."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "using_namespace.cc"
            test_file.write_text("""
namespace foo {
    int value = 1;
}

using namespace foo;

void test() {
    int x = value;
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("UsingDirectiveDecl")
            assert count >= 1, f"Expected at least 1 UsingDirectiveDecl, got {count}"
    
    def test_using_directive_in_function(self, postgres_url: str, plugin_path: str) -> None:
        """Test using directive in function scope."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "using_function.cc"
            test_file.write_text("""
namespace foo {
    int value = 1;
}

void test() {
    using namespace foo;
    int x = value;
}
""")
            
            compiler.compile(str(test_file))


class TestUsingDeclaration:
    """Tests for using declaration extraction."""
    
    def test_using_declaration(self, postgres_url: str, plugin_path: str) -> None:
        """Test using declaration (specific name)."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "using_decl.cc"
            test_file.write_text("""
namespace foo {
    int value = 1;
    int other = 2;
}

using foo::value;

void test() {
    int x = value;
    int y = foo::other;
}
""")
            
            compiler.compile(str(test_file))


class TestNamespaceAlias:
    """Tests for namespace alias extraction."""
    
    def test_namespace_alias(self, postgres_url: str, plugin_path: str) -> None:
        """Test namespace alias."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "namespace_alias.cc"
            test_file.write_text("""
namespace very_long_namespace_name {
    int value = 1;
}

namespace alias = very_long_namespace_name;

void test() {
    int x = alias::value;
}
""")
            
            compiler.compile(str(test_file))
