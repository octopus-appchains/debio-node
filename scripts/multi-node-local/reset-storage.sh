#!/bin/bash

script_path=$(realpath $0)
dir_path=$(dirname $script_path)

node_storage_1="$dir_path/../../storage/node-1/*"
node_storage_2="$dir_path/../../storage/node-2/*"
node_storage_3="$dir_path/../../storage/node-3/*"
node_storage_4="$dir_path/../../storage/node-4/*"

rm -rf $node_storage_1
rm -rf $node_storage_2
rm -rf $node_storage_3
rm -rf $node_storage_4
