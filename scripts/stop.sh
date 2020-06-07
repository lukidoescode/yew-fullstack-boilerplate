#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

docker-compose \
    -f "$WORKDIR/scripts/docker-compose.yml" \
    down -v --rmi all --remove-orphans

cd "$WORKDIR/scripts"
docker-compose \
    -f "$WORKDIR/scripts/docker-compose.yml" \
    -p yew-fullstack kill

docker-compose \
    -f "$WORKDIR/scripts/docker-compose.yml" \
    -p yew-fullstack rm -f
