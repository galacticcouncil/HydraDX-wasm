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
* @param {string} reserve_in
* @param {string} reserve_out
* @param {string} amount
* @returns {string}
*/
export function get_spot_price(reserve_in: string, reserve_out: string, amount: string): string;
/**
* @param {string} reserve_in
* @param {string} reserve_out
* @param {string} amount_in
* @param {string} amplification
* @param {string} precision
* @returns {string}
*/
export function calculate_out_given_in(reserve_in: string, reserve_out: string, amount_in: string, amplification: string, precision: string): string;
/**
* @param {string} reserve_in
* @param {string} reserve_out
* @param {string} amount_out
* @param {string} amplification
* @param {string} precision
* @returns {string}
*/
export function calculate_in_given_out(reserve_in: string, reserve_out: string, amount_out: string, amplification: string, precision: string): string;
