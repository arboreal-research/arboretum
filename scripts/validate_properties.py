#!/usr/bin/env python3
"""Validate properties.csv format and content."""
import csv
import sys
import re

def validate_properties_csv(path: str) -> bool:
    errors = []
    
    # The properties.csv uses tabs as delimiters
    with open(path, 'r') as f:
        content = f.read()
        lines = content.strip().split('\n')
        
        # Parse header
        header_line = lines[0]
        expected_cols = ['Type', 'Predicate', 'Enabled']
        actual_cols = header_line.split('\t')
        
        missing_cols = [c for c in expected_cols if c not in actual_cols]
        if missing_cols:
            errors.append(f"Missing columns: {missing_cols}")
        
        # Parse data rows
        predicates = set()
        for i, line in enumerate(lines[1:], 2):  # Skip header, start at row 2
            # Skip empty lines
            if not line.strip():
                continue
            
            parts = line.split('\t')
            if len(parts) < 3:
                errors.append(f"Row {i}: Expected 3 columns, got {len(parts)}")
                continue
            
            type_val, predicate, enabled = parts[0], parts[1], parts[2]
            
            if not type_val.strip():
                errors.append(f"Row {i}: Missing Type")
            if not predicate.strip():
                errors.append(f"Row {i}: Missing Predicate")
            if enabled.strip() not in {'0', '1'}:
                errors.append(f"Row {i}: Invalid Enabled value: {enabled}")
            if predicate in predicates:
                errors.append(f"Row {i}: Duplicate Predicate: {predicate}")
            if predicate:
                predicates.add(predicate)
    
    if errors:
        for e in errors:
            print(f"ERROR: {e}")
        return False
    return True

if __name__ == '__main__':
    if validate_properties_csv('reificator/properties.csv'):
        print("✓ properties.csv is valid")
        sys.exit(0)
    else:
        sys.exit(1)
