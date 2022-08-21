#!/bin/bash

# Build for KaiOS 2.5
function run () {
    echo "> $@"
    $@
}

# Convert WASM to JS
for w in $TRUNK_STAGING_DIR/*_bg.wasm
do
    run wasm2js $w -o "$w.js"
    # TODO is "debug" a valid env for rollup?
    run rollup --environment $TRUNK_PROFILE -i "$w.js" -o "$w.js"
    wrapper="${w%_bg.wasm}.js"
    run rollup --environment $TRUNK_PROFILE -i "$wrapper" -o "${wrapper%.js}.wrapper.js"
done

# Still have to do this,but should be shim script and scripts above added as nomodule scripts in the source html file
