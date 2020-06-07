#!/usr/bin/env bash

WORKDIR="$( cd "$(dirname "$0")"/.. ; pwd -P )" # get the project dir

rm -rf "$WORKDIR/frontend/dist"
rm -rf "$WORKDIR/frontend/node_modules"
rm -rf "$WORKDIR/frontend/pkg"

if [[ "$(docker images -q yew-fullstack/node-wasm-pack 2> /dev/null)" == "" ]]; then
  docker build -t yew-fullstack/node-wasm-pack -t yew-fullstack/node-wasm-pack:latest "$WORKDIR/frontend"
fi

docker run --name build_yew-fullstack-frontend \
    -v "$WORKDIR/frontend:/root/src" \
    --rm -i -t -w=/root/src yew-fullstack/node-wasm-pack \
    yarn install

docker run --name build_yew-fullstack-frontend \
    -v "$WORKDIR/frontend:/root/src" \
    --rm -i -t -w=/root/src yew-fullstack/node-wasm-pack \
    yarn run build

docker build -t yew-fullstack/application -t yew-fullstack/application:latest "$WORKDIR"

# cd "$PREVIOUS_PWD" # go back to where the user was before
