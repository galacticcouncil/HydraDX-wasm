/* tslint:disable */
/* eslint-disable */
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
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;
