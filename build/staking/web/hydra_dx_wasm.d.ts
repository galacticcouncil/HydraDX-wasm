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
* @param {string} current_reward_per_stake
* @param {string} pending_rewards
* @param {string} total_stake
* @returns {string}
*/
export function calculate_accumulated_rps(current_reward_per_stake: string, pending_rewards: string, total_stake: string): string;
/**
* @param {string} points
* @param {string} current_stake
* @param {string} stake_increase
* @param {string} stake_weight
* @returns {string}
*/
export function calculate_slashed_points(points: string, current_stake: string, stake_increase: string, stake_weight: string): string;
/**
* @param {string} period_length
* @param {string} block_number
* @returns {string}
*/
export function calculate_period_number(period_length: string, block_number: string): string;
/**
* @param {string} position_created_at
* @param {string} now
* @param {string} time_points_per_period
* @param {string} time_points_weight
* @param {string} action_points
* @param {string} action_points_weight
* @param {string} slashed_points
* @returns {string}
*/
export function calculate_points(position_created_at: string, now: string, time_points_per_period: string, time_points_weight: string, action_points: string, action_points_weight: string, slashed_points: string): string;
/**
* @param {string} x
* @param {string} a
* @param {string} b
* @returns {string}
*/
export function sigmoid(x: string, a: string, b: string): string;
/**
* @param {string} accumulated_reward_per_stake
* @param {string} reward_per_stake
* @param {string} stake
* @returns {string}
*/
export function calculate_rewards(accumulated_reward_per_stake: string, reward_per_stake: string, stake: string): string;
/**
* @param {string} amount
* @param {string} percentage
* @returns {string}
*/
export function calculate_percentage_amount(amount: string, percentage: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_accumulated_rps: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_slashed_points: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly calculate_period_number: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_points: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number) => void;
  readonly sigmoid: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_rewards: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_percentage_amount: (a: number, b: number, c: number, d: number, e: number) => void;
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
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
