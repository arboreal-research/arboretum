#!/usr/bin/env python3
"""Tests for statement AST node handling."""

import pytest

from ..postgres import postgres_session
from ..compiler import TestCompiler


pytestmark = pytest.mark.statements


class TestIfStatements:
    """Tests for if statement extraction."""
    
    def test_simple_if(self, postgres_url: str, plugin_path: str) -> None:
        """Test simple if statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "if_simple.cc"
            test_file.write_text("""
void test(int x) {
    if (x > 0) {
        // positive
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("IfStmt")
            assert count == 1, f"Expected 1 IfStmt, got {count}"
    
    def test_if_else(self, postgres_url: str, plugin_path: str) -> None:
        """Test if-else statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "if_else.cc"
            test_file.write_text("""
void test(int x) {
    if (x > 0) {
        // positive
    } else {
        // non-positive
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("IfStmt")
            assert count == 1, f"Expected 1 IfStmt, got {count}"
    
    def test_if_elseif_else(self, postgres_url: str, plugin_path: str) -> None:
        """Test if-elseif-else statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "if_elseif.cc"
            test_file.write_text("""
void test(int x) {
    if (x > 0) {
        // positive
    } else if (x < 0) {
        // negative
    } else {
        // zero
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("IfStmt")
            assert count == 1, f"Expected 1 IfStmt, got {count}"


class TestLoopStatements:
    """Tests for loop statement extraction."""
    
    def test_for_loop(self, postgres_url: str, plugin_path: str) -> None:
        """Test for loop statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "for_loop.cc"
            test_file.write_text("""
void test() {
    for (int i = 0; i < 10; i++) {
        // loop body
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("ForStmt")
            assert count == 1, f"Expected 1 ForStmt, got {count}"
    
    def test_while_loop(self, postgres_url: str, plugin_path: str) -> None:
        """Test while loop statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "while_loop.cc"
            test_file.write_text("""
void test() {
    int i = 0;
    while (i < 10) {
        i++;
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("WhileStmt")
            assert count == 1, f"Expected 1 WhileStmt, got {count}"
    
    def test_do_while_loop(self, postgres_url: str, plugin_path: str) -> None:
        """Test do-while loop statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "do_while.cc"
            test_file.write_text("""
void test() {
    int i = 0;
    do {
        i++;
    } while (i < 10);
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("DoStmt")
            assert count == 1, f"Expected 1 DoStmt, got {count}"


class TestSwitchStatements:
    """Tests for switch statement extraction."""
    
    def test_switch_statement(self, postgres_url: str, plugin_path: str) -> None:
        """Test switch statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "switch.cc"
            test_file.write_text("""
void test(int x) {
    switch (x) {
        case 1:
            break;
        case 2:
            break;
        default:
            break;
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("SwitchStmt")
            assert count == 1, f"Expected 1 SwitchStmt, got {count}"
    
    def test_case_statements(self, postgres_url: str, plugin_path: str) -> None:
        """Test case statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "case.cc"
            test_file.write_text("""
void test(int x) {
    switch (x) {
        case 1:
            break;
        case 2:
            break;
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("CaseStmt")
            assert count >= 2, f"Expected at least 2 CaseStmt, got {count}"


class TestCompoundStatements:
    """Tests for compound statement extraction."""
    
    def test_compound_statement(self, postgres_url: str, plugin_path: str) -> None:
        """Test compound statement (block) extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "compound.cc"
            test_file.write_text("""
void test() {
    {
        int x = 1;
    }
    {
        int y = 2;
    }
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("CompoundStmt")
            assert count >= 2, f"Expected at least 2 CompoundStmt, got {count}"


class TestReturnStatements:
    """Tests for return statement extraction."""
    
    def test_return_statement(self, postgres_url: str, plugin_path: str) -> None:
        """Test return statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path=plugin_path, postgres_url=db.connection_url)
            
            test_file = db.build_dir / "return.cc"
            test_file.write_text("""
int test() {
    return 42;
}

void void_test() {
    return;
}
""")
            
            compiler.compile(str(test_file))
            
            count = db.get_record_count("ReturnStmt")
            assert count >= 2, f"Expected at least 2 ReturnStmt, got {count}"
