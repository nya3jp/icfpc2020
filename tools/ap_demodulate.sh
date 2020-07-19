#!/bin/bash

#
# Example
#  echo "110110000111011111100001001111110100110000" | ./demodulate.sh  # =>  (1, (81740, nil))

cd $(dirname $0)

cd ../oka

cargo run -q --release -- --ap -d "$@"
