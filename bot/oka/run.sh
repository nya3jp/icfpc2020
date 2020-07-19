#!/bin/bash

export RUST_BACKTRACE=1
cargo build && python3 ../../infra/tester/tester.py tutorial 2 ./target/debug/oka
