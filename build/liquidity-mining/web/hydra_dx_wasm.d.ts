/* tslint:disable */
/* eslint-disable */
/**
* @param {string} a
* @param {string} b
* @returns {string}
*/
export function fixed_from_rational(a: string, b: string): string;
/**
* @param {string} period
* @param {string} initial_reward_percentage
* @param {string} scale_coef
* @returns {string}
*/
export function calculate_loyalty_multiplier(period: string, initial_reward_percentage: string, scale_coef: string): string;
/**
* @param {string} accumulated_rps_now
* @param {string} total_shares
* @param {string} reward
* @returns {string}
*/
export function calculate_accumulated_rps(accumulated_rps_now: string, total_shares: string, reward: string): string;
/**
* @param {string} accumulated_rpvs
* @param {string} valued_shares
* @param {string} accumulated_claimed_rewards
* @param {string} accumulated_rpvs_now
* @param {string} loyalty_multiplier
* @returns {string}
*/
export function calculate_user_reward(accumulated_rpvs: string, valued_shares: string, accumulated_claimed_rewards: string, accumulated_rpvs_now: string, loyalty_multiplier: string): string;
/**
* @param {string} accumulated_rpvs
* @param {string} valued_shares
* @param {string} accumulated_claimed_rewards
* @param {string} accumulated_rpvs_now
* @param {string} loyalty_multiplier
* @returns {string}
*/
export function calculate_user_unclaimed_reward(accumulated_rpvs: string, valued_shares: string, accumulated_claimed_rewards: string, accumulated_rpvs_now: string, loyalty_multiplier: string): string;
/**
* @param {string} shares
* @param {string} incentivized_asset_balance
* @returns {string}
*/
export function calculate_valued_shares(shares: string, incentivized_asset_balance: string): string;
/**
* @param {string} accumulated_rps_start
* @param {string} accumulated_rps_now
* @param {string} shares
* @returns {string}
*/
export function calculate_reward(accumulated_rps_start: string, accumulated_rps_now: string, shares: string): string;
/**
* @param {string} valued_shares
* @param {string} multiplier
* @returns {string}
*/
export function calculate_global_farm_shares(valued_shares: string, multiplier: string): string;
/**
* @param {string} yield_farm_rpz
* @param {string} global_farm_rpz
* @param {string} multiplier
* @param {string} total_valued_shares
* @returns {string}
*/
export function calculate_yield_farm_rewards(yield_farm_rpz: string, global_farm_rpz: string, multiplier: string, total_valued_shares: string): string;
/**
* @param {string} yield_farm_rpz
* @param {string} global_farm_rpz
* @param {string} multiplier
* @param {string} total_valued_shares
* @returns {string}
*/
export function calculate_yield_farm_delta_rpvs(yield_farm_rpz: string, global_farm_rpz: string, multiplier: string, total_valued_shares: string): string;
/**
* @param {string} total_shares_z
* @param {string} price_adjustment
* @param {string} yield_per_period
* @param {string} max_reward_per_period
* @param {string} periods_since_last_update
* @returns {string}
*/
export function calculate_global_farm_rewards(total_shares_z: string, price_adjustment: string, yield_per_period: string, max_reward_per_period: string, periods_since_last_update: string): string;
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
  readonly fixed_from_rational: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_loyalty_multiplier: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_accumulated_rps: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_user_reward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_user_unclaimed_reward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_valued_shares: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_reward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_global_farm_shares: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_yield_farm_rewards: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly calculate_yield_farm_delta_rpvs: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly calculate_global_farm_rewards: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
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
