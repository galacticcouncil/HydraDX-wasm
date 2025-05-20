pub use super::*;
use num_traits::Zero;
use sp_arithmetic::FixedU128;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_spot_price(s: String, b: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

    let result = hydra_dx_math::xyk::calculate_spot_price(sell_reserve, buy_reserve, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_spot_price(s: String, b: String) -> String {
    let (sell_reserve, buy_reserve) = to_u128!(s, b);

    let result = hydra_dx_math::xyk::calculate_spot_price_with_fee(sell_reserve, buy_reserve, None);

    result.unwrap_or(FixedU128::zero()).to_string()
}

#[wasm_bindgen]
pub fn calculate_spot_price_with_fee(s: String, b: String, fee_rate_n: String, fee_rate_d: String) -> String {
    let (sell_reserve, buy_reserve) = to_u128!(s, b);

    let (fee_rate_n, fee_rate_d) = to_u32!(fee_rate_n, fee_rate_d);

    let result =
        hydra_dx_math::xyk::calculate_spot_price_with_fee(sell_reserve, buy_reserve, Some((fee_rate_n, fee_rate_d)));

    result.unwrap_or(FixedU128::zero()).to_string()
}

#[wasm_bindgen]
pub fn calculate_out_given_in(s: String, b: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

    let result = hydra_dx_math::xyk::calculate_out_given_in(sell_reserve, buy_reserve, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_in_given_out(s: String, b: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

    let result = hydra_dx_math::xyk::calculate_in_given_out(buy_reserve, sell_reserve, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_liquidity_in(reserve_a: String, reserve_b: String, amount_a: String) -> String {
    let (reserve_a, reserve_b, amount_a) = to_u128!(reserve_a, reserve_b, amount_a);

    let result = hydra_dx_math::xyk::calculate_liquidity_in(reserve_a, reserve_b, amount_a);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_shares(reserve_a: String, amount_a: String, total_shares: String) -> String {
    let (reserve_a, amount_a, total_shares) = to_u128!(reserve_a, amount_a, total_shares);

    let result = hydra_dx_math::xyk::calculate_shares(reserve_a, amount_a, total_shares);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_liquidity_out_asset_a(
    reserve_a: String,
    reserve_b: String,
    shares: String,
    total_shares: String,
) -> String {
    let (reserve_a, reserve_b, shares, total_shares) = to_u128!(reserve_a, reserve_b, shares, total_shares);

    let result = hydra_dx_math::xyk::calculate_liquidity_out(reserve_a, reserve_b, shares, total_shares).ok();

    if let Some(values) = result {
        values.0.to_string()
    } else {
        "0".to_string()
    }
}

#[wasm_bindgen]
pub fn calculate_liquidity_out_asset_b(
    reserve_a: String,
    reserve_b: String,
    shares: String,
    total_shares: String,
) -> String {
    let (reserve_a, reserve_b, shares, total_shares) = to_u128!(reserve_a, reserve_b, shares, total_shares);

    let result = hydra_dx_math::xyk::calculate_liquidity_out(reserve_a, reserve_b, shares, total_shares).ok();

    if let Some(values) = result {
        values.1.to_string()
    } else {
        "0".to_string()
    }
}

#[test]
fn spot_price_works() {
    assert_eq!(
        xyk::get_spot_price(String::from("1000"), String::from("2000"), String::from("500")),
        "1000"
    );
    assert_eq!(
        xyk::get_spot_price(String::from("1000"), String::from("0"), String::from("500")),
        "0"
    );
}

#[test]
fn calculate_spot_price_works() {
    assert_eq!(
        xyk::calculate_spot_price(String::from("1000"), String::from("2000")),
        "2000000000000000000"
    );
    assert_eq!(xyk::calculate_spot_price(String::from("1000"), String::from("0")), "0");
}

#[test]
fn calculate_spot_price_with_fee_works() {
    assert_eq!(
        xyk::calculate_spot_price_with_fee(
            String::from("1000"),
            String::from("2000"),
            String::from("3"),
            String::from("1000")
        ),
        "1994000000000000000"
    );
    assert_eq!(
        xyk::calculate_spot_price_with_fee(
            String::from("1000"),
            String::from("2000"),
            String::from("0"),
            String::from("0")
        ),
        "0"
    );

    assert_eq!(
        xyk::calculate_spot_price_with_fee(
            String::from("1000"),
            String::from("0"),
            String::from("3"),
            String::from("1000")
        ),
        "0"
    );
}

#[test]
fn calculate_spot_price_has_better_precision() {
    assert_ne!(
        xyk::calculate_spot_price(String::from("5039030951140853000"), String::from("6987280000000000")),
        "0"
    );
    assert_eq!(
        xyk::get_spot_price(
            String::from("5039030951140853000"),
            String::from("6987280000000000"),
            String::from("100")
        ),
        "0"
    );
}

#[test]
fn out_in_works() {
    assert_eq!(
        xyk::calculate_out_given_in(String::from("1000"), String::from("2000"), String::from("500")),
        "666"
    );
    assert_eq!(
        xyk::calculate_out_given_in(String::from("1"), String::from("0"), String::from("0")),
        "0"
    );
}

#[test]
fn in_out_works() {
    assert_eq!(
        xyk::calculate_in_given_out(String::from("1000"), String::from("2000"), String::from("500")),
        "334"
    );
    assert_eq!(
        xyk::calculate_in_given_out(String::from("0"), String::from("1"), String::from("0")),
        "0"
    );
}

#[test]
fn add_liquidity_works() {
    assert_eq!(
        xyk::calculate_liquidity_in(String::from("1000"), String::from("2000"), String::from("500")),
        "1001"
    );
    assert_eq!(
        xyk::calculate_liquidity_in(String::from("0"), String::from("1"), String::from("0")),
        "0"
    );
}

#[test]
fn remove_liquidity_works() {
    assert_eq!(
        xyk::calculate_liquidity_out_asset_a(
            String::from("1000"),
            String::from("2000"),
            String::from("500"),
            String::from("1000"),
        ),
        "500"
    );
    assert_eq!(
        xyk::calculate_liquidity_out_asset_b(
            String::from("1000"),
            String::from("2000"),
            String::from("500"),
            String::from("1000"),
        ),
        "1000"
    );

    assert_eq!(
        xyk::calculate_liquidity_out_asset_a(
            String::from("0"),
            String::from("1"),
            String::from("0"),
            String::from("0"),
        ),
        "0"
    );
}

#[test]
fn share_calc_works() {
    assert_eq!(
        xyk::calculate_shares(String::from("1000"), String::from("100"), String::from("2000")),
        "200"
    );
    assert_eq!(
        xyk::calculate_liquidity_in(String::from("0"), String::from("1"), String::from("0")),
        "0"
    );
}
