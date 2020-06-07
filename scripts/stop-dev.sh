#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

docker-compose \
    -f "$WORKDIR/scripts/docker-compose.dev.yml" \
    down -v --rmi all --remove-orphans

cd "$WORKDIR/scripts"
docker-compose \
    -f "$WORKDIR/scripts/docker-compose.dev.yml" \
    -p yew-fullstack-dev kill

docker-compose \
    -f "$WORKDIR/scripts/docker-compose.dev.yml" \
    -p yew-fullstack-dev rm -f
