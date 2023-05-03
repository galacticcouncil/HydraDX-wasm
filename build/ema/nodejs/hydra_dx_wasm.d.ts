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
* Calculate the iterated exponential moving average for the given prices.
* `iterations` is the number of iterations of the EMA to calculate.
* `prev` is the previous oracle value, `incoming` is the new value to integrate.
* `smoothing` is the smoothing factor of the EMA.
* @param {string} iterations
* @param {string} prev_n
* @param {string} prev_d
* @param {string} incoming_n
* @param {string} incoming_d
* @param {string} smoothing
* @returns {string}
*/
export function iterated_price_ema(iterations: string, prev_n: string, prev_d: string, incoming_n: string, incoming_d: string, smoothing: string): string;
/**
* Calculate the iterated exponential moving average for the given balances.
* `iterations` is the number of iterations of the EMA to calculate.
* `prev` is the previous oracle value, `incoming` is the new value to integrate.
* `smoothing` is the smoothing factor of the EMA.
* @param {string} iterations
* @param {string} prev
* @param {string} incoming
* @param {string} smoothing
* @returns {string}
*/
export function iterated_balance_ema(iterations: string, prev: string, incoming: string, smoothing: string): string;
