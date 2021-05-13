#! /bin/bash

script_path=$(realpath $0)
dir_path=$(dirname $script_path)
node_storage="$dir_path/storage/node-2"

./target/release/debio-node --base-path $node_storage \
--chain ./chain-spec-local-raw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--validator \
--name node-2

