/* tslint:disable */
/* eslint-disable */
/**
* Calculate the iterated exponential moving average for the given prices.
* + `iterations` is the number of iterations of the EMA to calculate (expected to be a serialized `u32`).
* + `prev_n` and `prev_d` are the previous oracle value, `incoming_n` and `incoming_d` are the new value to
*   integrate (expected to be serialized `u128` values).
* + `smoothing` is the smoothing factor of the EMA (expected to be a serialized `u128` that gets interpreted as a
*   `Fraction`).
*
* Returns the new oracle value as a serialized `FixedU128` (lower precision than the input).
* @param {string} iterations
* @param {string} prev_n
* @param {string} prev_d
* @param {string} incoming_n
* @param {string} incoming_d
* @param {string} smoothing
* @returns {string}
*/
export function low_precision_iterated_price_ema(iterations: string, prev_n: string, prev_d: string, incoming_n: string, incoming_d: string, smoothing: string): string;
/**
* Calculate the iterated exponential moving average for the given balances.
* + `iterations` is the number of iterations of the EMA to calculate (expected to be a serialized `u32`).
* + `prev` is the previous oracle value, `incoming` is the new value to integrate (expected to be serialized
*   `u128` values).
* + `smoothing` is the smoothing factor of the EMA (expected to be a serialized `u128` that gets interpreted as a
*   `Fraction`).
*
* Returns the new oracle value as a serialized `u128`.
* @param {string} iterations
* @param {string} prev
* @param {string} incoming
* @param {string} smoothing
* @returns {string}
*/
export function iterated_balance_ema(iterations: string, prev: string, incoming: string, smoothing: string): string;
/**
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly low_precision_iterated_price_ema: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => void;
  readonly iterated_balance_ema: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
