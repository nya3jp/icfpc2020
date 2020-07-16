#!/bin/bash

readonly DOWNLOAD_URL="https://github.com/docker/compose/releases/download/1.26.2/docker-compose-$(uname -s)-$(uname -m)"
readonly CACHE_PATH="$HOME/.cache/icfpc2020/docker-compose"

if [[ ! -f "$CACHE_PATH" ]]; then
    echo "docker-compose not found. Downloading..." >&2
    mkdir -p "$(dirname "$CACHE_PATH")"
    wget -O "$CACHE_PATH" "$DOWNLOAD_URL" || exit $?
    chmod +x "$CACHE_PATH"
fi

exec "$CACHE_PATH" "$@"
