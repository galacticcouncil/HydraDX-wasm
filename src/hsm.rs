use crate::error;
use sp_arithmetic::{FixedU128, Perbill, Permill};
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

#[wasm_bindgen]
pub fn calculate_imbalance(hollar_reserve: String, collateral_peg: String, collateral_reserve: String) -> String {
    let hollar_reserve = parse_into2!(u128, hollar_reserve, error());
    let collateral_reserve = parse_into2!(u128, collateral_reserve, error());
    let peg = parse_peg(collateral_peg).unwrap();

    let imbalance = hydra_dx_math::hsm::calculate_imbalance(hollar_reserve, peg, collateral_reserve);

    if let Some(imbalance) = imbalance {
        imbalance.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_buyback_limit(imbalance: String, b: String) -> String {
    let imbalance = parse_into2!(u128, imbalance, error());
    let b = Perbill::from_float(parse_into2!(f64, b, error()));

    let limit = hydra_dx_math::hsm::calculate_buyback_limit(imbalance, b);
    limit.to_string()
}

#[wasm_bindgen]
pub fn calculate_buyback_price_with_fee(
    execution_price_n: String,
    execution_price_d: String,
    buyback_fee: String,
) -> String {
    let execution_price_n = parse_into2!(u128, execution_price_n, error());
    let execution_price_d = parse_into2!(u128, execution_price_d, error());
    let execution_price = (execution_price_n, execution_price_d);
    let buyback_fee = Permill::from_float(parse_into2!(f64, buyback_fee, error()));

    let buy_price = hydra_dx_math::hsm::calculate_buy_price_with_fee(execution_price, buyback_fee);

    if let Some(price) = buy_price {
        serde_json::to_string(&(price.0.to_string(), price.1.to_string())).unwrap_or_else(|_| error())
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_max_price(collateral_peg: String, coefficient: String) -> String {
    let peg = parse_peg(collateral_peg).unwrap();
    let coefficient = FixedU128::from_float(parse_into2!(f64, coefficient, error()));

    let max_price = hydra_dx_math::hsm::calculate_max_buy_price(peg, coefficient);
    serde_json::to_string(&(max_price.0.to_string(), max_price.1.to_string())).unwrap_or_else(|_| error())
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

    #[test]
    fn calculate_imbalance_should_work() {
        let peg = serde_json::to_string(&("1000".to_string(), "1000".to_string())).unwrap();
        let result = calculate_imbalance("2000".to_string(), peg, "1000".to_string());
        assert_ne!(result, "-1");
        let r = result.parse::<u128>().expect("Should be parsed");
        assert_eq!(r, 500);
    }

    #[test]
    fn calculate_imbalance_zero_when_hollar_less_than_pegged() {
        let peg = serde_json::to_string(&("1000".to_string(), "1000".to_string())).unwrap();
        let result = calculate_imbalance("800".to_string(), peg, "1000".to_string());
        assert_ne!(result, "-1");
        let r = result.parse::<u128>().expect("Should be parsed");
        assert_eq!(r, 0);
    }

    #[test]
    fn calculate_buyback_limit_should_work() {
        let result = calculate_buyback_limit("1000".to_string(), "0.1".to_string());
        assert_ne!(result, "-1");
        let r = result.parse::<u128>().expect("Should be parsed");
        assert_eq!(r, 100);
    }

    #[test]
    fn calculate_buyback_price_with_fee_should_work() {
        let result = calculate_buyback_price_with_fee("1000".to_string(), "1000".to_string(), "0.03".to_string());
        assert_ne!(result, "-1");
        let parsed: (String, String) = serde_json::from_str(&result).expect("Should parse JSON");
        let n = parsed.0.parse::<u128>().expect("Should parse numerator");
        let d = parsed.1.parse::<u128>().expect("Should parse denominator");
        assert_eq!(n, 1000000000);
        assert_eq!(d, 970000000);
    }

    #[test]
    fn calculate_max_price_should_work() {
        let peg = serde_json::to_string(&("1000".to_string(), "1000".to_string())).unwrap();
        let result = calculate_max_price(peg, "1.5".to_string());
        assert_ne!(result, "-1");
        let parsed: (String, String) = serde_json::from_str(&result).expect("Should parse JSON");
        let n = parsed.0.parse::<u128>().expect("Should parse numerator");
        let d = parsed.1.parse::<u128>().expect("Should parse denominator");
        assert_eq!(n, 1500000000000000000000);
        assert_eq!(d, 1000000000000000000000);
    }
}
