#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

export RUST_LOG="debug"
export YEW_FULLSTACK_STATIC="$(cd $WORKDIR/../frontend/dist; pwd -P)"
cargo watch -x "run"
