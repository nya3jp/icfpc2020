#!/bin/bash
# go run main.go ../../../messages/galaxy.txt "ap ap galaxy   ap ap cons $1         ap ap cons      ap ap cons $2 nil           ap ap cons 0     ap ap cons nil nil                   ap ap cons $3 $4" | gosh
go run main.go ../../../messages/galaxy.txt \
  "ap ap galaxy   ap ap cons $1         ap ap cons    ap ap cons $2   ap ap cons $3 nil           ap ap cons 0     ap ap cons nil nil                   ap ap cons $4 $5" | gosh
