#!/bin/bash
exec python3 ../../infra/interact/interact.py "$1" "$2" ./target/release/super_bot --log=INFO

cargo build --release &&  ICFPC_API_KEY=REDACTED python3 ../../infra/tester/tester.py battle ./target/release/super_bot ./target/release/super_bot |& tee log.txt
