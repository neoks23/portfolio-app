import { __cargo_web_snippet_53b6a8dccd8cdae12266b154acb29e83ba989dad } from './snippets/portfolio-app-dbd4cee37ddc1428/inline0.js';
import { __cargo_web_snippet_9983a55415fa9f55a62367a10f747551caff8c85 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline1.js';
import { __cargo_web_snippet_83015c1e57aa6b41f7f5ad37029c001ad4599ef4 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline10.js';
import { __cargo_web_snippet_dde41c57e3e12fe6715329b1be3ecb5968afdf88 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline11.js';
import { __cargo_web_snippet_1f4505a9a52a1adc65586876e0ebf88890ba0399 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline12.js';
import { __cargo_web_snippet_31d3816c58be7e6e6a0ea5e7555420abd38a3541 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline13.js';
import { __cargo_web_snippet_9f5b745f08d25fd3be22a5f88a2b759ee41a6428 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline14.js';
import { __cargo_web_snippet_0cdad3c948ce131f15bd32bb5d601ef1dc2ffa7f } from './snippets/portfolio-app-dbd4cee37ddc1428/inline15.js';
import { __cargo_web_snippet_f50cc248f06b326fdd4fb11b14df72deea771be8 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline16.js';
import { __cargo_web_snippet_a625bad725ad48c06e48e58d8809bb9ac3c2c2f8 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline17.js';
import { __cargo_web_snippet_b9759d3fa5ae6d67b8f5e700438e922317c8f818 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline18.js';
import { __cargo_web_snippet_d67ac6baeef6c51abf83b47906cb83107d4f2ee1 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline19.js';
import { __cargo_web_snippet_8d3e19a47a694acf0e6fa6cf704bb3bb434f34ef } from './snippets/portfolio-app-dbd4cee37ddc1428/inline2.js';
import { __cargo_web_snippet_41d85b986b58a974640d51bf2fc956807ef1dd53 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline20.js';
import { __cargo_web_snippet_62ba98747f9c2ba20b31b35a4cb3e85e7febd1b6 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline21.js';
import { __cargo_web_snippet_36a43b0918c86173e2053ff2c4da48c9fe2017b1 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline3.js';
import { __cargo_web_snippet_1d08625da0822b42b533903f0c034c4ed935e5ec } from './snippets/portfolio-app-dbd4cee37ddc1428/inline4.js';
import { __cargo_web_snippet_9f1bee6f5186fd188b51e8a9eee6bb3815587706 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline5.js';
import { __cargo_web_snippet_e853589303cd697aa56165d991517b3a3c0b5cf7 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline6.js';
import { __cargo_web_snippet_e873d7259824ee2415e78d10e1bfb37f0116a486 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline7.js';
import { __cargo_web_snippet_a230002551a5ee6325a80144144e81d4886ac6ac } from './snippets/portfolio-app-dbd4cee37ddc1428/inline8.js';
import { __cargo_web_snippet_2e2f59c6ab7049089c8856a5dfac2853485187b5 } from './snippets/portfolio-app-dbd4cee37ddc1428/inline9.js';
import { wasm_bindgen_initialize } from './snippets/stdweb-bb142200b065bd55/inline170.js';
import { __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0 } from './snippets/stdweb-bb142200b065bd55/inline201.js';
import { __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723 } from './snippets/stdweb-bb142200b065bd55/inline279.js';
import { __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6 } from './snippets/stdweb-bb142200b065bd55/inline280.js';
import { __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf } from './snippets/stdweb-bb142200b065bd55/inline281.js';
import { __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63 } from './snippets/stdweb-bb142200b065bd55/inline282.js';
import { __cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec } from './snippets/stdweb-bb142200b065bd55/inline664.js';

let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_24(arg0, arg1, arg2, arg3) {
    wasm._dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd292ce4445945c19(arg0, arg1, arg2, arg3);
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    var ret = wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1ef2302b1494deda(arg0, arg1, arg2);
    return ret;
}

function __wbg_adapter_30(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6162ea44289e0dcf(arg0, arg1, addHeapObject(arg2));
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_33(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha71e35e7aff8851a(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('index-ee9f93ab81344204_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_cargowebsnippet53b6a8dccd8cdae12266b154acb29e83ba989dad_08e74ef70dd05380 = function(arg0, arg1) {
        __cargo_web_snippet_53b6a8dccd8cdae12266b154acb29e83ba989dad(takeObject(arg0), arg1);
    };
    imports.wbg.__wbg_cargowebsnippet9983a55415fa9f55a62367a10f747551caff8c85_7c7d7f6a38f28e60 = function(arg0) {
        __cargo_web_snippet_9983a55415fa9f55a62367a10f747551caff8c85(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet8d3e19a47a694acf0e6fa6cf704bb3bb434f34ef_3d00770a431dbd4d = function(arg0) {
        __cargo_web_snippet_8d3e19a47a694acf0e6fa6cf704bb3bb434f34ef(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet36a43b0918c86173e2053ff2c4da48c9fe2017b1_debb9c1e16b2642b = function(arg0) {
        __cargo_web_snippet_36a43b0918c86173e2053ff2c4da48c9fe2017b1(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet1d08625da0822b42b533903f0c034c4ed935e5ec_7085574d1e96bad7 = function(arg0) {
        __cargo_web_snippet_1d08625da0822b42b533903f0c034c4ed935e5ec(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet9f1bee6f5186fd188b51e8a9eee6bb3815587706_aa7932c2ea72b939 = function(arg0) {
        __cargo_web_snippet_9f1bee6f5186fd188b51e8a9eee6bb3815587706(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippete853589303cd697aa56165d991517b3a3c0b5cf7_895794530212502c = function(arg0) {
        __cargo_web_snippet_e853589303cd697aa56165d991517b3a3c0b5cf7(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippete873d7259824ee2415e78d10e1bfb37f0116a486_64f1841f151d1b94 = function(arg0) {
        __cargo_web_snippet_e873d7259824ee2415e78d10e1bfb37f0116a486(takeObject(arg0));
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_cargowebsnippeta230002551a5ee6325a80144144e81d4886ac6ac_2d389eab22628c6f = function(arg0) {
        __cargo_web_snippet_a230002551a5ee6325a80144144e81d4886ac6ac(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet2e2f59c6ab7049089c8856a5dfac2853485187b5_10ff94c16be16540 = function(arg0) {
        __cargo_web_snippet_2e2f59c6ab7049089c8856a5dfac2853485187b5(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet83015c1e57aa6b41f7f5ad37029c001ad4599ef4_b95e72ba7b966416 = function(arg0) {
        __cargo_web_snippet_83015c1e57aa6b41f7f5ad37029c001ad4599ef4(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippetdde41c57e3e12fe6715329b1be3ecb5968afdf88_b829417e6e064953 = function(arg0) {
        __cargo_web_snippet_dde41c57e3e12fe6715329b1be3ecb5968afdf88(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet1f4505a9a52a1adc65586876e0ebf88890ba0399_7ec2a24c121731db = function(arg0) {
        __cargo_web_snippet_1f4505a9a52a1adc65586876e0ebf88890ba0399(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet31d3816c58be7e6e6a0ea5e7555420abd38a3541_e388dda73f8d9f20 = function(arg0) {
        __cargo_web_snippet_31d3816c58be7e6e6a0ea5e7555420abd38a3541(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet9f5b745f08d25fd3be22a5f88a2b759ee41a6428_f956528d2179ac6c = function(arg0) {
        __cargo_web_snippet_9f5b745f08d25fd3be22a5f88a2b759ee41a6428(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet0cdad3c948ce131f15bd32bb5d601ef1dc2ffa7f_2aa11e7a2348f248 = function(arg0) {
        __cargo_web_snippet_0cdad3c948ce131f15bd32bb5d601ef1dc2ffa7f(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippetf50cc248f06b326fdd4fb11b14df72deea771be8_0e09668c09f3ef26 = function(arg0) {
        __cargo_web_snippet_f50cc248f06b326fdd4fb11b14df72deea771be8(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippeta625bad725ad48c06e48e58d8809bb9ac3c2c2f8_8c15520ebd4d5c23 = function(arg0) {
        __cargo_web_snippet_a625bad725ad48c06e48e58d8809bb9ac3c2c2f8(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippetb9759d3fa5ae6d67b8f5e700438e922317c8f818_a2f08f89a35cf99c = function(arg0) {
        __cargo_web_snippet_b9759d3fa5ae6d67b8f5e700438e922317c8f818(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippetd67ac6baeef6c51abf83b47906cb83107d4f2ee1_58389e74ae22220c = function(arg0) {
        __cargo_web_snippet_d67ac6baeef6c51abf83b47906cb83107d4f2ee1(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet41d85b986b58a974640d51bf2fc956807ef1dd53_220a284c570ee069 = function(arg0) {
        __cargo_web_snippet_41d85b986b58a974640d51bf2fc956807ef1dd53(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet62ba98747f9c2ba20b31b35a4cb3e85e7febd1b6_7bbadffc52307280 = function(arg0) {
        __cargo_web_snippet_62ba98747f9c2ba20b31b35a4cb3e85e7febd1b6(takeObject(arg0));
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        var ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_wasmbindgeninitialize_c1c4df6b494511ad = function(arg0, arg1, arg2, arg3) {
        var ret = wasm_bindgen_initialize(takeObject(arg0), takeObject(arg1), getObject(arg2), getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_cargowebsnippet80d6d56760c65e49b7be8b6b01c1ea861b046bf0_5a8953894b8affd6 = function(arg0, arg1) {
        __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0(takeObject(arg0), arg1);
    };
    imports.wbg.__wbg_cargowebsnippet72fc447820458c720c68d0d8e078ede631edd723_ece3da0a4474dbeb = function(arg0, arg1, arg2, arg3) {
        __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723(takeObject(arg0), arg1, arg2, arg3);
    };
    imports.wbg.__wbg_cargowebsnippet97495987af1720d8a9a923fa4683a7b683e3acd6_a438202dc16f44c0 = function(arg0, arg1, arg2) {
        __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6(takeObject(arg0), arg1, arg2);
    };
    imports.wbg.__wbg_cargowebsnippetdc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf_ce5c721cab10d020 = function(arg0) {
        __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippet1c30acb32a1994a07c75e804ae9855b43f191d63_6d353463ef525961 = function(arg0) {
        __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63(takeObject(arg0));
    };
    imports.wbg.__wbg_cargowebsnippete9638d6405ab65f78daf4a5af9c9de14ecf1e2ec_ad1e81894f802539 = function(arg0, arg1) {
        __cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec(takeObject(arg0), arg1);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_new_693216e109162396 = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbg_warn_2aa0e7178e1d35f6 = function(arg0, arg1) {
        var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
        wasm.__wbindgen_free(arg0, arg1 * 4);
        console.warn(...v0);
    };
    imports.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_1c64944725c0d81d = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_location_f98ad02632f88c43 = function(arg0) {
        var ret = getObject(arg0).location;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_history_3ac8f1466e0c0528 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).history;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_body_78ae4fd43b446013 = function(arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_86c152812a141a62 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElementNS_ae12b8681c3957a3 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        var ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_365db3bc3d0523ab = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_querySelector_b92a6c73bcfe671b = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_target_cc69dde6c2d9ec90 = function(arg0) {
        var ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_cancelBubble_f67c419013823f11 = function(arg0) {
        var ret = getObject(arg0).cancelBubble;
        return ret;
    };
    imports.wbg.__wbg_preventDefault_9866c9fd51eecfb6 = function(arg0) {
        getObject(arg0).preventDefault();
    };
    imports.wbg.__wbg_href_1194e84dd750563a = function(arg0, arg1) {
        var ret = getObject(arg1).href;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setchecked_206243371da58f6a = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__wbg_value_0627d4b1c27534e6 = function(arg0, arg1) {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setvalue_2459f62386b6967f = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_addEventListener_09e11fbf8b4b719b = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_24d5a7c12c3f3c39 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
    }, arguments) };
    imports.wbg.__wbg_pushState_f772e11155235e9c = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).pushState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
    }, arguments) };
    imports.wbg.__wbg_pathname_32da720074d17e34 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg1).pathname;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_parentElement_0253a5d6c3ff0ba5 = function(arg0) {
        var ret = getObject(arg0).parentElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_lastChild_ca5bac177ef353f6 = function(arg0) {
        var ret = getObject(arg0).lastChild;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_setnodeValue_702374ad3d0ec3df = function(arg0, arg1, arg2) {
        getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_appendChild_d318db34c4559916 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_insertBefore_5b314357408fbec1 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_removeChild_d3ca7b53e537867e = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).removeChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Element_97d85e53f1805b82 = function(arg0) {
        var ret = getObject(arg0) instanceof Element;
        return ret;
    };
    imports.wbg.__wbg_namespaceURI_f4cd665d07463337 = function(arg0, arg1) {
        var ret = getObject(arg1).namespaceURI;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_removeAttribute_eea03ed128669b8f = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_setAttribute_1b533bf07966de55 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_pathname_3570d699aa5b91aa = function(arg0, arg1) {
        var ret = getObject(arg1).pathname;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_new_3f48783ae4f87f8f = function() { return handleError(function (arg0, arg1) {
        var ret = new URL(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_value_686b2a68422cb88d = function(arg0, arg1) {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setvalue_0a07023245efa3cc = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_newnoargs_be86524d73f67598 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_888d259a5fefc347 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_valueOf_99d85ac83228d11b = function(arg0) {
        var ret = getObject(arg0).valueOf();
        return ret;
    };
    imports.wbg.__wbg_is_0f5efc7977a2c50b = function(arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_0b83d3df67ecb33e = function() {
        var ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_globalThis_3f735a5746d41fbd = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_c6fbdfc2918d5e58 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_baec038b5ab35c54 = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_1bc0b39582740e95 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_get_4d0f21c2f823742e = function() { return handleError(function (arg0, arg1) {
        var ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_82a4e8a85e31ac42 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_function_table = function() {
        var ret = wasm.__wbindgen_export_2;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper7446 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 507, __wbg_adapter_24);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper7448 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 505, __wbg_adapter_27);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper10248 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 569, __wbg_adapter_30);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper11530 = function(arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 579, __wbg_adapter_33);
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

