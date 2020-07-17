#!/bin/bash
cd $(dirname "${BASH_SOURCE[0]}")
cd ../../

protoc --proto_path draftcode/protos --go_out=plugins=grpc:draftcode/protos --go_opt=paths=source_relative draftcode/protos/infra.proto
