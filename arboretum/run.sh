#!/bin/bash

rm -rf /tmp/arboretum/

RUST_LOG=arboretum-graph=trace,arboretum=trace cargo run -- --db-dir /tmp/arboretum/
