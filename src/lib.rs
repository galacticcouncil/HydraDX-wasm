use wasm_bindgen::prelude::*;

macro_rules! to_u128 {
    ($($x:expr),+) => (
        {($($x.parse::<u128>().unwrap_or(0)),+)}
    );
}

#[wasm_bindgen]
pub fn get_spot_price(s: String, b: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

    let result = hydra_dx_math::calculate_spot_price(sell_reserve, buy_reserve, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_out_given_in(s: String, b: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

    let result = hydra_dx_math::calculate_out_given_in(sell_reserve, buy_reserve, amount);

    result.unwrap_or(0).to_string()
}

#[wasm_bindgen]
pub fn calculate_in_given_out(s: String, b: String, a: String) -> String {
    let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

    let result = hydra_dx_math::calculate_in_given_out(buy_reserve, sell_reserve, amount);

    result.unwrap_or(0).to_string()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_in_given_out, calculate_out_given_in, get_spot_price};

    #[test]
    fn conversion() {
        assert_eq!(to_u128!("128"), 128_u128);
        assert_eq!(to_u128!("invalid"), 0_u128);
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
    fn outin_price_works() {
        assert_eq!(
            calculate_out_given_in(String::from("1000"), String::from("2000"), String::from("500")),
            "667"
        );
        assert_eq!(
            calculate_out_given_in(String::from("1"), String::from("0"), String::from("0")),
            "1"
        );
    }

    #[test]
    fn inout_price_works() {
        assert_eq!(
            calculate_in_given_out(String::from("1000"), String::from("2000"), String::from("500")),
            "334"
        );
        assert_eq!(
            calculate_in_given_out(String::from("0"), String::from("1"), String::from("0")),
            "1"
        );
    }
}
