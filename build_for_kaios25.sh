#!/bin/bash

# Build for KaiOS 2.5

# Convert WASM to JS
# wasm2js $TRUNK_STAGING_DIR/<file> 

# Bundle JavaScript using Rollup
# rollup $TRUNK_STAGING_DIR/<wasm-js-file> and $TRUNK_STAGING_DIR/<wrapper-js-file>

# Still have to do this,but should be shim script and scripts above added as nomodule scripts in the source html file
