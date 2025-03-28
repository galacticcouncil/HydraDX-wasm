/* tslint:disable */
/* eslint-disable */
export function calculate_out_given_in(reserves: string, asset_in: number, asset_out: number, amount_in: string, amplification: string, fee: string, pegs: string): string;
export function calculate_in_given_out(reserves: string, asset_in: number, asset_out: number, amount_out: string, amplification: string, fee: string, pegs: string): string;
export function calculate_amplification(initial_amplification: string, final_amplification: string, initial_block: string, final_block: string, current_block: string): string;
export function calculate_shares(reserves: string, assets: string, amplification: string, share_issuance: string, fee: string, pegs: string): string;
export function calculate_spot_price_with_fee(pool_id: string, reserves: string, amplification: string, asset_in: string, asset_out: string, share_issuance: string, fee: string, pegs: string): string;
export function calculate_shares_for_amount(reserves: string, asset_in: number, amount: string, amplification: string, share_issuance: string, fee: string, pegs: string): string;
export function calculate_add_one_asset(reserves: string, shares: string, asset_in: number, amplification: string, share_issuance: string, fee: string, pegs: string): string;
export function pool_account_name(share_asset_id: number): Uint8Array;
export function calculate_liquidity_out_one_asset(reserves: string, shares: string, asset_out: number, amplification: string, share_issuance: string, withdraw_fee: string, pegs: string): string;
export function recalculate_peg(current_pegs: string, target_pegs: string, current_block: string, max_peg_update: string, pool_fee: string): string;
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_out_given_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => void;
  readonly calculate_in_given_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => void;
  readonly calculate_amplification: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_shares: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => void;
  readonly calculate_spot_price_with_fee: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => void;
  readonly calculate_shares_for_amount: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => void;
  readonly calculate_add_one_asset: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => void;
  readonly pool_account_name: (a: number, b: number) => void;
  readonly calculate_liquidity_out_one_asset: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => void;
  readonly recalculate_peg: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
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
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
