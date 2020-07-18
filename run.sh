#!/bin/sh

# MAIN_PROGRAM=bazel-bin/examples/app
MAIN_PROGRAM=draftcode/api_test/target/release/api_test

exec python3 ./infra/interact/interact.py "$1" "$2" "$MAIN_PROGRAM" --log=INFO
