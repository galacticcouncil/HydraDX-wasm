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
* @param {string} reserves
* @param {number} asset_in
* @param {number} asset_out
* @param {string} amount_in
* @param {string} amplification
* @param {string} fee
* @returns {string}
*/
export function calculate_out_given_in(reserves: string, asset_in: number, asset_out: number, amount_in: string, amplification: string, fee: string): string;
/**
* @param {string} reserves
* @param {number} asset_in
* @param {number} asset_out
* @param {string} amount_out
* @param {string} amplification
* @param {string} fee
* @returns {string}
*/
export function calculate_in_given_out(reserves: string, asset_in: number, asset_out: number, amount_out: string, amplification: string, fee: string): string;
/**
* @param {string} initial_amplification
* @param {string} final_amplification
* @param {string} initial_block
* @param {string} final_block
* @param {string} current_block
* @returns {string}
*/
export function calculate_amplification(initial_amplification: string, final_amplification: string, initial_block: string, final_block: string, current_block: string): string;
/**
* @param {string} reserves
* @param {string} assets
* @param {string} amplification
* @param {string} share_issuance
* @param {string} fee
* @returns {string}
*/
export function calculate_shares(reserves: string, assets: string, amplification: string, share_issuance: string, fee: string): string;
/**
* @param {number} share_asset_id
* @returns {Uint8Array}
*/
export function pool_account_name(share_asset_id: number): Uint8Array;
/**
* @param {string} reserves
* @param {string} shares
* @param {number} asset_out
* @param {string} amplification
* @param {string} share_issuance
* @param {string} withdraw_fee
* @returns {string}
*/
export function calculate_liquidity_out_one_asset(reserves: string, shares: string, asset_out: number, amplification: string, share_issuance: string, withdraw_fee: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_out_given_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_in_given_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_amplification: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_shares: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly pool_account_name: (a: number, b: number) => void;
  readonly calculate_liquidity_out_one_asset: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
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
