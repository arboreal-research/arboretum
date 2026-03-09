#!/usr/bin/env python3
"""PostgreSQL database management for tests."""

import subprocess
import tempfile
import os
from contextlib import contextmanager
from typing import Optional, List, Tuple
from pathlib import Path


class TestDatabase:
    """Manages a PostgreSQL database for testing with isolation."""
    
    def __init__(self, postgres_url: str):
        """Initialize with the base PostgreSQL URL.
        
        Args:
            postgres_url: Base connection URL (e.g., postgres://localhost:5432)
        """
        self.postgres_url = postgres_url
        # Generate unique database name using PID and timestamp
        self.db_name = f"arboretum_test_{os.getpid()}"
        # Extract base URL without database name
        self.base_url = postgres_url.rstrip("/")
        if "/" in self.base_url:
            self.base_url = self.base_url.rsplit("/", 1)[0]
        self.connection_url = f"{self.base_url}/{self.db_name}"
    
    def create(self) -> None:
        """Create the test database."""
        print(f"Creating database: {self.db_name}")
        result = subprocess.run(
            ["psql", "-d", "postgres", "-c", f"CREATE DATABASE {self.db_name}"],
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            raise RuntimeError(
                f"Failed to create database {self.db_name}: {result.stderr}"
            )
    
    def drop(self) -> None:
        """Drop the test database."""
        print(f"Dropping database: {self.db_name}")
        result = subprocess.run(
            ["psql", "-d", "postgres", "-c", f"DROP DATABASE IF EXISTS {self.db_name}"],
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            # Don't raise - database might already be dropped
            print(f"Warning: Could not drop database {self.db_name}: {result.stderr}")
    
    def execute(self, sql: str) -> str:
        """Execute SQL and return results as string.
        
        Args:
            sql: SQL query to execute
            
        Returns:
            Query results as string
        """
        env = os.environ.copy()
        env["PGDATABASE"] = self.db_name
        
        # Set other connection params from base URL
        if self.postgres_url.startswith("postgres://"):
            parts = self.postgres_url[11:].split("@")
            if len(parts) == 2:
                # Parse user:password@host:port
                auth, host_port = parts
                if "/" in host_port:
                    host, port = host_port.split("/")
                else:
                    host = host_port
                    port = "5432"
                env["PGHOST"] = host
                env["PGPORT"] = port
                if ":" in auth:
                    env["PGUSER"], env["PGPASSWORD"] = auth.split(":", 1)
                else:
                    env["PGUSER"] = auth
        
        result = subprocess.run(
            ["psql", "-t", "-A", "-c", sql],
            capture_output=True,
            text=True,
            env=env
        )
        
        if result.returncode != 0:
            raise RuntimeError(f"SQL execution failed: {result.stderr}")
        
        return result.stdout.strip()
    
    def execute_file(self, sql_file: Path) -> None:
        """Execute SQL from a file.
        
        Args:
            sql_file: Path to SQL file
        """
        with open(sql_file) as f:
            sql = f.read()
        self.execute(sql)
    
    def get_tables(self) -> List[str]:
        """Get list of tables in the database.
        
        Returns:
            List of table names
        """
        result = self.execute("""
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public'
            ORDER BY table_name;
        """)
        return [t.strip() for t in result.split("\n") if t.strip()]
    
    def get_table_columns(self, table_name: str) -> List[str]:
        """Get column names for a table.
        
        Args:
            table_name: Name of the table
            
        Returns:
            List of column names
        """
        result = self.execute(f"""
            SELECT column_name 
            FROM information_schema.columns 
            WHERE table_name = '{table_name}'
            ORDER BY ordinal_position;
        """)
        return [c.strip() for c in result.split("\n") if c.strip()]
    
    def get_record_count(self, table_name: str) -> int:
        """Get record count for a table.
        
        Args:
            table_name: Name of the table
            
        Returns:
            Number of records in the table
        """
        result = self.execute(f"SELECT COUNT(*) FROM {table_name};")
        try:
            return int(result)
        except ValueError:
            return 0
    
    def query_one(self, sql: str) -> Optional[str]:
        """Execute query and return first result.
        
        Args:
            sql: SQL query
            
        Returns:
            First result or None if no results
        """
        result = self.execute(sql)
        return result if result else None


@contextmanager
def postgres_session(postgres_url: str, cleanup: bool = True):
    """Context manager for test database lifecycle.
    
    Args:
        postgres_url: PostgreSQL connection URL
        cleanup: Whether to drop database after context
        
    Yields:
        TestDatabase instance
    """
    db = TestDatabase(postgres_url)
    try:
        db.create()
        yield db
    finally:
        if cleanup:
            db.drop()


def setup_postgres_for_test(postgres_url: str) -> TestDatabase:
    """Set up PostgreSQL connection for a single test.
    
    This is a helper function for test setup that creates a database
    without using context manager (useful for pytest fixtures).
    
    Args:
        postgres_url: PostgreSQL connection URL
        
    Returns:
        TestDatabase instance (caller responsible for cleanup)
    """
    return TestDatabase(postgres_url)


def verify_tables_exist(db: TestDatabase, expected_tables: List[str]) -> Tuple[bool, List[str]]:
    """Verify that expected tables exist in the database.
    
    Args:
        db: TestDatabase instance
        expected_tables: List of expected table names
        
    Returns:
        Tuple of (success, missing_tables)
    """
    actual_tables = set(db.get_tables())
    expected_set = set(expected_tables)
    missing = expected_set - actual_tables
    return len(missing) == 0, list(missing)


def verify_record_counts(
    db: TestDatabase,
    expected_counts: dict[str, int]
) -> Tuple[bool, dict[str, Tuple[int, int]]]:
    """Verify record counts match expectations.
    
    Args:
        db: TestDatabase instance
        expected_counts: Dict of table_name -> expected_count
        
    Returns:
        Tuple of (success, details) where details is dict of
        table_name -> (actual, expected)
    """
    details = {}
    all_match = True
    
    for table_name, expected_count in expected_counts.items():
        actual_count = db.get_record_count(table_name)
        details[table_name] = (actual_count, expected_count)
        if actual_count != expected_count:
            all_match = False
    
    return all_match, details
