#!/bin/bash

set -e
set -x

CC_ARGS=(-c -fplugin="../build/libarboretum.so" -std=c++20)

../llvm/bin/clang++ ${CC_ARGS[@]} 0/a.cc &
../llvm/bin/clang++ ${CC_ARGS[@]} 0/b.cc &
../llvm/bin/clang++ ${CC_ARGS[@]} 0/c.cc &
../llvm/bin/clang++ ${CC_ARGS[@]} 0/d.cc &
wait

rm *.o
