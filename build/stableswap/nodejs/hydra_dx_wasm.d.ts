/* tslint:disable */
/* eslint-disable */
/**
* @param {string} _reserve_in
* @param {string} _reserve_out
* @param {string} _amount
* @returns {string}
*/
export function get_spot_price(_reserve_in: string, _reserve_out: string, _amount: string): string;
/**
* @param {string} _reserve_in
* @param {string} _reserve_out
* @param {string} _amount_in
* @param {string} _amplification
* @param {string} _precision
* @returns {string}
*/
export function calculate_out_given_in(_reserve_in: string, _reserve_out: string, _amount_in: string, _amplification: string, _precision: string): string;
/**
* @param {string} _reserve_in
* @param {string} _reserve_out
* @param {string} _amount_out
* @param {string} _amplification
* @param {string} _precision
* @returns {string}
*/
export function calculate_in_given_out(_reserve_in: string, _reserve_out: string, _amount_out: string, _amplification: string, _precision: string): string;
/**
* @param {string} a
* @param {number} fee_numerator
* @param {number} fee_denominator
* @returns {string}
*/
export function calculate_pool_trade_fee(a: string, fee_numerator: number, fee_denominator: number): string;
