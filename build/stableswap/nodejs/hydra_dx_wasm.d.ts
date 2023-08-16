/* tslint:disable */
/* eslint-disable */
/**
* @param {string} reserves
* @param {number} asset_in
* @param {number} asset_out
* @param {string} amount_in
* @param {string} amplification
* @param {string} fee
* @returns {string}
*/
export function calculate_out_given_in(reserves: string, asset_in: number, asset_out: number, amount_in: string, amplification: string, fee: string): string;
/**
* @param {string} reserves
* @param {number} asset_in
* @param {number} asset_out
* @param {string} amount_out
* @param {string} amplification
* @param {string} fee
* @returns {string}
*/
export function calculate_in_given_out(reserves: string, asset_in: number, asset_out: number, amount_out: string, amplification: string, fee: string): string;
/**
* @param {string} initial_amplification
* @param {string} final_amplification
* @param {string} initial_block
* @param {string} final_block
* @param {string} current_block
* @returns {string}
*/
export function calculate_amplification(initial_amplification: string, final_amplification: string, initial_block: string, final_block: string, current_block: string): string;
/**
* @param {string} reserves
* @param {string} assets
* @param {string} amplification
* @param {string} share_issuance
* @returns {string}
*/
export function calculate_shares(reserves: string, assets: string, amplification: string, share_issuance: string): string;
/**
* @param {number} share_asset_id
* @returns {string}
*/
export function pool_account_name(share_asset_id: number): string;
/**
* @param {string} share_asset_id
* @returns {string}
*/
export function stable_pool_account_name(share_asset_id: string): string;
/**
* @param {string} reserves
* @param {string} shares
* @param {number} asset_out
* @param {string} amplification
* @param {string} share_issuance
* @param {string} withdraw_fee
* @returns {string}
*/
export function calculate_liquidity_out_one_asset(reserves: string, shares: string, asset_out: number, amplification: string, share_issuance: string, withdraw_fee: string): string;
/**
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;
