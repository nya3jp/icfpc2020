#!/bin/bash
exec python3 ../../../infra/interact/interact.py "$1" "$2" ./target/release/testbot --log=INFO
