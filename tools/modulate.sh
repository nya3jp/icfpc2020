#!/bin/bash

#
# Example
#  echo "110110000111011111100001001111110100110000" | ./modulate.sh  =>  (1, (81740, nil))

cd $(dirname $0)

cd ../oka

cargo run --release -- "$@"
