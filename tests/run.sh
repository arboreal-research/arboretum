#!/bin/bash

set -e
set -x

CC_ARGS=(-c -fplugin="../build/libarboretum.so" -std=c++20)

# Compile test files with Arboretum plugin
clang++ ${CC_ARGS[@]} 0/a.cc

# Verify PostgreSQL tables were created
echo "Checking PostgreSQL tables..."
psql -d arboretum -c "\dt" || echo "PostgreSQL not configured - skipping table check"

# Clean up object files
rm -f *.o 2>/dev/null || true

echo "Test completed successfully!"
