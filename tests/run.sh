#!/bin/bash

set -e
set -x

CC_ARGS=(-c -fplugin="../build/libarboretum.so" -std=c++20)

../llvm/bin/clang++ ${CC_ARGS[@]} 0/a.cc

rm *.o
