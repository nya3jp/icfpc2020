#!/bin/bash

cargo build && python ../../infra/tester/tester.py tutorial 1 ./target/debug/oka
