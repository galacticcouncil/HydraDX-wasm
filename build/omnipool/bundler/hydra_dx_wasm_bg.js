import * as wasm from './hydra_dx_wasm_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @param {AssetState} asset_state
* @param {string} amount_in
* @returns {MathResult}
*/
export function calculate_shares(asset_state, amount_in) {
    _assertClass(asset_state, AssetState);
    var ptr0 = asset_state.ptr;
    asset_state.ptr = 0;
    const ptr1 = passStringToWasm0(amount_in, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.calculate_shares(ptr0, ptr1, len1);
    return MathResult.__wrap(ret);
}

/**
* @param {AssetState} asset_state
* @param {Position} position
* @param {string} shares
* @returns {LiquidityOutResult}
*/
export function calculate_liquidity_out(asset_state, position, shares) {
    _assertClass(asset_state, AssetState);
    var ptr0 = asset_state.ptr;
    asset_state.ptr = 0;
    _assertClass(position, Position);
    var ptr1 = position.ptr;
    position.ptr = 0;
    const ptr2 = passStringToWasm0(shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.calculate_liquidity_out(ptr0, ptr1, ptr2, len2);
    return LiquidityOutResult.__wrap(ret);
}

/**
* @param {AssetState} asset_in_state
* @param {AssetState} asset_out_state
* @param {string} amount_in
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {MathResult}
*/
export function calculate_out_given_in(asset_in_state, asset_out_state, amount_in, asset_fee, protocol_fee) {
    _assertClass(asset_in_state, AssetState);
    var ptr0 = asset_in_state.ptr;
    asset_in_state.ptr = 0;
    _assertClass(asset_out_state, AssetState);
    var ptr1 = asset_out_state.ptr;
    asset_out_state.ptr = 0;
    const ptr2 = passStringToWasm0(amount_in, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(asset_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passStringToWasm0(protocol_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.calculate_out_given_in(ptr0, ptr1, ptr2, len2, ptr3, len3, ptr4, len4);
    return MathResult.__wrap(ret);
}

/**
* @param {AssetState} asset_in_state
* @param {AssetState} asset_out_state
* @param {string} amount_out
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {MathResult}
*/
export function calculate_in_given_out(asset_in_state, asset_out_state, amount_out, asset_fee, protocol_fee) {
    _assertClass(asset_in_state, AssetState);
    var ptr0 = asset_in_state.ptr;
    asset_in_state.ptr = 0;
    _assertClass(asset_out_state, AssetState);
    var ptr1 = asset_out_state.ptr;
    asset_out_state.ptr = 0;
    const ptr2 = passStringToWasm0(amount_out, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(asset_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passStringToWasm0(protocol_fee, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.calculate_in_given_out(ptr0, ptr1, ptr2, len2, ptr3, len3, ptr4, len4);
    return MathResult.__wrap(ret);
}

/**
*/
export class AssetState {

    static __wrap(ptr) {
        const obj = Object.create(AssetState.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_assetstate_free(ptr);
    }
    /**
    * @param {string} reserve
    * @param {string} hub_reserve
    * @param {string} shares
    */
    constructor(reserve, hub_reserve, shares) {
        const ptr0 = passStringToWasm0(reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(hub_reserve, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.assetstate_new(ptr0, len0, ptr1, len1, ptr2, len2);
        return AssetState.__wrap(ret);
    }
}
/**
*/
export class LiquidityOutResult {

    static __wrap(ptr) {
        const obj = Object.create(LiquidityOutResult.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_liquidityoutresult_free(ptr);
    }
    /**
    * @returns {string}
    */
    get_asset_amount() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.liquidityoutresult_get_asset_amount(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {string}
    */
    get_lrna_amount() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.liquidityoutresult_get_lrna_amount(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {boolean}
    */
    is_error() {
        const ret = wasm.liquidityoutresult_is_error(this.ptr);
        return ret !== 0;
    }
}
/**
*/
export class MathResult {

    static __wrap(ptr) {
        const obj = Object.create(MathResult.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mathresult_free(ptr);
    }
    /**
    * @returns {string}
    */
    get_result() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.mathresult_get_result(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * @returns {boolean}
    */
    is_error() {
        const ret = wasm.mathresult_is_error(this.ptr);
        return ret !== 0;
    }
}
/**
*/
export class Position {

    static __wrap(ptr) {
        const obj = Object.create(Position.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_position_free(ptr);
    }
    /**
    * @param {string} amount
    * @param {string} shares
    * @param {string} price
    */
    constructor(amount, shares, price) {
        const ptr0 = passStringToWasm0(amount, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(shares, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(price, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.position_new(ptr0, len0, ptr1, len1, ptr2, len2);
        return Position.__wrap(ret);
    }
}
/**
*/
export class Tradability {

    static __wrap(ptr) {
        const obj = Object.create(Tradability.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tradability_free(ptr);
    }
    /**
    * @param {number} bits
    */
    constructor(bits) {
        const ret = wasm.tradability_new(bits);
        return Tradability.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    can_sell() {
        const ret = wasm.tradability_can_sell(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    can_buy() {
        const ret = wasm.tradability_can_buy(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    can_add_liquidity() {
        const ret = wasm.tradability_can_add_liquidity(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    can_remove_liquidity() {
        const ret = wasm.tradability_can_remove_liquidity(this.ptr);
        return ret !== 0;
    }
}

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

