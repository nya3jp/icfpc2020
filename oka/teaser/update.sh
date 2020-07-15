#!/bin/bash

cd $(dirname $0)

cd ..

d=teaser/pflockingen
n=$(ls ../$d/*.png | wc -l)
for i in $(seq 1 $n); do
    RUST_BACKTRACE=1 cargo run -- annotate -f "../$d/image$(printf '%02d' $i).png" > $d/annotated"$(printf '%02d' $i)".svg || exit 1;
done

d=teaser
n=$(ls ../$d/*.png | wc -l)
for i in $(seq 1 $n); do
    RUST_BACKTRACE=1 cargo run -- annotate "../$d/message$i.png" > $d/annotated$i.svg || exit 1;
done
