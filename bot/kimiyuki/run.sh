#!/bin/bash
find .
exec python3 ../../infra/interact/interact.py "$1" "$2" ./target/release/kimiyuki --log=INFO
