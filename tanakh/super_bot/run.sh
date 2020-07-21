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

exec python3 ../../infra/interact/interact.py "$1" "$2" ./target/release/super_bot --log=INFO

cargo build --release &&  ICFPC_API_KEY=REDACTED python3 ../../infra/tester/tester.py battle ./target/release/super_bot ./target/release/super_bot |& tee log.txt
