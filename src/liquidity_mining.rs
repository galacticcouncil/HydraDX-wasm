use super::*;
use wasm_bindgen::prelude::*;

use sp_arithmetic::{fixed_point::FixedU128, FixedPointNumber};

macro_rules! parse_into {
    ($x:ty, $y:expr) => {{
        let r = if let Some(x) = $y.parse::<$x>().ok() {
            x
        } else {
            return error();
        };
        r
    }};
}

#[wasm_bindgen]
pub fn fixed_from_rational(a: String, b: String) -> String {
    match FixedU128::checked_from_rational(parse_into!(u128, a), parse_into!(u128, b)) {
        Some(v) => v.to_string(),
        None => error(),
    }
}

#[wasm_bindgen]
pub fn calculate_loyalty_multiplier(period: String, initial_reward_percentage: String, scale_coef: String) -> String {
    let period = parse_into!(u128, period);
    let reward_percentage = FixedU128::from_inner(parse_into!(u128, initial_reward_percentage));
    let scale_coef = parse_into!(u32, scale_coef);

    let result =
        hydra_dx_math::liquidity_mining::calculate_loyalty_multiplier::<u128>(period, reward_percentage, scale_coef);

    if let Some(r) = result.ok() {
        r.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_accumulated_rps(accumulated_rps_now: String, total_shares: String, reward: String) -> String {
    let rps = FixedU128::from_inner(parse_into!(u128, accumulated_rps_now));
    let shares = parse_into!(u128, total_shares);
    let reward = parse_into!(u128, reward);

    let result = hydra_dx_math::liquidity_mining::calculate_accumulated_rps(rps, shares, reward);

    if let Some(r) = result.ok() {
        r.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_user_reward(
    accumulated_rpvs: String,
    valued_shares: String,
    accumulated_claimed_rewards: String,
    accumulated_rpvs_now: String,
    loyalty_multiplier: String,
) -> String {
    let rps = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs));
    let shares = parse_into!(u128, valued_shares);
    let rewards = parse_into!(u128, accumulated_claimed_rewards);

    let rps_now = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs_now));
    let multiplier = FixedU128::from_inner(parse_into!(u128, loyalty_multiplier));

    let result = hydra_dx_math::liquidity_mining::calculate_user_reward(rps, shares, rewards, rps_now, multiplier);

    if let Some(r) = result.ok() {
        r.0.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_user_unclaimed_reward(
    accumulated_rpvs: String,
    valued_shares: String,
    accumulated_claimed_rewards: String,
    accumulated_rpvs_now: String,
    loyalty_multiplier: String,
) -> String {
    let rps = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs));
    let shares = parse_into!(u128, valued_shares);
    let rewards = parse_into!(u128, accumulated_claimed_rewards);

    let rps_now = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs_now));
    let multiplier = FixedU128::from_inner(parse_into!(u128, loyalty_multiplier));

    let result = hydra_dx_math::liquidity_mining::calculate_user_reward(rps, shares, rewards, rps_now, multiplier);

    if let Some(r) = result.ok() {
        r.1.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_valued_shares(shares: String, incentivized_asset_balance: String) -> String {
    let shares = parse_into!(u128, shares);
    let balance = parse_into!(u128, incentivized_asset_balance);

    let result = hydra_dx_math::liquidity_mining::calculate_valued_shares(shares, balance);

    if let Some(r) = result.ok() {
        r.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_reward(accumulated_rps_start: String, accumulated_rps_now: String, shares: String) -> String {
    let rps_start = FixedU128::from_inner(parse_into!(u128, accumulated_rps_start));
    let rps_now = FixedU128::from_inner(parse_into!(u128, accumulated_rps_now));

    let shares = parse_into!(u128, shares);

    let result = hydra_dx_math::liquidity_mining::calculate_reward(rps_start, rps_now, shares);

    if let Some(r) = result.ok() {
        r.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_global_farm_shares(valued_shares: String, multiplier: String) -> String {
    let s = parse_into!(u128, valued_shares);
    let m = FixedU128::from_inner(parse_into!(u128, multiplier));

    let result = hydra_dx_math::liquidity_mining::calculate_global_farm_shares(s, m);

    if let Some(r) = result.ok() {
        r.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_yield_farm_rewards(
    yield_farm_rpz: String,
    global_farm_rpz: String,
    multiplier: String,
    total_valued_shares: String,
) -> String {
    let y_rpz = FixedU128::from_inner(parse_into!(u128, yield_farm_rpz));
    let g_rpz = FixedU128::from_inner(parse_into!(u128, global_farm_rpz));
    let m = FixedU128::from_inner(parse_into!(u128, multiplier));
    let vs = parse_into!(u128, total_valued_shares);

    let result = hydra_dx_math::liquidity_mining::calculate_yield_farm_rewards(y_rpz, g_rpz, m, vs);

    if let Some(v) = result.ok() {
        v.1.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_yield_farm_delta_rpvs(
    yield_farm_rpz: String,
    global_farm_rpz: String,
    multiplier: String,
    total_valued_shares: String,
) -> String {
    let y_rpz = FixedU128::from_inner(parse_into!(u128, yield_farm_rpz));
    let g_rpz = FixedU128::from_inner(parse_into!(u128, global_farm_rpz));
    let m = FixedU128::from_inner(parse_into!(u128, multiplier));
    let vs = parse_into!(u128, total_valued_shares);

    let result = hydra_dx_math::liquidity_mining::calculate_yield_farm_rewards(y_rpz, g_rpz, m, vs);

    if let Some(v) = result.ok() {
        v.0.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_global_farm_rewards(
    total_shares_z: String,
    price_adjustment: String,
    yield_per_period: String,
    max_reward_per_period: String,
    periods_since_last_update: String,
) -> String {
    let ts = parse_into!(u128, total_shares_z);
    let p_adj = FixedU128::from_inner(parse_into!(u128, price_adjustment));
    let ypp = FixedU128::from_inner(parse_into!(u128, yield_per_period));
    let max_rew_per_period = parse_into!(u128, max_reward_per_period);
    let periods = parse_into!(u128, periods_since_last_update);

    let result =
        hydra_dx_math::liquidity_mining::calculate_global_farm_rewards(ts, p_adj, ypp, max_rew_per_period, periods);

    if let Some(v) = result.ok() {
        v.to_string()
    } else {
        error()
    }
}

#[test]
fn calculate_global_farm_rewards_should_work_when_input_is_correct() {
    assert_eq!(
        calculate_global_farm_rewards(
            "17989865464312".to_string(),
            "1000000000000000000".to_string(),
            "138571428600000000".to_string(),
            "59898".to_string(),
            "1".to_string()
        ),
        "59898"
    );

    assert_eq!(
        calculate_global_farm_rewards(
            "35189".to_string(),
            "1000000000000000000".to_string(),
            "133333333300000000".to_string(),
            "468787897".to_string(),
            "10".to_string()
        ),
        "46918"
    );
}

#[test]
fn calculate_yield_farm_rewards_should_work_when_input_is_correct() {
    assert_eq!(
        calculate_yield_farm_rewards(
            "82000000000000000000".to_string(),
            "357000000000000000000".to_string(),
            "1000000000000000000".to_string(),
            "932564".to_string()
        ),
        "256455100"
    );

    assert_eq!(
        calculate_yield_farm_rewards(
            "2491000000000000000000".to_string(),
            "2537000000000000000000".to_string(),
            "1000000000000000000".to_string(),
            "85100506".to_string()
        ),
        "3914623276"
    );
}

#[test]
fn calculate_yield_farm_delta_rpvs_should_work_when_input_is_correct() {
    assert_eq!(
        calculate_yield_farm_delta_rpvs(
            "82000000000000000000".to_string(),
            "357000000000000000000".to_string(),
            "1000000000000000000".to_string(),
            "932564".to_string()
        ),
        "275000000000000000000"
    );

    assert_eq!(
        calculate_yield_farm_delta_rpvs(
            "2491000000000000000000".to_string(),
            "2537000000000000000000".to_string(),
            "1000000000000000000".to_string(),
            "85100506".to_string()
        ),
        "46000000000000000000"
    );
}

#[test]
fn calculate_loyalty_multiplier_should_work_when_input_is_correct() {
    assert_eq!(
        calculate_loyalty_multiplier("100".to_string(), "200000000000000000".to_string(), "2".to_string()),
        "984313725490196078"
    );
    assert_eq!(
        calculate_loyalty_multiplier("100".to_string(), "1000000000000000000".to_string(), "2".to_string()),
        "1000000000000000000"
    );
}

#[test]
fn calculate_loyalty_multiplier_should_fail_when_input_is_incorrect() {
    assert_eq!(
        calculate_loyalty_multiplier("invalid".to_string(), "200000000000000000".to_string(), "2".to_string()),
        "-1"
    );
    assert_eq!(
        calculate_loyalty_multiplier("100".to_string(), "invalid".to_string(), "2".to_string()),
        "-1"
    );
    assert_eq!(
        calculate_loyalty_multiplier(
            "100".to_string(),
            "100000000000000000".to_string(),
            "invalid".to_string(),
        ),
        "-1"
    );
}

#[test]
fn fixed_from_rational_should_work() {
    assert_eq!(
        fixed_from_rational("1".to_string(), "5".to_string()),
        "200000000000000000"
    );

    assert_eq!(fixed_from_rational("1".to_string(), "0".to_string()), "-1");

    assert_eq!(
        fixed_from_rational("5343".to_string(), "5".to_string()),
        "1068600000000000000000"
    );

    assert_eq!(
        fixed_from_rational("93846346337460743".to_string(), "100000000".to_string()),
        "938463463374607430000000000"
    );

    assert_eq!(
        fixed_from_rational("340282366920938463463374607431768211455".to_string(), "10".to_string()),
        "-1"
    );
}
