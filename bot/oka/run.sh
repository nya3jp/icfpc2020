#!/bin/bash

export RUST_BACKTRACE=1
cargo build && python ../../infra/tester/tester.py tutorial 1 ./target/debug/oka
