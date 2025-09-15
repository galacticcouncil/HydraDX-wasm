/* tslint:disable */
/* eslint-disable */
export function calculate_shares(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, amount_in: string): string;
export function calculate_withdrawal_fee(spot_price: string, oracle_price: string, min_withdrawal_fee: string): string;
export function calculate_liquidity_out(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, position_amount: string, position_shares: string, position_price: string, shares_to_remove: string, withdrawal_fee: string): string;
export function calculate_liquidity_lrna_out(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, position_amount: string, position_shares: string, position_price: string, shares_to_remove: string, withdrawal_fee: string): string;
export function recalculate_asset_fee(oracle_amount_in: string, oracle_amount_out: string, oracle_liquidity: string, oracle_period: string, current_asset_liquidity: string, previous_fee: string, block_difference: string, min_fee: string, max_fee: string, decay: string, amplification: string): string;
export function recalculate_protocol_fee(oracle_amount_in: string, oracle_amount_out: string, oracle_liquidity: string, oracle_period: string, current_asset_liquidity: string, previous_fee: string, block_difference: string, min_fee: string, max_fee: string, decay: string, amplification: string): string;
export function calculate_out_given_in(asset_in_reserve: string, asset_in_hub_reserve: string, asset_in_shares: string, asset_out_reserve: string, asset_out_hub_reserve: string, asset_out_shares: string, amount_in: string, asset_fee: string, protocol_fee: string): string;
export function calculate_out_given_lrna_in(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, amount_in: string, asset_fee: string): string;
export function calculate_in_given_out(asset_in_reserve: string, asset_in_hub_reserve: string, asset_in_shares: string, asset_out_reserve: string, asset_out_hub_reserve: string, asset_out_shares: string, amount_out: string, asset_fee: string, protocol_fee: string): string;
export function calculate_lrna_in_given_out(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, amount_out: string, asset_fee: string): string;
export function calculate_spot_price(asset_a_reserve: string, asset_a_hub_reserve: string, asset_b_reserve: string, asset_b_hub_reserve: string): string;
export function calculate_spot_price_with_fee(asset_a_reserve: string, asset_a_hub_reserve: string, asset_b_reserve: string, asset_b_hub_reserve: string, protocol_fee: string, asset_fee: string): string;
export function calculate_lrna_spot_price(asset_reserve: string, asset_hub_reserve: string): string;
export function calculate_lrna_spot_price_with_fee(asset_reserve: string, asset_hub_reserve: string, asset_fee: string): string;
export function calculate_cap_difference(asset_reserve: string, asset_hub_reserve: string, asset_cap: string, total_hub_reserve: string): string;
export function verify_asset_cap(asset_hub_reserve: string, asset_cap: string, hub_added: string, total_hub_reserve: string): boolean;
export function calculate_tvl_cap_difference(asset_reserve: string, asset_hub_reserve: string, stable_asset_reserve: string, stable_asset_hub_reserve: string, tvl_cap: string, total_hub_reserve: string): string;
export function calculate_liquidity_hub_in(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, amount_in: string): string;
export function is_sell_allowed(bits: number): boolean;
export function is_buy_allowed(bits: number): boolean;
export function is_add_liquidity_allowed(bits: number): boolean;
export function is_remove_liquidity_allowed(bits: number): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_shares: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly calculate_withdrawal_fee: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_liquidity_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => void;
  readonly calculate_liquidity_lrna_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => void;
  readonly recalculate_asset_fee: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number) => void;
  readonly recalculate_protocol_fee: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number) => void;
  readonly calculate_out_given_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number) => void;
  readonly calculate_out_given_lrna_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_in_given_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number) => void;
  readonly calculate_lrna_in_given_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_spot_price: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly calculate_spot_price_with_fee: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => void;
  readonly calculate_lrna_spot_price: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_lrna_spot_price_with_fee: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_cap_difference: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly verify_asset_cap: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly calculate_tvl_cap_difference: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => void;
  readonly calculate_liquidity_hub_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly is_sell_allowed: (a: number) => number;
  readonly is_buy_allowed: (a: number) => number;
  readonly is_add_liquidity_allowed: (a: number) => number;
  readonly is_remove_liquidity_allowed: (a: number) => number;
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
