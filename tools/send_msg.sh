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

mod="$(echo "$1" | ./modulate.sh || exit 1)"

echo -n "$1 => " 1>&2
curl -s -X POST "https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=REDACTED" -H "accept: */*" -H "Content-Type: text/plain" -d "$mod" | ./demodulate.sh
