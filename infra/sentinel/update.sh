#!/bin/bash

cd "$(dirname "$0")"

./scripts/docker-compose.sh up --build --detach --remove-orphans
