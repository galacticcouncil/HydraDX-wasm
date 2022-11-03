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
