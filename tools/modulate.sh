#!/bin/bash

#
# Example
#  echo "(1, (81740, nil))" | ./modulate.sh  # =>  "110110000111011111100001001111110100110000"

cd $(dirname $0)

cd ../oka

cargo run --release -- "$@"
