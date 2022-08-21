#!/bin/bash

# Build for KaiOS 2.5

function run () {
    echo "> $*"
    $@
}

# Convert WASM to JS
for w in "$TRUNK_STAGING_DIR"/*_bg.wasm
do
    run wasm2js "$w" -o "$w.js"
    wrapper="${w%_bg.wasm}.js"
    # TODO we will probably need a config for this as CLI options don't seem to do the trick
    # TODO is "debug" a valid env for rollup?
    # TODO compile for kaios 2.5 browser which is actually the point here
    run rollup --environment "$TRUNK_PROFILE" -i "$w.js" -i "$wrapper" --entryFileNames "[name].es5.js"
done

# Still have to do this,but should be shim script and scripts above added as nomodule scripts in the source html file
# define my scripts
# get heade of file (prob until last line)
# inject my scripts
# add back last line
