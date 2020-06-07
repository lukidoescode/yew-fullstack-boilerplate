#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

cd "$WORKDIR/frontend"

rm -rf node_modules

yarn install

cd "$WORKDIR/scripts"

# Only if building got messed up
docker-compose \
    -f "$WORKDIR/scripts/docker-compose.dev.yml" \
    -p yew-fullstack \
    up \
    --force-recreate --remove-orphans --build \
    --exit-code-from backend
