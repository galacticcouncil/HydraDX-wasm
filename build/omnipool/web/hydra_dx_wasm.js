
let wasm;

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = new TextEncoder('utf-8');

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

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a, fee_numerator, fee_denominator) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(a, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.calculate_pool_trade_fee(retptr, ptr0, len0, fee_numerator, fee_denominator);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_shares
* @param {string} amount_in
* @returns {string}
*/
export function calculate_shares(asset_reserve, asset_hub_reserve, asset_shares, amount_in) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(amount_in, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        wasm.calculate_shares(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_shares
* @param {string} position_amount
* @param {string} position_shares
* @param {string} position_price
* @param {string} shares_to_remove
* @returns {string}
*/
export function calculate_liquidity_out(asset_reserve, asset_hub_reserve, asset_shares, position_amount, position_shares, position_price, shares_to_remove) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(position_amount, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(position_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len4 = WASM_VECTOR_LEN;
        const ptr5 = passStringToWasm0(position_price, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len5 = WASM_VECTOR_LEN;
        const ptr6 = passStringToWasm0(shares_to_remove, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len6 = WASM_VECTOR_LEN;
        wasm.calculate_liquidity_out(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_shares
* @param {string} position_amount
* @param {string} position_shares
* @param {string} position_price
* @param {string} shares_to_remove
* @returns {string}
*/
export function calculate_liquidity_lrna_out(asset_reserve, asset_hub_reserve, asset_shares, position_amount, position_shares, position_price, shares_to_remove) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(position_amount, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(position_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len4 = WASM_VECTOR_LEN;
        const ptr5 = passStringToWasm0(position_price, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len5 = WASM_VECTOR_LEN;
        const ptr6 = passStringToWasm0(shares_to_remove, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len6 = WASM_VECTOR_LEN;
        wasm.calculate_liquidity_lrna_out(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_in_reserve
* @param {string} asset_in_hub_reserve
* @param {string} asset_in_shares
* @param {string} asset_out_reserve
* @param {string} asset_out_hub_reserve
* @param {string} asset_out_shares
* @param {string} amount_in
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {string}
*/
export function calculate_out_given_in(asset_in_reserve, asset_in_hub_reserve, asset_in_shares, asset_out_reserve, asset_out_hub_reserve, asset_out_shares, amount_in, asset_fee, protocol_fee) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_in_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_in_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_in_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(asset_out_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(asset_out_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len4 = WASM_VECTOR_LEN;
        const ptr5 = passStringToWasm0(asset_out_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len5 = WASM_VECTOR_LEN;
        const ptr6 = passStringToWasm0(amount_in, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len6 = WASM_VECTOR_LEN;
        const ptr7 = passStringToWasm0(asset_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len7 = WASM_VECTOR_LEN;
        const ptr8 = passStringToWasm0(protocol_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len8 = WASM_VECTOR_LEN;
        wasm.calculate_out_given_in(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6, ptr7, len7, ptr8, len8);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_in_reserve
* @param {string} asset_in_hub_reserve
* @param {string} asset_in_shares
* @param {string} asset_out_reserve
* @param {string} asset_out_hub_reserve
* @param {string} asset_out_shares
* @param {string} amount_out
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {string}
*/
export function calculate_in_given_out(asset_in_reserve, asset_in_hub_reserve, asset_in_shares, asset_out_reserve, asset_out_hub_reserve, asset_out_shares, amount_out, asset_fee, protocol_fee) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_in_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_in_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_in_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(asset_out_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(asset_out_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len4 = WASM_VECTOR_LEN;
        const ptr5 = passStringToWasm0(asset_out_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len5 = WASM_VECTOR_LEN;
        const ptr6 = passStringToWasm0(amount_out, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len6 = WASM_VECTOR_LEN;
        const ptr7 = passStringToWasm0(asset_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len7 = WASM_VECTOR_LEN;
        const ptr8 = passStringToWasm0(protocol_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len8 = WASM_VECTOR_LEN;
        wasm.calculate_in_given_out(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6, ptr7, len7, ptr8, len8);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_a_reserve
* @param {string} asset_a_hub_reserve
* @param {string} asset_b_reserve
* @param {string} asset_b_hub_reserve
* @returns {string}
*/
export function calculate_spot_price(asset_a_reserve, asset_a_hub_reserve, asset_b_reserve, asset_b_hub_reserve) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_a_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_a_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_b_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(asset_b_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        wasm.calculate_spot_price(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_cap
* @param {string} total_hub_reserve
* @returns {string}
*/
export function calculate_cap_difference(asset_reserve, asset_hub_reserve, asset_cap, total_hub_reserve) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(total_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        wasm.calculate_cap_difference(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} asset_hub_reserve
* @param {string} asset_cap
* @param {string} hub_added
* @param {string} total_hub_reserve
* @returns {boolean}
*/
export function verify_asset_cap(asset_hub_reserve, asset_cap, hub_added, total_hub_reserve) {
    const ptr0 = passStringToWasm0(asset_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(asset_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(hub_added, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(total_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len3 = WASM_VECTOR_LEN;
    const ret = wasm.verify_asset_cap(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    return ret !== 0;
}

/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_shares
* @param {string} amount_in
* @returns {string}
*/
export function calculate_liquidity_hub_in(asset_reserve, asset_hub_reserve, asset_shares, amount_in) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(asset_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(asset_hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(asset_shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(amount_in, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        wasm.calculate_liquidity_hub_in(retptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {number} bits
* @returns {boolean}
*/
export function is_sell_allowed(bits) {
    const ret = wasm.is_sell_allowed(bits);
    return ret !== 0;
}

/**
* @param {number} bits
* @returns {boolean}
*/
export function is_buy_allowed(bits) {
    const ret = wasm.is_buy_allowed(bits);
    return ret !== 0;
}

/**
* @param {number} bits
* @returns {boolean}
*/
export function is_add_liquidity_allowed(bits) {
    const ret = wasm.is_add_liquidity_allowed(bits);
    return ret !== 0;
}

/**
* @param {number} bits
* @returns {boolean}
*/
export function is_remove_liquidity_allowed(bits) {
    const ret = wasm.is_remove_liquidity_allowed(bits);
    return ret !== 0;
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

function getImports() {
    const imports = {};
    imports.wbg = {};

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = new Int32Array();
    cachedUint8Memory0 = new Uint8Array();


    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('hydra_dx_wasm_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
