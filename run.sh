#!/bin/sh

MAIN_PROGRAM=bazel-bin/examples/app

exec python3 ./infra/interact/interact.py "$1" "$2" "$MAIN_PROGRAM" --log=INFO
