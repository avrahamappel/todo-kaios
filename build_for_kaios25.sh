#!/bin/bash

# Build for KaiOS 2.5

set -e

function run () {
    echo "> $*"
    # shellcheck disable=SC2068
    $@
}

WASM_JS_FILE=""
WRAPPER_JS_FILE=""

# Convert WASM to JS
for w in "$TRUNK_STAGING_DIR"/*_bg.wasm
do
    WASM_JS_FILE="$w.js"

    echo "Compiling WASM to JS for older platforms..."
    run wasm2js --emscripten "$w" -o "$WASM_JS_FILE"

    echo "Bundling es5 code..."
    wrapper="${w%_bg.wasm}.js"
    WRAPPER_JS_FILE="${w%_bg.wasm}.wrapper.js" 
    run npx babel "$wrapper" -o "$WRAPPER_JS_FILE"
    # Remove some incompatible code
    run sed -i 's/,\simport.meta.url//' "$WRAPPER_JS_FILE" 
    run sed -i 's/wasm.__wbindgen_start();//' "$WRAPPER_JS_FILE" 
done

echo "Injecting es5 scripts..."

# Define scripts
# shellcheck disable=SC2295
wasm_file_url="${WASM_JS_FILE#$TRUNK_STAGING_DIR}"
# shellcheck disable=SC2295
wrapper_file_url="${WRAPPER_JS_FILE#$TRUNK_STAGING_DIR}"
scripts=$(cat <<HTML
<script nomodule>var exports = {};</script>
<script nomodule src="/wasm-shim.js"></script>
<script nomodule src="$wasm_file_url"></script>
<script nomodule src="$wrapper_file_url"></script>
<script nomodule>
var init = exports.default;
init(null).then(function (wasm) {
    // app = thing;
    console.log("Starting!");
    wasm.main();
});
</script>
HTML
)

# Get file parts
html_file="$TRUNK_STAGING_DIR"/index.html
html_head=$(head -n -1 "$html_file")
html_tail=$(tail -n 1 "$html_file")

# Inject scripts
{ echo "$html_head"; echo "$scripts"; echo "$html_tail"; } > "$html_file"
echo "Done"
