use crate::error;
use sp_arithmetic::Permill;
use wasm_bindgen::prelude::wasm_bindgen;

fn parse_peg(data: String) -> Option<(u128, u128)> {
    let peg: (String, String) = serde_json::from_str(&data).ok()?;
    let n = peg.0.parse::<u128>().ok()?;
    let d = peg.1.parse::<u128>().ok()?;
    Some((n, d))
}

#[wasm_bindgen]
pub fn calculate_hollar_out_given_collateral_in(
    amount_in: String,
    collateral_peg: String,
    purchase_fee: String,
) -> String {
    let amount_in = parse_into2!(u128, amount_in, error());
    let purchase_fee = Permill::from_float(parse_into2!(f64, purchase_fee, error()));
    let peg = parse_peg(collateral_peg).unwrap();
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
    let peg = parse_peg(collateral_peg).unwrap();

    let purchase_price = hydra_dx_math::hsm::calculate_purchase_price(peg, purchase_fee);
    let collateral_in = hydra_dx_math::hsm::calculate_collateral_amount(amount_out, purchase_price);

    if let Some(amount_in) = collateral_in {
        amount_in.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_collateral_out_given_hollar_in(amount_in: String, amount_out: String, buyback_fee: String) -> String {
    let amount_in = parse_into2!(u128, amount_in, error());
    let amount_out = parse_into2!(u128, amount_out, error());
    let exec_price = (amount_out, amount_in);
    let buyback_fee = Permill::from_float(parse_into2!(f64, buyback_fee, error()));

    let buy_price = hydra_dx_math::hsm::calculate_buy_price_with_fee(exec_price, buyback_fee).unwrap();
    let collateral_out = hydra_dx_math::hsm::calculate_collateral_amount(amount_in, buy_price);

    if let Some(amount_out) = collateral_out {
        amount_out.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_hollar_in_given_collateral_out(amount_out: String, amount_in: String, buyback_fee: String) -> String {
    let amount_out = parse_into2!(u128, amount_out, error());
    let amount_in = parse_into2!(u128, amount_in, error());
    let exec_price = (amount_out, amount_in);
    let buyback_fee = Permill::from_float(parse_into2!(f64, buyback_fee, error()));

    let buy_price = hydra_dx_math::hsm::calculate_buy_price_with_fee(exec_price, buyback_fee).unwrap();
    let hollar_in = hydra_dx_math::hsm::calculate_hollar_amount(amount_out, buy_price);

    if let Some(amount_in) = hollar_in {
        amount_in.to_string()
    } else {
        error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_hollar_out_given_collateral_in_should_work() {
        let peg = serde_json::to_string(&("1000".to_string(), "1000".to_string())).unwrap();
        let result = calculate_hollar_out_given_collateral_in("1000".to_string(), peg, "0.05".to_string());
        assert_ne!(result, "-1");
        let r = result.parse::<u128>().expect("failed to parse to u128");
        assert_eq!(r, 952);
    }
    #[test]
    fn calculate_collateral_out_given_hollar_in_should_work() {
        let result =
            calculate_collateral_out_given_hollar_in("1000".to_string(), "1000".to_string(), "0.03".to_string());
        assert_ne!(result, "-1");
        let r = result.parse::<u128>().expect("failed to parse to u128");
        assert_eq!(r, 1031);
    }

    #[test]
    fn calculate_hollar_in_given_collateral_out_should_work() {
        let result = calculate_hollar_in_given_collateral_out("970".to_string(), "970".to_string(), "0.03".to_string());
        assert_ne!(result, "-1");
        let r = result.parse::<u128>().expect("failed to parse to u128");
        assert_eq!(r, 940);
    }

    #[test]
    fn calculate_collateral_in_given_hollar_out_should_work() {
        let peg = serde_json::to_string(&("1000".to_string(), "1000".to_string())).unwrap();
        let result = calculate_collateral_in_given_hollar_out("950".to_string(), peg, "0.05".to_string());
        assert_ne!(result, "-1");
        let r = result.parse::<u128>().expect("Should be parsed");
        assert_eq!(r, 998);
    }
}
