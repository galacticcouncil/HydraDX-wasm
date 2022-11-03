/* tslint:disable */
/* eslint-disable */
/**
* @param {AssetState} asset_state
* @param {string} amount_in
* @returns {MathResult}
*/
export function calculate_shares(asset_state: AssetState, amount_in: string): MathResult;
/**
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;
/**
*/
export class AssetState {
  free(): void;
/**
* @param {string} reserve
* @param {string} hub_reserve
* @param {string} shares
*/
  constructor(reserve: string, hub_reserve: string, shares: string);
}
/**
*/
export class MathResult {
  free(): void;
/**
* @returns {string}
*/
  get_result(): string;
/**
* @returns {boolean}
*/
  is_error(): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_mathresult_free: (a: number) => void;
  readonly mathresult_get_result: (a: number, b: number) => void;
  readonly mathresult_is_error: (a: number) => number;
  readonly __wbg_assetstate_free: (a: number) => void;
  readonly assetstate_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly calculate_shares: (a: number, b: number, c: number) => number;
  readonly calculate_pool_trade_fee: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
