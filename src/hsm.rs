use crate::error;
use crate::utils::parse_ratio;
use sp_arithmetic::Permill;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn calculate_hollar_out_given_collateral_in(
    amount_in: String,
    collateral_peg: String,
    purchase_fee: String,
) -> String {
    let amount_in = parse_into2!(u128, amount_in, error());
    let purchase_fee = Permill::from_float(parse_into2!(f64, purchase_fee, error()));
    let peg = parse_ratio(&collateral_peg).unwrap();
    let purchase_price = hydra_dx_math::hsm::calculate_purchase_price(peg, purchase_fee);
    let hollar_out = hydra_dx_math::hsm::calculate_hollar_amount(amount_in, purchase_price);

    if let Some(hollar_out) = hollar_out {
        hollar_out.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_collateral_in_given_hollar_out(
    amount_out: String,
    collateral_peg: String,
    purchase_fee: String,
) -> String {
    let amount_out = parse_into2!(u128, amount_out, error());
    let purchase_fee = Permill::from_float(parse_into2!(f64, purchase_fee, error()));
    let peg = parse_ratio(&collateral_peg).unwrap();

    let purchase_price = hydra_dx_math::hsm::calculate_purchase_price(peg, purchase_fee);
    let collateral_in = hydra_dx_math::hsm::calculate_collateral_amount(amount_out, purchase_price);

    if let Some(amount_in) = collateral_in {
        amount_in.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_collateral_out_given_hollar_in(
    amount_in: String,
    execution_price: String,
    buyback_fee: String,
) -> String {
    let _amount_in = parse_into2!(u128, amount_in, error());
    let exec_price = parse_ratio(&execution_price).unwrap();
    let buyback_fee = Permill::from_float(parse_into2!(f64, buyback_fee, error()));

    let buy_price = hydra_dx_math::hsm::calculate_buy_price_with_fee(exec_price, buyback_fee).unwrap();
    let collateral_out = hydra_dx_math::hsm::calculate_collateral_amount(exec_price.1, buy_price);

    if let Some(amount_out) = collateral_out {
        amount_out.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_hollar_in_given_collateral_out(
    amount_out: String,
    execution_price: String,
    buyback_fee: String,
) -> String {
    let _amount_out = parse_into2!(u128, amount_out, error());
    let exec_price = parse_ratio(&execution_price).unwrap();
    let buyback_fee = Permill::from_float(parse_into2!(f64, buyback_fee, error()));

    let buy_price = hydra_dx_math::hsm::calculate_buy_price_with_fee(exec_price, buyback_fee).unwrap();
    let hollar_in = hydra_dx_math::hsm::calculate_hollar_amount(exec_price.0, buy_price);

    if let Some(amount_in) = hollar_in {
        amount_in.to_string()
    } else {
        error()
    }
}
