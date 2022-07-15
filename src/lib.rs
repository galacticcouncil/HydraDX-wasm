use wasm_bindgen::prelude::*;

macro_rules! to_u128 {
    ($($x:expr),+) => (
        {($($x.parse::<u128>().unwrap_or(0)),+)}
    );
}

#[cfg(feature = "xyk")]
pub mod xyk {
    pub use super::*;

    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::xyk::calculate_spot_price(sell_reserve, buy_reserve, amount);

        result.unwrap_or(0).to_string()
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
                String::from("1000")
            ),
            "500"
        );
        assert_eq!(
            xyk::calculate_liquidity_out_asset_b(
                String::from("1000"),
                String::from("2000"),
                String::from("500"),
                String::from("1000")
            ),
            "1000"
        );

        assert_eq!(
            xyk::calculate_liquidity_out_asset_a(
                String::from("0"),
                String::from("1"),
                String::from("0"),
                String::from("0")
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
}

#[cfg(feature = "lbp")]
pub mod lbp {
    pub use super::*;

    macro_rules! to_u32 {
        ($($x:expr),+) => (
            {($($x.parse::<u32>().unwrap_or(0)),+)}
        );
    }

    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::calculate_spot_price(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_out_given_in(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::calculate_out_given_in(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_in_given_out(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::calculate_in_given_out(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

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

#[cfg(feature = "stableswap")]
pub mod stableswap {
    pub use super::*;

    const D_ITERATIONS: u8 = 128;
    const Y_ITERATIONS: u8 = 64;

    #[wasm_bindgen]
    pub fn get_spot_price(reserve_in: String, reserve_out: String, amount: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(reserve_in, reserve_out, amount);

        let result = hydra_dx_math::xyk::calculate_spot_price(sell_reserve, buy_reserve, amount);

        result.unwrap_or(0).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_out_given_in(
        reserve_in: String,
        reserve_out: String,
        amount_in: String,
        amplification: String,
        precision: String,
    ) -> String {
        let (reserve_in, reserve_out, amount_in, amplification, precision) =
            to_u128!(reserve_in, reserve_out, amount_in, amplification, precision);
        let result = hydra_dx_math::stableswap::math::calculate_out_given_in::<D_ITERATIONS, Y_ITERATIONS>(
            reserve_in,
            reserve_out,
            amount_in,
            amplification,
            precision,
        );

        result.unwrap_or(0).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_in_given_out(
        reserve_in: String,
        reserve_out: String,
        amount_out: String,
        amplification: String,
        precision: String,
    ) -> String {
        let (reserve_in, reserve_out, amount_out, amplification, precision) =
            to_u128!(reserve_in, reserve_out, amount_out, amplification, precision);
        let result = hydra_dx_math::stableswap::math::calculate_in_given_out::<D_ITERATIONS, Y_ITERATIONS>(
            reserve_in,
            reserve_out,
            amount_out,
            amplification,
            precision,
        );

        result.unwrap_or(0).to_string()
    }

    #[test]
    fn out_in_works() {
        assert_eq!(
            calculate_out_given_in(
                String::from("1000"),
                String::from("2000"),
                String::from("500"),
                String::from("400"),
                String::from("1")
            ),
            "497"
        );
        assert_eq!(
            calculate_out_given_in(
                String::from("1"),
                String::from("0"),
                String::from("0"),
                String::from("400"),
                String::from("1")
            ),
            "0"
        );
    }

    #[test]
    fn in_out_works() {
        assert_eq!(
            calculate_in_given_out(
                String::from("1000"),
                String::from("2000"),
                String::from("500"),
                String::from("100"),
                String::from("1"),
            ),
            "503"
        );
        assert_eq!(
            calculate_in_given_out(
                String::from("0"),
                String::from("1"),
                String::from("0"),
                "100".to_string(),
                "1".to_string()
            ),
            "0"
        );
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn conversion() {
        assert_eq!(to_u128!("128"), 128_u128);
        assert_eq!(to_u128!("invalid"), 0_u128);
    }
}

pub mod fee {
    use super::*;

    #[wasm_bindgen]
    pub fn calculate_pool_trade_fee(a: String, fee_numerator: u32, fee_denominator: u32) -> String {
        let amount = to_u128!(a);

        let result = hydra_dx_math::fee::calculate_pool_trade_fee(amount, (fee_numerator, fee_denominator));

        result.unwrap_or(0).to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fee_calculations_works() {
            let n: u32 = 2;
            let d: u32 = 1000;
            assert_eq!(calculate_pool_trade_fee("1000".to_string(), n, d), "2".to_string());
            assert_eq!(
                calculate_pool_trade_fee(u128::MAX.to_string(), n, d),
                "680564733841876926926749214863536422".to_string()
            );
            assert_eq!(
                calculate_pool_trade_fee("1000000000000".to_string(), n, d),
                "2000000000".to_string()
            );
            assert_eq!(
                calculate_pool_trade_fee("1000000000000".to_string(), 0, 0),
                "0".to_string()
            );
            assert_eq!(
                calculate_pool_trade_fee(u128::MAX.to_string(), u32::MAX, 1),
                "0".to_string()
            );
            assert_eq!(
                calculate_pool_trade_fee(u128::MAX.to_string(), 1, 10),
                "34028236692093846346337460743176821145".to_string()
            );
        }
    }
}
