#!/bin/bash

# Build for KaiOS 2.5

function run () {
    echo "> $@"
    exec $@
}

# Convert WASM to JS
for w in $TRUNK_STAGING_DIR/*_bg.wasm
do
    run wasm2js $w -o "$w.js"
done

# Bundle JavaScript using Rollup
# rollup $TRUNK_STAGING_DIR/<wasm-js-file> and $TRUNK_STAGING_DIR/<wrapper-js-file>

# Still have to do this,but should be shim script and scripts above added as nomodule scripts in the source html file
