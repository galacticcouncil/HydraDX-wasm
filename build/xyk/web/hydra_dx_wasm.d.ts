/* tslint:disable */
/* eslint-disable */
/**
* @param {string} s
* @param {string} b
* @param {string} a
* @returns {string}
*/
export function get_spot_price(s: string, b: string, a: string): string;
/**
* @param {string} s
* @param {string} b
* @param {string} a
* @returns {string}
*/
export function calculate_out_given_in(s: string, b: string, a: string): string;
/**
* @param {string} s
* @param {string} b
* @param {string} a
* @returns {string}
*/
export function calculate_in_given_out(s: string, b: string, a: string): string;
/**
* @param {string} amount
* @returns {string}
*/
export function calculate_default_pool_trade_fee(amount: string): string;
/**
* @param {string} amount
* @param {string} _fee
* @returns {string}
*/
export function calculate_pool_trade_fee(amount: string, _fee: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_spot_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_out_given_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_in_given_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_default_pool_trade_fee: (a: number, b: number, c: number) => void;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
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
