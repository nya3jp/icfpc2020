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

# go run main.go ../../../messages/galaxy.txt "ap ap galaxy   ap ap cons $1         ap ap cons      ap ap cons $2 nil           ap ap cons 0     ap ap cons nil nil                   ap ap cons $3 $4" | gosh
go run main.go ../../../messages/galaxy.txt \
  "ap ap galaxy   ap ap cons $1         ap ap cons    ap ap cons $2   ap ap cons $3 nil           ap ap cons 0     ap ap cons nil nil                   ap ap cons $4 $5" | gosh
