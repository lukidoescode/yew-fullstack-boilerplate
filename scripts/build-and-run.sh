#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

source "$WORKDIR/scripts/build.sh"
source "$WORKDIR/scripts/run.sh"
