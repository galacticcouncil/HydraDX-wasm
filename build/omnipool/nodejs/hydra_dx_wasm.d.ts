/* tslint:disable */
/* eslint-disable */
/**
* @param {AssetState} asset_state
* @param {string} amount_in
* @returns {MathResult}
*/
export function calculate_shares(asset_state: AssetState, amount_in: string): MathResult;
/**
* @param {AssetState} asset_state
* @param {Position} position
* @param {string} shares
* @returns {LiquidityOutResult}
*/
export function calculate_liquidity_out(asset_state: AssetState, position: Position, shares: string): LiquidityOutResult;
/**
* @param {AssetState} asset_in_state
* @param {AssetState} asset_out_state
* @param {string} amount_in
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {MathResult}
*/
export function calculate_out_given_in(asset_in_state: AssetState, asset_out_state: AssetState, amount_in: string, asset_fee: string, protocol_fee: string): MathResult;
/**
* @param {AssetState} asset_in_state
* @param {AssetState} asset_out_state
* @param {string} amount_out
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {MathResult}
*/
export function calculate_in_given_out(asset_in_state: AssetState, asset_out_state: AssetState, amount_out: string, asset_fee: string, protocol_fee: string): MathResult;
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
export class LiquidityOutResult {
  free(): void;
/**
* @returns {string}
*/
  get_asset_amount(): string;
/**
* @returns {string}
*/
  get_lrna_amount(): string;
/**
* @returns {boolean}
*/
  is_error(): boolean;
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
/**
*/
export class Position {
  free(): void;
/**
* @param {string} amount
* @param {string} shares
* @param {string} price
*/
  constructor(amount: string, shares: string, price: string);
}
