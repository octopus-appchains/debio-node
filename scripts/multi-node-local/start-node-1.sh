#! /bin/bash

script_path=$(realpath $0)
dir_path=$(dirname $script_path)
node_storage_1="$dir_path/storage/node-1"

./target/release/debio-node --base-path $node_storage_1 \
--chain ./chain-spec-local-raw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--validator \
--name node-1

