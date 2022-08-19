function WebAssembly() {
    // this.foo = foo;
}

WebAssembly.instantiate = function (bytes, imports) {
    //console.log(bytes);
    //console.log(imports.wbg);
    Object.assign(imports.wbg, {
        abort: function () {
            console.trace();
            throw new Error("ABORT");
        },
    });
    var wasm = instantiate(imports.wbg);
    //console.log(res);
    return Promise.resolve({
        instance: { exports: wasm },
        // module: "module",
    });
};

// var app;
wasm_bindgen(null).then(function (wasm) {
    // app = thing;
    console.log("Starting!");
    wasm.main();
});
