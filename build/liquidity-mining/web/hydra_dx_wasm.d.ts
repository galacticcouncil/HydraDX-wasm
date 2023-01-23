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
* @param {string} period
* @param {string} initial_reward_percentage
* @param {string} scale_coef
* @returns {string}
*/
export function calculate_loyalty_multiplier(period: string, initial_reward_percentage: string, scale_coef: string): string;
/**
* @param {string} yield_per_period
* @param {string} total_farm_shares_z
* @param {string} max_reward_per_period
* @returns {string}
*/
export function calculate_global_farm_reward_per_period(yield_per_period: string, total_farm_shares_z: string, max_reward_per_period: string): string;
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
* @param {string} shares
* @param {string} price_adjustment
* @returns {string}
*/
export function calculate_adjusted_shares(shares: string, price_adjustment: string): string;
/**
* @param {string} valued_shares
* @param {string} multiplier
* @returns {string}
*/
export function calculate_global_farm_shares(valued_shares: string, multiplier: string): string;
/**
* @param {string} reward_per_period
* @param {string} periods_since_last_updated
* @returns {string}
*/
export function calculate_rewards_for_periods(reward_per_period: string, periods_since_last_updated: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_loyalty_multiplier: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_global_farm_reward_per_period: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_accumulated_rps: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_user_reward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_user_unclaimed_reward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_valued_shares: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_reward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_adjusted_shares: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_global_farm_shares: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_rewards_for_periods: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
