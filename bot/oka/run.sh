#!/bin/bash

export RUST_BACKTRACE=1
cd "$(dirname $0)"
cargo build && python3 ../../infra/tester/tester.py tutorial 2 ./target/debug/oka
