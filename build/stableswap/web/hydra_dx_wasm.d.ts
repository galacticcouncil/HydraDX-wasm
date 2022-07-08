/* tslint:disable */
/* eslint-disable */
/**
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;
/**
* @param {string} reserve_in
* @param {string} reserve_out
* @param {string} amount
* @returns {string}
*/
export function get_spot_price(reserve_in: string, reserve_out: string, amount: string): string;
/**
* @param {string} reserve_in
* @param {string} reserve_out
* @param {string} amount_in
* @param {string} amplification
* @param {string} precision
* @returns {string}
*/
export function calculate_out_given_in(reserve_in: string, reserve_out: string, amount_in: string, amplification: string, precision: string): string;
/**
* @param {string} reserve_in
* @param {string} reserve_out
* @param {string} amount_out
* @param {string} amplification
* @param {string} precision
* @returns {string}
*/
export function calculate_in_given_out(reserve_in: string, reserve_out: string, amount_out: string, amplification: string, precision: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly get_spot_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_out_given_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_in_given_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
