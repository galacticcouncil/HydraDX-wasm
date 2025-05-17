/* tslint:disable */
/* eslint-disable */
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;
export function calculate_accumulated_rps(current_reward_per_stake: string, pending_rewards: string, total_stake: string): string;
export function calculate_slashed_points(points: string, current_stake: string, stake_increase: string, stake_weight: string, min_slash_point: string): string;
export function calculate_period_number(period_length: string, block_number: string, six_sec_block_since: string): string;
export function calculate_points(position_created_at: string, now: string, time_points_per_period: string, time_points_weight: string, action_points: string, action_points_weight: string, slashed_points: string): string;
export function sigmoid(x: string, a: string, b: string): string;
export function calculate_rewards(accumulated_reward_per_stake: string, reward_per_stake: string, stake: string): string;
export function calculate_percentage_amount(amount: string, percentage: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly calculate_accumulated_rps: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_slashed_points: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly calculate_period_number: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_points: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number) => void;
  readonly sigmoid: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_rewards: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_percentage_amount: (a: number, b: number, c: number, d: number, e: number) => void;
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
