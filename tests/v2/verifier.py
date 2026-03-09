#!/usr/bin/env python3
"""Verification functions for Arboretum test results."""

import json
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
from typing import cast as typing_cast

from .postgres import TestDatabase


class ResultVerifier:
    """Verifies test results against expected values."""
    
    def __init__(self, expected_dir: Optional[Path] = None):
        """Initialize the verifier.
        
        Args:
            expected_dir: Directory containing expected results (optional)
        """
        self.expected_dir = expected_dir or Path(__file__).parent.parent / "expected"
    
    def verify_table_exists(self, db: TestDatabase, table_name: str) -> bool:
        """Verify a table exists in the database.
        
        Args:
            db: TestDatabase instance
            table_name: Name of the table to verify
            
        Returns:
            True if table exists
        """
        tables = db.get_tables()
        exists = table_name in tables
        if not exists:
            print(f"Table '{table_name}' not found in database")
            print(f"Available tables: {', '.join(sorted(tables))}")
        return exists
    
    def verify_record_count(
        self,
        db: TestDatabase,
        table_name: str,
        expected_count: int,
        tolerance: int = 0,
    ) -> Tuple[bool, int, int]:
        """Verify record count matches expectation.
        
        Args:
            db: TestDatabase instance
            table_name: Name of the table
            expected_count: Expected number of records
            tolerance: Allowed deviation from expected count
            
        Returns:
            Tuple of (success, actual_count, expected_count)
        """
        actual_count = db.get_record_count(table_name)
        success = abs(actual_count - expected_count) <= tolerance
        
        if not success:
            print(f"Table '{table_name}': expected {expected_count}, got {actual_count}")
        
        return success, actual_count, expected_count
    
    def verify_column_exists(
        self,
        db: TestDatabase,
        table_name: str,
        column_name: str,
    ) -> bool:
        """Verify a column exists in a table.
        
        Args:
            db: TestDatabase instance
            table_name: Name of the table
            column_name: Name of the column
            
        Returns:
            True if column exists
        """
        columns = db.get_table_columns(table_name)
        exists = column_name in columns
        if not exists:
            print(f"Column '{column_name}' not found in table '{table_name}'")
            print(f"Available columns: {', '.join(sorted(columns))}")
        return exists
    
    def verify_expected_data(
        self,
        db: TestDatabase,
        expected_data: Dict[str, int],
    ) -> Tuple[bool, Dict[str, Tuple[int, int]]]:
        """Verify actual data matches expected counts.
        
        Args:
            db: TestDatabase instance
            expected_data: Dict of table_name -> expected_count
            
        Returns:
            Tuple of (success, details) where details shows actual vs expected
        """
        details = {}
        all_match = True
        
        for table_name, expected_count in expected_data.items():
            actual_count = db.get_record_count(table_name)
            details[table_name] = (actual_count, expected_count)
            if actual_count != expected_count:
                all_match = False
        
        return all_match, details
    
    def verify_table_schema(
        self,
        db: TestDatabase,
        expected_schema: Dict[str, List[str]],
    ) -> Tuple[bool, Dict[str, List[str]]]:
        """Verify table schema matches expected columns.
        
        Args:
            db: TestDatabase instance
            expected_schema: Dict of table_name -> list of column names
            
        Returns:
            Tuple of (success, missing_columns)
        """
        missing = {}
        all_match = True
        
        for table_name, expected_columns in expected_schema.items():
            actual_columns = db.get_table_columns(table_name)
            missing_columns = set(expected_columns) - set(actual_columns)
            
            if missing_columns:
                missing[table_name] = list(missing_columns)
                all_match = False
        
        return all_match, missing
    
    def verify_foreign_key_resolution(
        self,
        db: TestDatabase,
        table_name: str,
        foreign_key_column: str,
        referenced_table: str,
    ) -> Tuple[bool, int]:
        """Verify foreign key references resolve to valid IDs.
        
        Args:
            db: TestDatabase instance
            table_name: Name of the table with foreign key
            foreign_key_column: Name of the foreign key column
            referenced_table: Name of the referenced table
            
        Returns:
            Tuple of (success, invalid_count)
        """
        # Get count of records where foreign key doesn't exist in referenced table
        query = f"""
            SELECT COUNT(*)
            FROM {table_name} t1
            LEFT JOIN {referenced_table} t2
                ON t1.{foreign_key_column} = t2.id
            WHERE t2.id IS NULL
              AND t1.{foreign_key_column} IS NOT NULL;
        """
        
        result = db.execute(query)
        invalid_count = int(result)
        success = invalid_count == 0
        
        if not success:
            print(f"Found {invalid_count} invalid {foreign_key_column} references in {table_name}")
        
        return success, invalid_count


def load_expected_counts(filepath: str) -> Dict[str, int]:
    """Load expected record counts from JSON file.
    
    Args:
        filepath: Path to expected_counts.json
        
    Returns:
        Dict of table_name -> expected_count
    """
    path = Path(filepath)
    if not path.exists():
        return {}
    
    with open(path) as f:
        data = json.load(f)
    
    # Flatten nested structure if present
    result = {}
    for category, tables in data.items():
        if isinstance(tables, dict):
            result.update(tables)
        elif isinstance(tables, int):
            result[category] = tables
    
    return result


def load_expected_schema(filepath: str) -> Dict[str, List[str]]:
    """Load expected table schemas from SQL file.
    
    Args:
        filepath: Path to expected schema file
        
    Returns:
        Dict of table_name -> list of column names
    """
    path = Path(filepath)
    if not path.exists():
        return {}
    
    tables = {}
    current_table = None
    current_columns = []
    
    with open(path) as f:
        for line in f:
            line = line.strip()
            
            # Start of table definition
            if line.startswith("CREATE TABLE"):
                if current_table:
                    tables[current_table] = current_columns
                # Extract table name
                table_part = line.split("CREATE TABLE ")[1]
                current_table = table_part.split("(")[0].strip()
                current_columns = []
            
            # Column definition
            elif line and line != ");" and current_table:
                # Extract column name (first word before space)
                parts = line.split()
                if parts and not parts[0].startswith("--"):
                    col_name = parts[0].rstrip(",")
                    # Skip constraints (PRIMARY KEY, FOREIGN KEY, etc.)
                    if not col_name.upper().startswith(("PRIMARY", "FOREIGN", "CHECK", "UNIQUE")):
                        current_columns.append(col_name)
        
        # Don't forget last table
        if current_table:
            tables[current_table] = current_columns
    
    return tables


def verify_all_tables_have_records(db: TestDatabase) -> Tuple[bool, List[str]]:
    """Verify all created tables have at least one record.
    
    Args:
        db: TestDatabase instance
        
    Returns:
        Tuple of (success, empty_tables)
    """
    tables = db.get_tables()
    empty_tables = []
    
    for table in tables:
        count = db.get_record_count(table)
        if count == 0:
            empty_tables.append(table)
    
    success = len(empty_tables) == 0
    return success, empty_tables
