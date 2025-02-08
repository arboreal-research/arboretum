#!/bin/bash

rm -rf /tmp/arboretum/
mkdir /tmp/arboretum/

RUST_BACKTRACE=1 cargo run -- --db-dir /tmp/arboretum/
