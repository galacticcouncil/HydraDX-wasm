/* tslint:disable */
/* eslint-disable */
export function calculate_hollar_out_given_collateral_in(amount_in: string, collateral_peg: string, purchase_fee: string): string;
export function calculate_collateral_in_given_hollar_out(amount_out: string, collateral_peg: string, purchase_fee: string): string;
export function calculate_collateral_out_given_hollar_in(amount_in: string, execution_price: string, buyback_fee: string): string;
export function calculate_hollar_in_given_collateral_out(amount_out: string, execution_price: string, buyback_fee: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly calculate_hollar_out_given_collateral_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_collateral_in_given_hollar_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_collateral_out_given_hollar_in: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly calculate_hollar_in_given_collateral_out: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
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
