/* tslint:disable */
/* eslint-disable */
/**
* @param {string} s
* @param {string} b
* @param {string} s_w
* @param {string} b_w
* @param {string} a
* @returns {string}
*/
export function get_spot_price(s: string, b: string, s_w: string, b_w: string, a: string): string;
/**
* @param {string} s
* @param {string} b
* @param {string} s_w
* @param {string} b_w
* @returns {string}
*/
export function calculate_spot_price(s: string, b: string, s_w: string, b_w: string): string;
/**
* @param {string} s
* @param {string} b
* @param {string} s_w
* @param {string} b_w
* @param {string} fee_asset
* @param {string} asset_out
* @param {string} fee_rate_n
* @param {string} fee_rate_d
* @returns {string}
*/
export function calculate_spot_price_with_fee(s: string, b: string, s_w: string, b_w: string, fee_asset: string, asset_out: string, fee_rate_n: string, fee_rate_d: string): string;
/**
* @param {string} s
* @param {string} b
* @param {string} s_w
* @param {string} b_w
* @param {string} a
* @returns {string}
*/
export function calculate_out_given_in(s: string, b: string, s_w: string, b_w: string, a: string): string;
/**
* @param {string} s
* @param {string} b
* @param {string} s_w
* @param {string} b_w
* @param {string} a
* @returns {string}
*/
export function calculate_in_given_out(s: string, b: string, s_w: string, b_w: string, a: string): string;
/**
* @param {string} start_x
* @param {string} end_x
* @param {string} start_y
* @param {string} end_y
* @param {string} at
* @returns {string}
*/
export function calculate_linear_weights(start_x: string, end_x: string, start_y: string, end_y: string, at: string): string;
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
  readonly get_spot_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_spot_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly calculate_spot_price_with_fee: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => void;
  readonly calculate_out_given_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_in_given_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_linear_weights: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
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
