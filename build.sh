#!/bin/sh

bazel --batch build --distdir=/bazel/dist -c opt //examples:app
