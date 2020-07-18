#!/bin/bash

cd $(dirname $0)

mod="$1"

echo -n "$1 => " 1>&2
res=$(curl -s -X POST "https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=REDACTED" -H "accept: */*" -H "Content-Type: text/plain" -d "$mod")
echo ${res}
