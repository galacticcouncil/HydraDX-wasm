use wasm_bindgen::prelude::*;

macro_rules! to_u128 {
    ($($x:expr),+) => (
        {($($x.parse::<u128>().unwrap_or(0)),+)}
    );
}

pub mod xyk {
    pub use super::*;

    #[cfg(feature = "xyk")]
    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::calculate_spot_price(sell_reserve, buy_reserve, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "xyk")]
    #[wasm_bindgen]
    pub fn calculate_out_given_in(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::calculate_out_given_in(sell_reserve, buy_reserve, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "xyk")]
    #[wasm_bindgen]
    pub fn calculate_in_given_out(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::calculate_in_given_out(buy_reserve, sell_reserve, amount);

        result.unwrap_or(0).to_string()
    }
}

pub mod lbp {
    pub use super::*;

   #[cfg(feature = "lbp")]
    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, sell_weight, buy_weight, amount) = to_u128!(s, b, s_w, b_w, a);

        let result = hydra_dx_math::lbp::calculate_spot_price(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "lbp")]
    #[wasm_bindgen]
    pub fn calculate_out_given_in(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, sell_weight, buy_weight, amount) = to_u128!(s, b, s_w, b_w, a);

        let result = hydra_dx_math::lbp::calculate_out_given_in(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "lbp")]
    #[wasm_bindgen]
    pub fn calculate_in_given_out(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, sell_weight, buy_weight, amount) = to_u128!(s, b, s_w, b_w, a);

        let result = hydra_dx_math::lbp::calculate_in_given_out(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }
}


#[cfg(test)]
mod tests {
    #[cfg(feature = "xyk")]
    use crate::xyk;

    #[cfg(feature = "lbp")]
    use crate::lbp;

    #[test]
    fn conversion() {
        assert_eq!(to_u128!("128"), 128_u128);
        assert_eq!(to_u128!("invalid"), 0_u128);
    }

    #[cfg(feature = "xyk")]
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

    #[cfg(feature = "xyk")]
    #[test]
    fn outin_price_works() {
        assert_eq!(
            xyk::calculate_out_given_in(String::from("1000"), String::from("2000"), String::from("500")),
            "667"
        );
        assert_eq!(
            xyk::calculate_out_given_in(String::from("1"), String::from("0"), String::from("0")),
            "1"
        );
    }

    #[cfg(feature = "xyk")]
    #[test]
    fn inout_price_works() {
        assert_eq!(
            xyk::calculate_in_given_out(String::from("1000"), String::from("2000"), String::from("500")),
            "334"
        );
        assert_eq!(
            xyk::calculate_in_given_out(String::from("0"), String::from("1"), String::from("0")),
            "1"
        );
    }

    #[cfg(feature = "lbp")]
    #[test]
    fn spot_price_works() {
        assert_eq!(
            lbp::get_spot_price(String::from("1000"), String::from("2000"), String::from("1000"), String::from("2000"), String::from("500")),
            "500"
        );
        assert_eq!(
            lbp::get_spot_price(String::from("1000"), String::from("0"), String::from("1000"), String::from("2000"), String::from("500")),
            "0"
        );
    }

    #[cfg(feature = "lbp")]
    #[test]
    fn outin_price_works() {
        assert_eq!(
            lbp::calculate_out_given_in(String::from("1000"), String::from("2000"), String::from("1000"), String::from("2000"), String::from("500")),
            "367"
        );
        assert_eq!(
            lbp::calculate_out_given_in(String::from("1"), String::from("0"), String::from("1000"), String::from("2000"), String::from("0")),
            "0"
        );
    }

    #[cfg(feature = "lbp")]
    #[test]
    fn inout_price_works() {
        assert_eq!(
            lbp::calculate_in_given_out(String::from("1000"), String::from("2000"), String::from("1000"), String::from("2000"), String::from("500")),
            "778"
        );
        assert_eq!(
            lbp::calculate_in_given_out(String::from("0"), String::from("1"), String::from("1000"), String::from("2000"), String::from("0")),
            "1"
        );
    }
}
