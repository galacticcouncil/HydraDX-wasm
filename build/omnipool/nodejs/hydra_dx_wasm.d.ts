/* tslint:disable */
/* eslint-disable */
/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_shares
* @param {string} amount_in
* @returns {string}
*/
export function calculate_shares(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, amount_in: string): string;
/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_shares
* @param {string} position_amount
* @param {string} position_shares
* @param {string} position_price
* @param {string} shares_to_remove
* @returns {string}
*/
export function calculate_liquidity_out(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, position_amount: string, position_shares: string, position_price: string, shares_to_remove: string): string;
/**
* @param {string} asset_reserve
* @param {string} asset_hub_reserve
* @param {string} asset_shares
* @param {string} position_amount
* @param {string} position_shares
* @param {string} position_price
* @param {string} shares_to_remove
* @returns {string}
*/
export function calculate_liquidity_lrna_out(asset_reserve: string, asset_hub_reserve: string, asset_shares: string, position_amount: string, position_shares: string, position_price: string, shares_to_remove: string): string;
/**
* @param {string} asset_in_reserve
* @param {string} asset_in_hub_reserve
* @param {string} asset_in_shares
* @param {string} asset_out_reserve
* @param {string} asset_out_hub_reserve
* @param {string} asset_out_shares
* @param {string} amount_in
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {string}
*/
export function calculate_out_given_in(asset_in_reserve: string, asset_in_hub_reserve: string, asset_in_shares: string, asset_out_reserve: string, asset_out_hub_reserve: string, asset_out_shares: string, amount_in: string, asset_fee: string, protocol_fee: string): string;
/**
* @param {string} asset_in_reserve
* @param {string} asset_in_hub_reserve
* @param {string} asset_in_shares
* @param {string} asset_out_reserve
* @param {string} asset_out_hub_reserve
* @param {string} asset_out_shares
* @param {string} amount_out
* @param {string} asset_fee
* @param {string} protocol_fee
* @returns {string}
*/
export function calculate_in_given_out(asset_in_reserve: string, asset_in_hub_reserve: string, asset_in_shares: string, asset_out_reserve: string, asset_out_hub_reserve: string, asset_out_shares: string, amount_out: string, asset_fee: string, protocol_fee: string): string;
/**
* @param {string} asset_a_reserve
* @param {string} asset_a_hub_reserve
* @param {string} asset_b_reserve
* @param {string} asset_b_hub_reserve
* @returns {string}
*/
export function calculate_spot_price(asset_a_reserve: string, asset_a_hub_reserve: string, asset_b_reserve: string, asset_b_hub_reserve: string): string;
/**
* @param {string} asset_hub_reserve
* @param {string} asset_cap
* @param {string} total_hub_reserve
* @returns {string}
*/
export function calculate_cap_difference(asset_hub_reserve: string, asset_cap: string, total_hub_reserve: string): string;
/**
* @param {string} asset_hub_reserve
* @param {string} asset_cap
* @param {string} hub_added
* @param {string} total_hub_reserve
* @returns {boolean}
*/
export function verify_asset_cap(asset_hub_reserve: string, asset_cap: string, hub_added: string, total_hub_reserve: string): boolean;
/**
* @param {number} bits
* @returns {boolean}
*/
export function is_sell_allowed(bits: number): boolean;
/**
* @param {number} bits
* @returns {boolean}
*/
export function is_buy_allowed(bits: number): boolean;
/**
* @param {number} bits
* @returns {boolean}
*/
export function is_add_liquidity_allowed(bits: number): boolean;
/**
* @param {number} bits
* @returns {boolean}
*/
export function is_remove_liquidity_allowed(bits: number): boolean;
/**
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;
