#!/usr/bin/env python3
"""End-to-end integration tests."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler
from ..verifier import ResultVerifier, verify_all_tables_have_records


pytestmark = pytest.mark.integration


class TestCompletePipeline:
    """Tests for the complete extraction pipeline."""
    
    def test_basic_extraction(self, postgres_url: str, plugin_path: str) -> None:
        """Test basic extraction with multiple AST nodes."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "basic.cc"
            test_file.write_text("""
// Basic test with various AST nodes
int add(int a, int b) {
    return a + b;
}

class Calculator {
public:
    int multiply(int a, int b) {
        return a * b;
    }
};

void test_if(int x) {
    if (x > 0) {
        // positive
    } else {
        // non-positive
    }
}
""")
            
            compiler.compile(str(test_file))
            
            # Verify multiple tables have records
            tables_with_data = []
            for table in ["FunctionDecl", "CXXRecordDecl", "IfStmt", "BinaryOperator"]:
                count = db.get_record_count(table)
                if count > 0:
                    tables_with_data.append(table)
            
            assert len(tables_with_data) >= 3, (
                f"Expected data in at least 3 tables, got: {tables_with_data}"
            )
    
    def test_multiple_files(self, postgres_url: str, plugin_path: str) -> None:
        """Test extraction across multiple files."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            # Create multiple test files
            files = []
            
            # File 1: Functions
            f1 = db.build_dir / "file1.cc"
            f1.write_text("""
int func1() { return 1; }
int func2() { return 2; }
""")
            files.append(str(f1))
            
            # File 2: Classes
            f2 = db.build_dir / "file2.cc"
            f2.write_text("""
class A {};
class B {};
""")
            files.append(str(f2))
            
            # Compile all files
            compiler.compile_multiple(files)
            
            # Verify data from multiple files
            func_count = db.get_record_count("FunctionDecl")
            class_count = db.get_record_count("CXXRecordDecl")
            
            assert func_count >= 2, f"Expected at least 2 functions, got {func_count}"
            assert class_count >= 2, f"Expected at least 2 classes, got {class_count}"


class TestSchemaGeneration:
    """Tests for schema generation based on properties."""
    
    def test_table_creation(self, postgres_url: str, plugin_path: str) -> None:
        """Verify tables are created for enabled properties."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "schema_test.cc"
            test_file.write_text("""
int test() { return 0; }
""")
            
            compiler.compile(str(test_file))
            
            tables = db.get_tables()
            
            # Check that core tables were created
            core_tables = ["FunctionDecl", "VarDecl", "Stmt", "Expr", "Decl"]
            created_tables = [t for t in core_tables if t in tables]
            
            assert len(created_tables) >= 2, (
                f"Expected at least 2 core tables, got: {created_tables}"
            )
    
    def test_column_types(self, postgres_url: str, plugin_path: str) -> None:
        """Verify column types are correctly mapped."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "column_types.cc"
            test_file.write_text("""
int test(int x) {
    return x;
}
""")
            
            compiler.compile(str(test_file))
            
            # Check FunctionDecl columns
            columns = db.get_table_columns("FunctionDecl")
            
            expected_columns = ["id", "name", "isDefinition", "beginLoc", "endLoc"]
            found_columns = [c for c in expected_columns if c in columns]
            
            assert len(found_columns) >= 3, (
                f"Expected at least 3 columns in FunctionDecl, got: {found_columns}"
            )


class TestDataIntegrity:
    """Tests for data integrity after extraction."""
    
    def test_no_empty_tables(self, postgres_url: str, plugin_path: str) -> None:
        """Verify all created tables have at least one record."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "integrity.cc"
            test_file.write_text("""
int test() { return 42; }
""")
            
            compiler.compile(str(test_file))
            
            # Verify all tables have data
            success, empty_tables = verify_all_tables_have_records(db)
            
            if not success:
                pytest.fail(f"Empty tables found: {empty_tables}")
    
    def test_id_field_exists(self, postgres_url: str, plugin_path: str) -> None:
        """Verify all tables have id field."""
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


class TestReferenceResolution:
    """Tests for reference resolution in extracted data."""
    
    def test_return_type_reference(self, postgres_url: str, plugin_path: str) -> None:
        """Verify return type references are valid."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "return_type.cc"
            test_file.write_text("""
int test() { return 0; }
double pi() { return 3.14; }
""")
            
            compiler.compile(str(test_file))
            
            # Check that FunctionDecl has returnType entries
            result = db.execute("""
                SELECT COUNT(*) 
                FROM FunctionDecl 
                WHERE returnType IS NOT NULL;
            """)
            
            count = int(result.strip()) if result.strip() else 0
            assert count > 0, "No FunctionDecl records with returnType"
