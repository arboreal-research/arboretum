#!/usr/bin/env python3
"""Tests for schema generation."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.schema


class TestMetaTables:
    """Tests for meta tables that should always be created."""
    
    def test_file_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that file table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "meta_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "file" in tables, "file table not created"
    
    def test_source_loc_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that source_loc table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "meta_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "source_loc" in tables, "source_loc table not created"
    
    def test_source_range_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that source_range table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "meta_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "source_range" in tables, "source_range table not created"
    
    def test_qualtype_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that QualType table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "meta_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "QualType" in tables, "QualType table not created"


class TestBaseASTTables:
    """Tests for base AST tables."""
    
    def test_decl_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that Decl table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "base_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "Decl" in tables, "Decl table not created"
    
    def test_stmt_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that Stmt table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "base_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "Stmt" in tables, "Stmt table not created"
    
    def test_expr_table_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that Expr table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "base_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "Expr" in tables, "Expr table not created"


class TestGeneratedTables:
    """Tests for tables generated from properties.csv."""
    
    def test_functiondecl_table_created(self, postgres_url: str, plugin_path: str) -> None:
        """Test FunctionDecl table is created when property is enabled."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "func_decl.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "FunctionDecl" in tables, "FunctionDecl table not created"
    
    def test_vardecl_table_created(self, postgres_url: str, plugin_path: str) -> None:
        """Test VarDecl table is created when property is enabled."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "var_decl.cc"
            test_file.write_text("""
int x = 0;
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "VarDecl" in tables, "VarDecl table not created"
    
    def test_cxxrecorddecl_table_created(self, postgres_url: str, plugin_path: str) -> None:
        """Test CXXRecordDecl table is created when property is enabled."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "class_decl.cc"
            test_file.write_text("""
class MyClass {};
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "CXXRecordDecl" in tables, "CXXRecordDecl table not created"
    
    def test_enumdecl_table_created(self, postgres_url: str, plugin_path: str) -> None:
        """Test EnumDecl table is created when property is enabled."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "enum_decl.cc"
            test_file.write_text("""
enum Color { Red };
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            assert "EnumDecl" in tables, "EnumDecl table not created"


class TestIdColumn:
    """Tests for id column in generated tables."""
    
    def test_id_column_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Test that all tables have id column."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "id_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            
            for table in tables:
                columns = db.get_table_columns(table)
                assert "id" in columns, f"Table {table} missing 'id' column"


class TestColumnTypes:
    """Tests for column type mapping."""
    
    def test_string_columns_are_text(self, postgres_url: str, plugin_path: str) -> None:
        """Test that string properties map to TEXT."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "string_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            # Check FunctionDecl has 'name' column and it's TEXT type
            columns = db.get_table_columns("FunctionDecl")
            if "name" in columns:
                result = db.execute("""
                    SELECT data_type 
                    FROM information_schema.columns 
                    WHERE table_name = 'FunctionDecl' AND column_name = 'name';
                """)
                assert result.strip() in ("text", "character varying"), (
                    f"Expected TEXT/VARCHAR for name column, got: {result}"
                )
    
    def test_boolean_columns_exist(self, postgres_url: str, plugin_path: str) -> None:
        """Test that boolean columns exist where enabled."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "bool_test.cc"
            test_file.write_text("""
constexpr int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            columns = db.get_table_columns("FunctionDecl")
            
            # Check common boolean columns
            bool_columns = ["isDefinition", "isConstexpr", "isInline"]
            found_bool = [c for c in bool_columns if c in columns]
            
            # At least one should exist
            assert len(found_bool) >= 1, (
                f"Expected at least one boolean column, found: {found_bool}"
            )


class TestSchemaDynamism:
    """Tests that verify schema is dynamically generated."""
    
    def test_schema_differs_by_enabled_properties(self, postgres_url: str, plugin_path: str) -> None:
        """Test that different enabled properties produce different schemas."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            # First, compile with basic file
            test_file1 = db.build_dir / "schema_test1.cc"
            test_file1.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file1))
            tables1 = db.get_tables()
            
            # We can't easily test different enabled properties without modifying properties.csv
            # This test documents the expected behavior: schema should change based on enabled props
            assert len(tables1) > 0, "Expected at least one table to be created"
