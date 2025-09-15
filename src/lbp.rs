pub use super::*;
use num_traits::Zero;
use sp_arithmetic::FixedU128;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_spot_price(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
    let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

    let result = hydra_dx_math::lbp::calculate_spot_price(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_spot_price(s: String, b: String, s_w: String, b_w: String) -> String {
    let (sell_reserve, buy_reserve) = to_u128!(s, b);
    let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

    let result = hydra_dx_math::lbp::calculate_spot_price_with_fee(
        sell_reserve,
        buy_reserve,
        sell_weight,
        buy_weight,
        0,
        0,
        None,
    );

    result.unwrap_or(FixedU128::zero()).to_string()
}

#[wasm_bindgen]
pub fn calculate_spot_price_with_fee(
    s: String,
    b: String,
    s_w: String,
    b_w: String,
    fee_asset: String,
    asset_out: String,
    fee_rate_n: String,
    fee_rate_d: String,
) -> String {
    let (sell_reserve, buy_reserve) = to_u128!(s, b);
    let (sell_weight, buy_weight, fee_asset, asset_out, fee_rate_n, fee_rate_d) =
        to_u32!(s_w, b_w, fee_asset, asset_out, fee_rate_n, fee_rate_d);

    let result = hydra_dx_math::lbp::calculate_spot_price_with_fee(
        sell_reserve,
        buy_reserve,
        sell_weight,
        buy_weight,
        fee_asset,
        asset_out,
        Some((fee_rate_n, fee_rate_d)),
    );

    result.unwrap_or(FixedU128::zero()).to_string()
}

#[wasm_bindgen]
pub fn calculate_out_given_in(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
    let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

    let result = hydra_dx_math::lbp::calculate_out_given_in(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_in_given_out(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
    let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

    let result = hydra_dx_math::lbp::calculate_in_given_out(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_linear_weights(start_x: String, end_x: String, start_y: String, end_y: String, at: String) -> String {
    let (start_x, end_x, start_y, end_y, at) = to_u32!(start_x, end_x, start_y, end_y, at);

    let result = hydra_dx_math::lbp::calculate_linear_weights(start_x, end_x, start_y, end_y, at);

    result.unwrap_or(0).to_string()
}

#[test]
fn spot_price_works() {
    assert_eq!(
        lbp::get_spot_price(
            String::from("1000"),
            String::from("2000"),
            String::from("1000"),
            String::from("2000"),
            String::from("500"),
        ),
        "500"
    );
    assert_eq!(
        lbp::get_spot_price(
            String::from("1000"),
            String::from("0"),
            String::from("1000"),
            String::from("2000"),
            String::from("500"),
        ),
        "0"
    );
}

#[test]
fn calculate_spot_price_works() {
    assert_eq!(
        lbp::calculate_spot_price(
            String::from("1000"),
            String::from("2000"),
            String::from("1000"),
            String::from("2000"),
        ),
        "1000000000000000000"
    );

    assert_eq!(
        lbp::calculate_spot_price(
            String::from("1000"),
            String::from("2000"),
            String::from("0"),
            String::from("2000"),
        ),
        "0"
    );
}

#[test]
fn calculate_spot_price_with_fee_works() {
    assert_eq!(
        lbp::calculate_spot_price_with_fee(
            String::from("1000"),
            String::from("2000"),
            String::from("1000"),
            String::from("2000"),
            String::from("5"),
            String::from("5"),
            String::from("1"),
            String::from("100")
        ),
        "990000000000000000"
    );

    assert_eq!(
        lbp::calculate_spot_price_with_fee(
            String::from("1000"),
            String::from("2000"),
            String::from("1000"),
            String::from("2000"),
            String::from("1"),
            String::from("5"),
            String::from("1"),
            String::from("100")
        ),
        "1000000000000000000"
    );

    assert_eq!(
        lbp::calculate_spot_price_with_fee(
            String::from("1000"),
            String::from("2000"),
            String::from("1000"),
            String::from("0"),
            String::from("1"),
            String::from("5"),
            String::from("1"),
            String::from("100")
        ),
        "0"
    );
}

#[test]
fn outin_price_works() {
    assert_eq!(
        lbp::calculate_out_given_in(
            String::from("1000"),
            String::from("2000"),
            String::from("1000"),
            String::from("2000"),
            String::from("500"),
        ),
        "367"
    );
    assert_eq!(
        lbp::calculate_out_given_in(
            String::from("1"),
            String::from("0"),
            String::from("1000"),
            String::from("2000"),
            String::from("0"),
        ),
        "0"
    );
}

#[test]
fn inout_price_works() {
    assert_eq!(
        lbp::calculate_in_given_out(
            String::from("1000"),
            String::from("2000"),
            String::from("1000"),
            String::from("2000"),
            String::from("500"),
        ),
        "779"
    );
    assert_eq!(
        lbp::calculate_in_given_out(
            String::from("0"),
            String::from("1"),
            String::from("1000"),
            String::from("2000"),
            String::from("0"),
        ),
        "1"
    );
}

#[test]
fn linear_weights_works() {
    assert_eq!(
        lbp::calculate_linear_weights(
            String::from("100"),
            String::from("200"),
            String::from("1000"),
            String::from("2000"),
            String::from("170"),
        ),
        "1700"
    );
}
