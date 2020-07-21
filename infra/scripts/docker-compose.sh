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


readonly DOWNLOAD_URL="https://github.com/docker/compose/releases/download/1.26.2/docker-compose-$(uname -s)-$(uname -m)"
readonly CACHE_PATH="$HOME/.cache/icfpc2020/docker-compose"

if [[ ! -f "$CACHE_PATH" ]]; then
    echo "docker-compose not found. Downloading..." >&2
    mkdir -p "$(dirname "$CACHE_PATH")"
    wget -O "$CACHE_PATH" "$DOWNLOAD_URL" || exit $?
    chmod +x "$CACHE_PATH"
fi

exec "$CACHE_PATH" "$@"
