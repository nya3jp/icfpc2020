#!/bin/bash

cd $(dirname $0)

mod="$(echo "$1" | ./modulate.sh || exit 1)"

echo -n "$1 => " 1>&2
curl -s -X POST "https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=REDACTED" -H "accept: */*" -H "Content-Type: text/plain" -d "$mod"
