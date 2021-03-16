use hydra_dx_math::calculate_buy_price;
use hydra_dx_math::calculate_sell_price;
use hydra_dx_math::calculate_spot_price;

use wasm_bindgen::prelude::*;

fn convert_to_u128(s: &str) -> u128 {
    match s.parse::<u128>() {
        Ok(v) => v,
        Err(_) => 0,
    }
}

#[wasm_bindgen]
pub fn get_spot_price(s: String, b: String, a: String) -> String {
    let sell_reserve = convert_to_u128(&s);
    let buy_reserve = convert_to_u128(&b);
    let amount = convert_to_u128(&a);

    let result = calculate_spot_price(sell_reserve, buy_reserve, amount);

    match result {
        Some(val) => val.to_string().chars().collect::<String>(),
        None => "0".chars().collect::<String>(),
    }
}

#[wasm_bindgen]
pub fn get_sell_price(s: String, b: String, a: String) -> String {
    let sell_reserve = convert_to_u128(&s);
    let buy_reserve = convert_to_u128(&b);
    let amount = convert_to_u128(&a);

    let result = calculate_sell_price(sell_reserve, buy_reserve, amount);

    match result {
        Some(val) => val.to_string().chars().collect::<String>(),
        None => "0".chars().collect::<String>(),
    }
}

#[wasm_bindgen]
pub fn get_buy_price(s: String, b: String, a: String) -> String {
    let sell_reserve = convert_to_u128(&s);
    let buy_reserve = convert_to_u128(&b);
    let amount = convert_to_u128(&a);

    let result = calculate_buy_price(sell_reserve, buy_reserve, amount);

    match result {
        Some(val) => val.to_string().chars().collect::<String>(),
        None => "0".chars().collect::<String>(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert_to_u128, get_buy_price, get_sell_price, get_spot_price};

    #[test]
    fn conversion() {
        assert_eq!(convert_to_u128("128"), 128_u128);
        assert_eq!(convert_to_u128("invalid"), 0_u128);
    }

    #[test]
    fn spot_price_works() {
        assert_eq!(
            get_spot_price(String::from("1000"), String::from("2000"), String::from("500")),
            "1000"
        );
        assert_eq!(
            get_spot_price(String::from("1000"), String::from("0"), String::from("500")),
            "0"
        );
    }

    #[test]
    fn sell_price_works() {
        assert_eq!(
            get_sell_price(String::from("1000"), String::from("2000"), String::from("500")),
            "667"
        );
        assert_eq!(
            get_sell_price(String::from("1"), String::from("0"), String::from("0")),
            "1"
        );
    }

    #[test]
    fn buy_price_works() {
        assert_eq!(
            get_buy_price(String::from("1000"), String::from("2000"), String::from("500")),
            "334"
        );
        assert_eq!(
            get_buy_price(String::from("0"), String::from("1"), String::from("0")),
            "1"
        );
    }
}
