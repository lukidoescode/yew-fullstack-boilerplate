#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

#export RUST_LOG="actix_web=debug"
export RUST_LOG="debug"
cargo watch -x "run --features \"forward-frontend\""
