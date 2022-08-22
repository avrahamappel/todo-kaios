function WebAssembly() {}

WebAssembly.Module = function (bytes) {
    this.bytes = bytes;
};

WebAssembly.Instance = function (module, imports) {
    this.module = module;
    this.imports = imports;
};

WebAssembly.instantiate = function (bytes, imports) {
    Object.assign(imports.wbg, {
        abort: function () {
            console.trace();
            throw new Error("ABORT");
        },
    });
    // This comes from wasm.js
    var wasm = instantiate(imports.wbg);
    return Promise.resolve({
        instance: { exports: wasm },
    });
};
