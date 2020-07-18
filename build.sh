#!/bin/sh

# bazel --batch build --distdir=/bazel/dist -c opt //examples:app

cd ./draftcode/api_test/
cargo build --release --offline

