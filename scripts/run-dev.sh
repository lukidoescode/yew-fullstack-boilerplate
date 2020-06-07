#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

cd "$WORKDIR/frontend"

yarn install

cd "$WORKDIR/scripts"

docker-compose \
    -f "$WORKDIR/scripts/docker-compose.dev.yml" \
    -p yew-fullstack-dev \
    up \
    --force-recreate --remove-orphans \
    --exit-code-from backend
