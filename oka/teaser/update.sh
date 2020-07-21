#!/bin/bash
# Copyright 2020 Google LLC
# Copyright 2020 Team Spacecat
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


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
