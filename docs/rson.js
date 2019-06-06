var wasm;

let WASM_VECTOR_LEN = 0;

let cachegetNodeBufferMemory = null;
function getNodeBufferMemory() {
    if (cachegetNodeBufferMemory === null || cachegetNodeBufferMemory.buffer !== wasm.memory.buffer) {
        cachegetNodeBufferMemory = Buffer.from(wasm.memory.buffer);
    }
    return cachegetNodeBufferMemory;
}

function passStringToWasm(arg) {

    const size = Buffer.byteLength(arg);
    const ptr = wasm.__wbindgen_malloc(size);
    getNodeBufferMemory().write(arg, ptr, size);
    WASM_VECTOR_LEN = size;
    return ptr;
}
/**
* @param {string} s
* @returns {void}
*/
module.exports.run = function(s) {
    const ptr0 = passStringToWasm(s);
    const len0 = WASM_VECTOR_LEN;
    return wasm.run(ptr0, len0);
};

wasm = require('./rson_bg');

