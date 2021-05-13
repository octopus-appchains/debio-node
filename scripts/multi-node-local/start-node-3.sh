#! /bin/bash

script_path=$(realpath $0)
dir_path=$(dirname $script_path)
node_storage="$dir_path/storage/node-3"

./target/release/debio-node --base-path $node_storage \
--chain ./chain-spec-local-raw.json \
--port 30335 \
--ws-port 9947 \
--rpc-port 9935 \
--validator \
--name node-3 

