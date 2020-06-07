#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

cd "$WORKDIR/scripts"
docker-compose \
    -f "$WORKDIR/scripts/docker-compose.yml" \
    -p yew-fullstack \
    up \
    --force-recreate --remove-orphans \
    --exit-code-from application \
    --ignore-pull-failures
