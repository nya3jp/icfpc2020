#!/bin/bash

cd $(dirname $0)

cd ..

# d=messages/pflockingen
# n=$(ls ../$d/*.png | wc -l)
# for i in $(seq 1 $n); do
#     RUST_BACKTRACE=1 cargo run -- annotate -f "../$d/image$(printf '%02d' $i).png" > $d/annotated"$(printf '%02d' $i)".svg || exit 1;
# done

d=messages
n=$(ls ../$d/*.png | wc -l)
for i in $(seq 1 $n); do
    RUST_BACKTRACE=1 cargo run -- annotate "../messages/message$i.png" > teaser/annotated$i.svg || exit 1;
done
