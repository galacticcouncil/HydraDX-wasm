
use wasm_bindgen::prelude::wasm_bindgen;
use crate::error;

#[wasm_bindgen]
pub fn calculate_hollar_out_given_collateral_in(
    amount_in: String,
    collateral_peg: String,
    purchase_fee: String,
) -> String {
    let amount_in = parse_into2!(u128, amount_in, error());
    let purchase_fee = parse_into2!(f64, purchase_fee, error());
    let peg = parse_into2!(f64, collateral_peg, error());
    "-1".to_string()
}

#[wasm_bindgen]
pub fn calculate_collateral_in_given_hollar_out(
    amount_out: String,
    collateral_peg: String,
    purchase_fee: String,
) -> String {
    let amount_out = parse_into2!(u128, amount_out, error());
    let purchase_fee = parse_into2!(f64, purchase_fee, error());
    let peg = parse_into2!(f64, collateral_peg, error());
    "-1".to_string()
}

#[wasm_bindgen]
pub fn calculate_collateral_out_given_hollar_in(
    amount_in: String,
    collateral_peg: String,
    execution_price: String,
    buyback_fee: String,
) -> String {
    let amount_in = parse_into2!(u128, amount_in, error());
    let exec_price = parse_into2!(f64, execution_price, error());
    let buyback_fee = parse_into2!(f64, buyback_fee, error());
    let peg = parse_into2!(f64, collateral_peg, error());
    "-1".to_string()
}

#[wasm_bindgen]
pub fn calculate_hollar_in_given_collateral_out(
    amount_out: String,
    collateral_peg: String,
    execution_price: String,
    buyback_fee: String,
) -> String {
    let amount_out = parse_into2!(u128, amount_out, error());
    let exec_price = parse_into2!(f64, execution_price, error());
    let buyback_fee = parse_into2!(f64, buyback_fee, error());
    let peg = parse_into2!(f64, collateral_peg, error());
    "-1".to_string()
}
