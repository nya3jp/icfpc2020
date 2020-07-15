#!/bin/sh

bazel-bin/examples/app "$@" || echo "run error code: $?"
