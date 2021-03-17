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
export function get_sell_price(s: string, b: string, a: string): string;
/**
* @param {string} s
* @param {string} b
* @param {string} a
* @returns {string}
*/
export function get_buy_price(s: string, b: string, a: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_spot_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly get_sell_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly get_buy_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
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
        