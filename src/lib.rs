use wasm_bindgen::prelude::*;

macro_rules! to_u128 {
    ($($x:expr),+) => (
        {($($x.parse::<u128>().unwrap_or(0)),+)}
    );
}

macro_rules! to_u32 {
    ($($x:expr),+) => (
        {($($x.parse::<u32>().unwrap_or(0)),+)}
    );
}

pub mod xyk {
    pub use super::*;

    #[cfg(feature = "xyk")]
    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::xyk::calculate_spot_price(sell_reserve, buy_reserve, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "xyk")]
    #[wasm_bindgen]
    pub fn calculate_out_given_in(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::xyk::calculate_out_given_in(sell_reserve, buy_reserve, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "xyk")]
    #[wasm_bindgen]
    pub fn calculate_in_given_out(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::xyk::calculate_in_given_out(buy_reserve, sell_reserve, amount);

        result.unwrap_or(0).to_string()
    }
}

pub mod lbp {
    pub use super::*;

    #[cfg(feature = "lbp")]
    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::calculate_spot_price(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "lbp")]
    #[wasm_bindgen]
    pub fn calculate_out_given_in(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::calculate_out_given_in(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "lbp")]
    #[wasm_bindgen]
    pub fn calculate_in_given_out(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::calculate_in_given_out(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[cfg(feature = "lbp")]
    #[wasm_bindgen]
    pub fn calculate_linear_weights(
        start_x: String,
        end_x: String,
        start_y: String,
        end_y: String,
        at: String,
    ) -> String {
        let (start_x, end_x, start_y, end_y, at) = to_u32!(start_x, end_x, start_y, end_y, at);

        let result = hydra_dx_math::lbp::calculate_linear_weights(start_x, end_x, start_y, end_y, at);

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
            "666"
        );
        assert_eq!(
            xyk::calculate_out_given_in(String::from("1"), String::from("0"), String::from("0")),
            "0"
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
            "0"
        );
    }

    #[cfg(feature = "lbp")]
    #[test]
    fn spot_price_works() {
        assert_eq!(
            lbp::get_spot_price(
                String::from("1000"),
                String::from("2000"),
                String::from("1000"),
                String::from("2000"),
                String::from("500")
            ),
            "500"
        );
        assert_eq!(
            lbp::get_spot_price(
                String::from("1000"),
                String::from("0"),
                String::from("1000"),
                String::from("2000"),
                String::from("500")
            ),
            "0"
        );
    }

    #[cfg(feature = "lbp")]
    #[test]
    fn outin_price_works() {
        assert_eq!(
            lbp::calculate_out_given_in(
                String::from("1000"),
                String::from("2000"),
                String::from("1000"),
                String::from("2000"),
                String::from("500")
            ),
            "365"
        );
        assert_eq!(
            lbp::calculate_out_given_in(
                String::from("1"),
                String::from("0"),
                String::from("1000"),
                String::from("2000"),
                String::from("0")
            ),
            "0"
        );
    }

    #[cfg(feature = "lbp")]
    #[test]
    fn inout_price_works() {
        assert_eq!(
            lbp::calculate_in_given_out(
                String::from("1000"),
                String::from("2000"),
                String::from("1000"),
                String::from("2000"),
                String::from("500")
            ),
            "773"
        );
        assert_eq!(
            lbp::calculate_in_given_out(
                String::from("0"),
                String::from("1"),
                String::from("1000"),
                String::from("2000"),
                String::from("0")
            ),
            "0"
        );
    }

    #[cfg(feature = "lbp")]
    #[test]
    fn linear_weights_works() {
        assert_eq!(
            lbp::calculate_linear_weights(
                String::from("100"),
                String::from("200"),
                String::from("1000"),
                String::from("2000"),
                String::from("170")
            ),
            "1700"
        );
    }
}

pub mod fee {
    use super::*;

    #[wasm_bindgen]
    pub fn calculate_default_pool_trade_fee(amount: String) -> String {
        let amount = to_u128!(amount);

        let result = hydra_dx_math::fee::calculate_default_pool_trade_fee(amount);

        result.unwrap_or(0).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_pool_trade_fee(amount: String, _fee: String) -> String {
        let amount = to_u128!(amount);

        let result = hydra_dx_math::fee::calculate_default_pool_trade_fee(amount);

        result.unwrap_or(0).to_string()
    }
}