#!/bin/bash -e

cd "$(dirname "$0")"

rm -rf dist
npm run build -- --mode production
cp -v dist/{index.html,web.js} ../../docs/
