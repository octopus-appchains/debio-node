#! /bin/bash

script_path=$(realpath $0)
dir_path=$(dirname $script_path)
node_storage="$dir_path/storage/node-4"

./target/release/debio-node --base-path $node_storage \
--chain ./chain-spec-local-raw.json \
--port 30336 \
--ws-port 9948 \
--rpc-port 9936 \
--validator \
--name node-4 

