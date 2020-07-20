#!/bin/bash

cd $(dirname $0)

set -e

( cd bot/oka; cargo build )
( cd bot/do_nothing; cargo build )
( cd bot/kimiyuki; cargo build )
( cd bot/psh/testbot; cargo build )
