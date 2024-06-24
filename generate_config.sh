#!/bin/sh

echo "executablepath: $(realpath $(dirname $0))/target/release/rplugin" > "rplugin_config.yaml"
