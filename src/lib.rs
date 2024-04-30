extern crate core;

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

fn error() -> String {
    "-1".to_string()
}

#[cfg(feature = "xyk")]
pub mod xyk {
    use num_traits::Zero;
    use sp_arithmetic::{FixedU128};
    pub use super::*;

    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);

        let result = hydra_dx_math::xyk::calculate_spot_price(sell_reserve, buy_reserve, amount);

        result.unwrap_or(0).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price(s: String, b: String) -> String {
        let (sell_reserve, buy_reserve) = to_u128!(s, b);

        let result = hydra_dx_math::xyk::spot_price(sell_reserve, buy_reserve, None);

        result.unwrap_or(FixedU128::zero()).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price_with_fee(s: String, b: String, fee_rate_n: String, fee_rate_d: String) -> String {
        let (sell_reserve, buy_reserve) = to_u128!(s, b);

        let (fee_rate_n, fee_rate_d) = to_u32!(fee_rate_n, fee_rate_d);

        let result = hydra_dx_math::xyk::spot_price(sell_reserve, buy_reserve, Some((fee_rate_n, fee_rate_d)));

        result.unwrap_or(FixedU128::zero()).to_string()
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
    fn calculate_spot_price_works() {
        assert_eq!(
            xyk::calculate_spot_price(String::from("1000"), String::from("2000")),
            "2000000000000000000"
        );
        assert_eq!(
            xyk::calculate_spot_price(String::from("1000"), String::from("0")),
            "0"
        );
    }

    #[test]
    fn calculate_spot_price_with_fee_works() {
        assert_eq!(
            xyk::calculate_spot_price_with_fee(String::from("1000"), String::from("2000"), String::from("3"), String::from("1000")),
            "1994000000000000000"
        );
        assert_eq!(
            xyk::calculate_spot_price_with_fee(String::from("1000"), String::from("2000"), String::from("0"), String::from("0")),
            "0"
        );

        assert_eq!(
            xyk::calculate_spot_price_with_fee(String::from("1000"), String::from("0"), String::from("3"), String::from("1000")),
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
                String::from("1000"),
            ),
            "500"
        );
        assert_eq!(
            xyk::calculate_liquidity_out_asset_b(
                String::from("1000"),
                String::from("2000"),
                String::from("500"),
                String::from("1000"),
            ),
            "1000"
        );

        assert_eq!(
            xyk::calculate_liquidity_out_asset_a(
                String::from("0"),
                String::from("1"),
                String::from("0"),
                String::from("0"),
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
    use num_traits::Zero;
    use sp_arithmetic::FixedU128;
    pub use super::*;

    #[wasm_bindgen]
    pub fn get_spot_price(s: String, b: String, s_w: String, b_w: String, a: String) -> String {
        let (sell_reserve, buy_reserve, amount) = to_u128!(s, b, a);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::calculate_spot_price(sell_reserve, buy_reserve, sell_weight, buy_weight, amount);

        result.unwrap_or(0).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price(s: String, b: String, s_w: String, b_w: String) -> String {
        let (sell_reserve, buy_reserve) = to_u128!(s, b);
        let (sell_weight, buy_weight) = to_u32!(s_w, b_w);

        let result =
            hydra_dx_math::lbp::spot_price(sell_reserve, buy_reserve, sell_weight, buy_weight, 0, 0, None);

        result.unwrap_or(FixedU128::zero()).to_string()
    }


    #[wasm_bindgen]
    pub fn calculate_spot_price_with_fee(s: String, b: String, s_w: String, b_w: String, fee_asset: String, asset_out: String, fee_rate_n: String, fee_rate_d: String ) -> String {
        let (sell_reserve, buy_reserve) = to_u128!(s, b);
        let (sell_weight, buy_weight, fee_asset, asset_out, fee_rate_n, fee_rate_d) = to_u32!(s_w, b_w, fee_asset, asset_out, fee_rate_n, fee_rate_d);

        let result =
            hydra_dx_math::lbp::spot_price(sell_reserve, buy_reserve, sell_weight, buy_weight, fee_asset, asset_out, Some((fee_rate_n, fee_rate_d)));

        result.unwrap_or(FixedU128::zero()).to_string()
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
}

#[cfg(feature = "stableswap")]
pub mod stableswap {
    pub use super::*;
    use hydra_dx_math::stableswap::types::AssetReserve;
    use std::collections::HashMap;

    use serde::Deserialize;
    use sp_arithmetic::{FixedPointNumber, FixedU128, Permill};
    #[cfg(test)]
    use sp_core::crypto::UncheckedFrom;
    #[cfg(test)]
    use sp_core::Hasher;
    #[cfg(test)]
    use sp_runtime::traits::IdentifyAccount;

    use serde_aux::prelude::*;

    macro_rules! parse_into {
        ($x:ty, $y:expr) => {{
            let r = if let Some(x) = $y.parse::<$x>().ok() {
                x
            } else {
                return error();
            };
            r
        }};
    }
    const D_ITERATIONS: u8 = 128;
    const Y_ITERATIONS: u8 = 64;

    #[derive(Deserialize, Copy, Clone, Debug)]
    pub struct AssetBalance {
        asset_id: u32,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        amount: u128,
        decimals: u8,
    }

    impl From<&AssetBalance> for AssetReserve {
        fn from(value: &AssetBalance) -> Self {
            Self {
                amount: value.amount,
                decimals: value.decimals,
            }
        }
    }

    #[derive(Deserialize, Copy, Clone, Debug)]
    pub struct AssetAmount {
        asset_id: u32,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        amount: u128,
    }

    #[wasm_bindgen]
    pub fn calculate_out_given_in(
        reserves: String,
        asset_in: u32,
        asset_out: u32,
        amount_in: String,
        amplification: String,
        fee: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);

        let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
        let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);

        if idx_in.is_none() || idx_out.is_none() {
            return error();
        }

        let amount_in = parse_into!(u128, amount_in);
        let amplification = parse_into!(u128, amplification);
        let fee = Permill::from_float(parse_into!(f64, fee));

        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();

        let result = hydra_dx_math::stableswap::calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
            &balances,
            idx_in.unwrap(),
            idx_out.unwrap(),
            amount_in,
            amplification,
            fee,
        );

        if let Some(r) = result {
            r.0.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_in_given_out(
        reserves: String,
        asset_in: u32,
        asset_out: u32,
        amount_out: String,
        amplification: String,
        fee: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);

        let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
        let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);

        if idx_in.is_none() || idx_out.is_none() {
            return error();
        }

        let amount_out = parse_into!(u128, amount_out);
        let amplification = parse_into!(u128, amplification);
        let fee = Permill::from_float(parse_into!(f64, fee));

        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();

        let result = hydra_dx_math::stableswap::calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
            &balances,
            idx_in.unwrap(),
            idx_out.unwrap(),
            amount_out,
            amplification,
            fee,
        );

        if let Some(r) = result {
            r.0.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_amplification(
        initial_amplification: String,
        final_amplification: String,
        initial_block: String,
        final_block: String,
        current_block: String,
    ) -> String {
        let initial_amplification = parse_into!(u128, initial_amplification);
        let final_amplification = parse_into!(u128, final_amplification);
        let initial_block = parse_into!(u128, initial_block);
        let final_block = parse_into!(u128, final_block);
        let current_block = parse_into!(u128, current_block);

        hydra_dx_math::stableswap::calculate_amplification(
            initial_amplification,
            final_amplification,
            initial_block,
            final_block,
            current_block,
        )
            .to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_shares(
        reserves: String,
        assets: String,
        amplification: String,
        share_issuance: String,
        fee: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);

        let assets: serde_json::Result<Vec<AssetAmount>> = serde_json::from_str(&assets);
        if assets.is_err() {
            return error();
        }
        let assets = assets.unwrap();
        if assets.len() > reserves.len() {
            return error();
        }

        let mut updated_reserves = reserves.clone();

        let mut liquidity: HashMap<u32, u128> = HashMap::new();
        for a in assets.iter() {
            let r = liquidity.insert(a.asset_id, a.amount);
            if r.is_some() {
                return error();
            }
        }
        for reserve in updated_reserves.iter_mut() {
            if let Some(v) = liquidity.get(&reserve.asset_id) {
                reserve.amount += v;
            }
        }
        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
        let updated_balances: Vec<AssetReserve> = updated_reserves.iter().map(|v| v.into()).collect();
        let amplification = parse_into!(u128, amplification);
        let issuance = parse_into!(u128, share_issuance);
        let fee = Permill::from_float(parse_into!(f64, fee));

        let result = hydra_dx_math::stableswap::calculate_shares::<D_ITERATIONS>(
            &balances,
            &updated_balances,
            amplification,
            issuance,
            fee,
        );

        if let Some(r) = result {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price(
        reserves: String,
        amplification: String,
        asset_in: String,
        asset_out: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);

        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
        let amplification = parse_into!(u128, amplification);
        let (asset_in, asset_out) = to_u32!(asset_in, asset_out);

        let Some(d) = hydra_dx_math::stableswap::calculate_d::<D_ITERATIONS>(&balances, amplification.clone()) else {
            return error()
        };

        let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
        let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);
        if idx_in.is_none() || idx_out.is_none() {
            return error();
        }

        let result = hydra_dx_math::stableswap::calculate_spot_price(
            &balances,
            amplification,
            d,
            idx_in.unwrap(),
            idx_out.unwrap(),
            None,
        );

        if let Some(r) = result {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price_with_fee(
        reserves: String,
        amplification: String,
        asset_in: String,
        asset_out: String,
        fee: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);

        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
        let amplification = parse_into!(u128, amplification);
        let (asset_in, asset_out) = to_u32!(asset_in, asset_out);
        let fee = Permill::from_float(parse_into!(f64, fee));

        let Some(d) = hydra_dx_math::stableswap::calculate_d::<D_ITERATIONS>(&balances, amplification.clone()) else {
            return error()
        };

        let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
        let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);
        if idx_in.is_none() || idx_out.is_none() {
            return error();
        }

        let result = hydra_dx_math::stableswap::calculate_spot_price(
            &balances,
            amplification,
            d,
            idx_in.unwrap(),
            idx_out.unwrap(),
            Some(fee),
        );

        if let Some(r) = result {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price_between_share_and_stableasset_with_fee(
        reserves: String,
        amplification: String,
        asset_out: String,
        share_issuance: String,
        pool_fee: String
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);

        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
        let amplification = parse_into!(u128, amplification);
        let asset_out = to_u32!(asset_out);
        let issuance = parse_into!(u128, share_issuance);
        let fee = Permill::from_float(parse_into!(f64, pool_fee));

        let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);
        if idx_out.is_none(){
            return error();
        }

        let reference_amount = 1000; //We use the  same MinTradingLimit we have configured to stableswap
        let result = hydra_dx_math::stableswap::calculate_spot_price_between_share_and_stableasset(
            &balances,
            idx_out.unwrap(),
            reference_amount,
            amplification,
            issuance,
            fee
        );

        if let Some(r) = result {
            r.to_string()
        } else {
            error()
        }
    }


    #[wasm_bindgen]
    pub fn calculate_spot_price_between_stableasset_and_share_with_fee(
        reserves: String,
        amplification: String,
        asset_out: String,
        share_issuance: String,
        pool_fee: String
    ) -> String {
        let reference_amount = 1000; //We use the  same MinTradingLimit we have configured to stableswap

        let assets = format!(
            r#"[{{"asset_id":{},"amount":"{}"}}]"#,
            asset_out, reference_amount
        );

        let shares_calculation = calculate_shares(
            reserves,
            assets,
            amplification,
            share_issuance,
            pool_fee,
        );

        let shares = to_u128!(shares_calculation);

        let Some(spot_price) = FixedU128::checked_from_rational(shares, reference_amount) else {
            return error();
        };

        spot_price.to_string()
    }


    #[wasm_bindgen]
    pub fn calculate_shares_for_amount(
        reserves: String,
        asset_in: u32,
        amount: String,
        amplification: String,
        share_issuance: String,
        fee: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);
        let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
        if idx_in.is_none() {
            return error();
        }
        let amount_in = parse_into!(u128, amount);
        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
        let amplification = parse_into!(u128, amplification);
        let issuance = parse_into!(u128, share_issuance);
        let fee = Permill::from_float(parse_into!(f64, fee));

        let result = hydra_dx_math::stableswap::calculate_shares_for_amount::<D_ITERATIONS>(
            &balances,
            idx_in.unwrap(),
            amount_in,
            amplification,
            issuance,
            fee,
        );

        if let Some(r) = result {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_add_one_asset(
        reserves: String,
        shares: String,
        asset_in: u32,
        amplification: String,
        share_issuance: String,
        fee: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);
        let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
        if idx_in.is_none() {
            return error();
        }

        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
        let shares = parse_into!(u128, shares);
        let amplification = parse_into!(u128, amplification);
        let issuance = parse_into!(u128, share_issuance);
        let fee = Permill::from_float(parse_into!(f64, fee));

        let result = hydra_dx_math::stableswap::calculate_add_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
            &balances,
            shares,
            idx_in.unwrap(),
            issuance,
            amplification,
            fee,
        );

        if let Some(r) = result {
            r.0.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn pool_account_name(share_asset_id: u32) -> Vec<u8> {
        let mut name = "sts".as_bytes().to_vec();
        name.extend_from_slice(&(share_asset_id).to_le_bytes());
        return name;
    }

    #[test]
    fn test_account_derive() {
        let share_asset_id: u32 = 2000;
        let account_name = pool_account_name(share_asset_id);
        let hashed = sp_runtime::traits::BlakeTwo256::hash(&account_name);
        let account = <<sp_runtime::MultiSignature as sp_runtime::traits::Verify>::Signer as IdentifyAccount>::AccountId::unchecked_from(hashed);
        assert_eq!(
            account.to_string(),
            "5CmwA9nfiBThjkLw1PSBbEQmZMdGMtd3WHtxJLy4hdT6LtRu".to_string()
        );
    }

    #[wasm_bindgen]
    pub fn calculate_liquidity_out_one_asset(
        reserves: String,
        shares: String,
        asset_out: u32,
        amplification: String,
        share_issuance: String,
        withdraw_fee: String,
    ) -> String {
        let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
        if reserves.is_err() {
            return error();
        }
        let mut reserves = reserves.unwrap();
        reserves.sort_by_key(|v| v.asset_id);

        let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);
        if idx_out.is_none() {
            return error();
        }

        let shares_out = parse_into!(u128, shares);
        let amplification = parse_into!(u128, amplification);
        let issuance = parse_into!(u128, share_issuance);
        let fee = Permill::from_float(parse_into!(f64, withdraw_fee));

        let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();

        let result = hydra_dx_math::stableswap::calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
            &balances,
            shares_out,
            idx_out.unwrap(),
            issuance,
            amplification,
            fee,
        );

        if let Some(r) = result {
            r.0.to_string()
        } else {
            error()
        }
    }

    #[test]
    fn calculate_out_givein_should_work_when_correct_json_format_provided() {
        let data = r#"
        [{
            "asset_id": 1,
            "amount": "1000000000000",
            "decimals": 12
        },
        {
            "asset_id": 0,
            "amount": "1000000000000",
            "decimals": 12
        }
        ]"#;
        let result = calculate_out_given_in(
            data.to_string(),
            0,
            1,
            "1000000000".to_string(),
            "1".to_string(),
            "0".to_string(),
        );

        assert_eq!(result, "999500248".to_string());
    }

    #[test]
    fn calculate_shares_should_work_when_correct_json_format_provided() {
        let data = r#"
        [{
            "asset_id": 0,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount": "5000000000000000000000",
            "decimals": 12
        }
        ]"#;
        let assets = r#"
            [{"asset_id":1,"amount":"43000000000000000000"}]
        "#;

        let result = calculate_shares(
            data.to_string(),
            assets.to_string(),
            "1000".to_string(),
            "64839594451719860".to_string(),
            "0".to_string(),
        );

        assert_eq!(result, "371541351762585".to_string());
    }

    #[test]
    fn calculate_spot_price_should_work() {
        let data = r#"
        [{
            "asset_id": 0,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount": "5000000000000000000000",
            "decimals": 12
        }
        ]"#;


        let result = calculate_spot_price(
            data.to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
        );

        assert_eq!(result, "36043643".to_string());

        let result = calculate_spot_price(
            "0".to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
        );

        assert_eq!(result, "-1".to_string());
    }

    #[test]
    fn calculate_spot_price_with_fee_should_work() {
        let data = r#"
        [{
            "asset_id": 0,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount": "5000000000000000000000",
            "decimals": 12
        }
        ]"#;


        let result = calculate_spot_price_with_fee(
            data.to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "36407720".to_string());

        let result = calculate_spot_price_with_fee(
            "0".to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "-1".to_string());
    }

    #[test]
    fn calculate_spot_price_between_share_and_stable_with_fee_should_work() {
        let data = r#"
        [{
            "asset_id": 0,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount": "5000000000000000000000",
            "decimals": 12
        }
        ]"#;


        let result = calculate_spot_price_between_share_and_stableasset_with_fee(
            data.to_string(),
            100.to_string(),
            "0".to_string(),
            "2000000000".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "125000000000000000000".to_string());

        let result = calculate_spot_price_between_share_and_stableasset_with_fee(
            data.to_string(),
            100.to_string(),
            "999".to_string(),
            "2000000000".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "-1".to_string());
    }


    #[test]
    fn calculate_spot_price_between_stable_and_share_with_fee_should_work() {
        let data = r#"
        [{
            "asset_id": 0,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount": "5000000000000000000000",
            "decimals": 12
        }
        ]"#;
        let assets = r#"
            [{"asset_id":1,"amount":"43000000000000000000"}]
        "#;

        let result = calculate_spot_price_between_stableasset_and_share_with_fee(
            data.to_string(),
            100.to_string(),
            "1".to_string(),
            "648395944517198603232".to_string(),
            "0".to_string(),
        );

        assert_eq!(result, "86000000000000000".to_string());

        let result = calculate_spot_price_between_stableasset_and_share_with_fee(
            data.to_string(),
            100.to_string(),
            "9999".to_string(),
            "648395944517198603232".to_string(),
            "0".to_string(),
        );

        assert_eq!(result, "0".to_string());
    }


}

#[cfg(feature = "liquidity-mining")]
pub mod liquidity_mining {
    pub use super::*;
    use sp_arithmetic::{fixed_point::FixedU128, FixedPointNumber};

    macro_rules! parse_into {
        ($x:ty, $y:expr) => {{
            let r = if let Some(x) = $y.parse::<$x>().ok() {
                x
            } else {
                return error();
            };
            r
        }};
    }

    #[wasm_bindgen]
    pub fn fixed_from_rational(a: String, b: String) -> String {
        match FixedU128::checked_from_rational(parse_into!(u128, a), parse_into!(u128, b)) {
            Some(v) => v.to_string(),
            None => error(),
        }
    }

    #[wasm_bindgen]
    pub fn calculate_loyalty_multiplier(
        period: String,
        initial_reward_percentage: String,
        scale_coef: String,
    ) -> String {
        let period = parse_into!(u128, period);
        let reward_percentage = FixedU128::from_inner(parse_into!(u128, initial_reward_percentage));
        let scale_coef = parse_into!(u32, scale_coef);

        let result = hydra_dx_math::liquidity_mining::calculate_loyalty_multiplier::<u128>(
            period,
            reward_percentage,
            scale_coef,
        );

        if let Some(r) = result.ok() {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_accumulated_rps(accumulated_rps_now: String, total_shares: String, reward: String) -> String {
        let rps = FixedU128::from_inner(parse_into!(u128, accumulated_rps_now));
        let shares = parse_into!(u128, total_shares);
        let reward = parse_into!(u128, reward);

        let result = hydra_dx_math::liquidity_mining::calculate_accumulated_rps(rps, shares, reward);

        if let Some(r) = result.ok() {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_user_reward(
        accumulated_rpvs: String,
        valued_shares: String,
        accumulated_claimed_rewards: String,
        accumulated_rpvs_now: String,
        loyalty_multiplier: String,
    ) -> String {
        let rps = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs));
        let shares = parse_into!(u128, valued_shares);
        let rewards = parse_into!(u128, accumulated_claimed_rewards);

        let rps_now = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs_now));
        let multiplier = FixedU128::from_inner(parse_into!(u128, loyalty_multiplier));

        let result = hydra_dx_math::liquidity_mining::calculate_user_reward(rps, shares, rewards, rps_now, multiplier);

        if let Some(r) = result.ok() {
            r.0.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_user_unclaimed_reward(
        accumulated_rpvs: String,
        valued_shares: String,
        accumulated_claimed_rewards: String,
        accumulated_rpvs_now: String,
        loyalty_multiplier: String,
    ) -> String {
        let rps = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs));
        let shares = parse_into!(u128, valued_shares);
        let rewards = parse_into!(u128, accumulated_claimed_rewards);

        let rps_now = FixedU128::from_inner(parse_into!(u128, accumulated_rpvs_now));
        let multiplier = FixedU128::from_inner(parse_into!(u128, loyalty_multiplier));

        let result = hydra_dx_math::liquidity_mining::calculate_user_reward(rps, shares, rewards, rps_now, multiplier);

        if let Some(r) = result.ok() {
            r.1.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_valued_shares(shares: String, incentivized_asset_balance: String) -> String {
        let shares = parse_into!(u128, shares);
        let balance = parse_into!(u128, incentivized_asset_balance);

        let result = hydra_dx_math::liquidity_mining::calculate_valued_shares(shares, balance);

        if let Some(r) = result.ok() {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_reward(accumulated_rps_start: String, accumulated_rps_now: String, shares: String) -> String {
        let rps_start = FixedU128::from_inner(parse_into!(u128, accumulated_rps_start));
        let rps_now = FixedU128::from_inner(parse_into!(u128, accumulated_rps_now));

        let shares = parse_into!(u128, shares);

        let result = hydra_dx_math::liquidity_mining::calculate_reward(rps_start, rps_now, shares);

        if let Some(r) = result.ok() {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_global_farm_shares(valued_shares: String, multiplier: String) -> String {
        let s = parse_into!(u128, valued_shares);
        let m = FixedU128::from_inner(parse_into!(u128, multiplier));

        let result = hydra_dx_math::liquidity_mining::calculate_global_farm_shares(s, m);

        if let Some(r) = result.ok() {
            r.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_yield_farm_rewards(
        yield_farm_rpz: String,
        global_farm_rpz: String,
        multiplier: String,
        total_valued_shares: String,
    ) -> String {
        let y_rpz = FixedU128::from_inner(parse_into!(u128, yield_farm_rpz));
        let g_rpz = FixedU128::from_inner(parse_into!(u128, global_farm_rpz));
        let m = FixedU128::from_inner(parse_into!(u128, multiplier));
        let vs = parse_into!(u128, total_valued_shares);

        let result = hydra_dx_math::liquidity_mining::calculate_yield_farm_rewards(y_rpz, g_rpz, m, vs);

        if let Some(v) = result.ok() {
            v.1.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_yield_farm_delta_rpvs(
        yield_farm_rpz: String,
        global_farm_rpz: String,
        multiplier: String,
        total_valued_shares: String,
    ) -> String {
        let y_rpz = FixedU128::from_inner(parse_into!(u128, yield_farm_rpz));
        let g_rpz = FixedU128::from_inner(parse_into!(u128, global_farm_rpz));
        let m = FixedU128::from_inner(parse_into!(u128, multiplier));
        let vs = parse_into!(u128, total_valued_shares);

        let result = hydra_dx_math::liquidity_mining::calculate_yield_farm_rewards(y_rpz, g_rpz, m, vs);

        if let Some(v) = result.ok() {
            v.0.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_global_farm_rewards(
        total_shares_z: String,
        price_adjustment: String,
        yield_per_period: String,
        max_reward_per_period: String,
        periods_since_last_update: String,
    ) -> String {
        let ts = parse_into!(u128, total_shares_z);
        let p_adj = FixedU128::from_inner(parse_into!(u128, price_adjustment));
        let ypp = FixedU128::from_inner(parse_into!(u128, yield_per_period));
        let max_rew_per_period = parse_into!(u128, max_reward_per_period);
        let periods = parse_into!(u128, periods_since_last_update);

        let result =
            hydra_dx_math::liquidity_mining::calculate_global_farm_rewards(ts, p_adj, ypp, max_rew_per_period, periods);

        if let Some(v) = result.ok() {
            v.to_string()
        } else {
            error()
        }
    }

    #[test]
    fn calculate_global_farm_rewards_should_work_when_input_is_correct() {
        assert_eq!(
            calculate_global_farm_rewards(
                "17989865464312".to_string(),
                "1000000000000000000".to_string(),
                "138571428600000000".to_string(),
                "59898".to_string(),
                "1".to_string()
            ),
            "59898"
        );

        assert_eq!(
            calculate_global_farm_rewards(
                "35189".to_string(),
                "1000000000000000000".to_string(),
                "133333333300000000".to_string(),
                "468787897".to_string(),
                "10".to_string()
            ),
            "46918"
        );
    }

    #[test]
    fn calculate_yield_farm_rewards_should_work_when_input_is_correct() {
        assert_eq!(
            calculate_yield_farm_rewards(
                "82000000000000000000".to_string(),
                "357000000000000000000".to_string(),
                "1000000000000000000".to_string(),
                "932564".to_string()
            ),
            "256455100"
        );

        assert_eq!(
            calculate_yield_farm_rewards(
                "2491000000000000000000".to_string(),
                "2537000000000000000000".to_string(),
                "1000000000000000000".to_string(),
                "85100506".to_string()
            ),
            "3914623276"
        );
    }

    #[test]
    fn calculate_yield_farm_delta_rpvs_should_work_when_input_is_correct() {
        assert_eq!(
            calculate_yield_farm_delta_rpvs(
                "82000000000000000000".to_string(),
                "357000000000000000000".to_string(),
                "1000000000000000000".to_string(),
                "932564".to_string()
            ),
            "275000000000000000000"
        );

        assert_eq!(
            calculate_yield_farm_delta_rpvs(
                "2491000000000000000000".to_string(),
                "2537000000000000000000".to_string(),
                "1000000000000000000".to_string(),
                "85100506".to_string()
            ),
            "46000000000000000000"
        );
    }

    #[test]
    fn calculate_loyalty_multiplier_should_work_when_input_is_correct() {
        assert_eq!(
            calculate_loyalty_multiplier("100".to_string(), "200000000000000000".to_string(), "2".to_string()),
            "984313725490196078"
        );
        assert_eq!(
            calculate_loyalty_multiplier("100".to_string(), "1000000000000000000".to_string(), "2".to_string()),
            "1000000000000000000"
        );
    }

    #[test]
    fn calculate_loyalty_multiplier_should_fail_when_input_is_incorrect() {
        assert_eq!(
            calculate_loyalty_multiplier("invalid".to_string(), "200000000000000000".to_string(), "2".to_string()),
            "-1"
        );
        assert_eq!(
            calculate_loyalty_multiplier("100".to_string(), "invalid".to_string(), "2".to_string()),
            "-1"
        );
        assert_eq!(
            calculate_loyalty_multiplier(
                "100".to_string(),
                "100000000000000000".to_string(),
                "invalid".to_string(),
            ),
            "-1"
        );
    }

    #[test]
    fn fixed_from_rational_should_work() {
        assert_eq!(
            fixed_from_rational("1".to_string(), "5".to_string()),
            "200000000000000000"
        );

        assert_eq!(fixed_from_rational("1".to_string(), "0".to_string()), "-1");

        assert_eq!(
            fixed_from_rational("5343".to_string(), "5".to_string()),
            "1068600000000000000000"
        );

        assert_eq!(
            fixed_from_rational("93846346337460743".to_string(), "100000000".to_string()),
            "938463463374607430000000000"
        );

        assert_eq!(
            fixed_from_rational("340282366920938463463374607431768211455".to_string(), "10".to_string()),
            "-1"
        );
    }
}

#[cfg(feature = "omnipool")]
pub mod omnipool {
    pub use super::*;
    use hydra_dx_math::dynamic_fees::types::{FeeParams, OracleEntry};
    use hydra_dx_math::omnipool::types::{AssetReserveState, Position as OmnipoolPosition, I129};
    use sp_arithmetic::{FixedPointNumber, FixedU128, Permill};

    macro_rules! parse_into {
        ($x:ty, $y:expr, $e:expr) => {{
            let r = if let Some(x) = $y.parse::<$x>().ok() {
                x
            } else {
                return $e;
            };
            r
        }};
    }

    #[wasm_bindgen]
    pub fn calculate_shares(
        asset_reserve: String,
        asset_hub_reserve: String,
        asset_shares: String,
        amount_in: String,
    ) -> String {
        let amount = parse_into!(u128, amount_in, error());
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let shares = parse_into!(u128, asset_shares, error());

        let state = AssetReserveState {
            reserve,
            hub_reserve,
            shares,
            ..Default::default()
        };

        if let Some(state_changes) = hydra_dx_math::omnipool::calculate_add_liquidity_state_changes(
            &state,
            amount,
            I129 {
                value: 0u128,
                negative: false,
            },
            0u128,
        ) {
            (*state_changes.asset.delta_shares).to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_withdrawal_fee(spot_price: String, oracle_price: String, min_withdrawal_fee: String) -> String {
        let spot_price = FixedU128::from_rational(parse_into!(u128, spot_price, error()), FixedU128::DIV);
        let oracle_price = FixedU128::from_rational(parse_into!(u128, oracle_price, error()), FixedU128::DIV);
        let min_fee = Permill::from_float(parse_into!(f64, min_withdrawal_fee, error()));

        hydra_dx_math::omnipool::calculate_withdrawal_fee(spot_price, oracle_price, min_fee).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_liquidity_out(
        asset_reserve: String,
        asset_hub_reserve: String,
        asset_shares: String,
        position_amount: String,
        position_shares: String,
        position_price: String,
        shares_to_remove: String,
        withdrawal_fee: String,
    ) -> String {
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let shares = parse_into!(u128, asset_shares, error());
        let position_amount = parse_into!(u128, position_amount, error());
        let position_shares = parse_into!(u128, position_shares, error());
        let position_price = parse_into!(u128, position_price, error());
        let shares_amount = parse_into!(u128, shares_to_remove, error());
        let withdrawal_fee = FixedU128::from_rational(parse_into!(u128, withdrawal_fee, error()), FixedU128::DIV);

        let state = AssetReserveState {
            reserve,
            hub_reserve,
            shares,
            ..Default::default()
        };

        let position = OmnipoolPosition {
            amount: position_amount,
            shares: position_shares,
            price: (position_price, FixedU128::DIV),
        };

        if let Some(state_changes) = hydra_dx_math::omnipool::calculate_remove_liquidity_state_changes(
            &state,
            shares_amount,
            &position,
            I129 {
                value: 0u128,
                negative: false,
            },
            0u128,
            withdrawal_fee,
        ) {
            (*state_changes.asset.delta_reserve).to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_liquidity_lrna_out(
        asset_reserve: String,
        asset_hub_reserve: String,
        asset_shares: String,
        position_amount: String,
        position_shares: String,
        position_price: String,
        shares_to_remove: String,
        withdrawal_fee: String,
    ) -> String {
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let shares = parse_into!(u128, asset_shares, error());
        let position_amount = parse_into!(u128, position_amount, error());
        let position_shares = parse_into!(u128, position_shares, error());
        let position_price = parse_into!(u128, position_price, error());
        let shares_amount = parse_into!(u128, shares_to_remove, error());
        let withdrawal_fee = FixedU128::from_rational(parse_into!(u128, withdrawal_fee, error()), FixedU128::DIV);

        let state = AssetReserveState {
            reserve,
            hub_reserve,
            shares,
            ..Default::default()
        };

        let position = OmnipoolPosition {
            amount: position_amount,
            shares: position_shares,
            price: (position_price, FixedU128::DIV),
        };

        if let Some(state_changes) = hydra_dx_math::omnipool::calculate_remove_liquidity_state_changes(
            &state,
            shares_amount,
            &position,
            I129 {
                value: 0u128,
                negative: false,
            },
            0u128,
            withdrawal_fee,
        ) {
            state_changes.lp_hub_amount.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn recalculate_asset_fee(
        asset_amount_in: String,
        asset_amount_out: String,
        asset_liquidity: String,
        previous_fee: String,
        block_difference: String,
        min_fee: String,
        max_fee: String,
        decay: String,
        amplification: String,
    ) -> String {
        let amount_in = parse_into!(u128, asset_amount_in, error());
        let amount_out = parse_into!(u128, asset_amount_out, error());
        let liquidity = parse_into!(u128, asset_liquidity, error());
        let block_difference = parse_into!(u128, block_difference, error());
        let previous_fee = Permill::from_float(parse_into!(f64, previous_fee, error()));
        let min_fee = Permill::from_float(parse_into!(f64, min_fee, error()));
        let max_fee = Permill::from_float(parse_into!(f64, max_fee, error()));
        let decay = FixedU128::from_rational(parse_into!(u128, decay, error()), FixedU128::DIV);
        let amplification = FixedU128::from_rational(parse_into!(u128, amplification, error()), FixedU128::DIV);

        let entry = OracleEntry {
            amount_in,
            amount_out,
            liquidity,
        };

        let result = hydra_dx_math::dynamic_fees::recalculate_asset_fee(
            entry,
            previous_fee,
            block_difference,
            FeeParams {
                min_fee,
                max_fee,
                decay,
                amplification,
            },
        );
        FixedU128::from(result).to_float().to_string()
    }

    #[wasm_bindgen]
    pub fn recalculate_protocol_fee(
        asset_amount_in: String,
        asset_amount_out: String,
        asset_liquidity: String,
        previous_fee: String,
        block_difference: String,
        min_fee: String,
        max_fee: String,
        decay: String,
        amplification: String,
    ) -> String {
        let amount_in = parse_into!(u128, asset_amount_in, error());
        let amount_out = parse_into!(u128, asset_amount_out, error());
        let liquidity = parse_into!(u128, asset_liquidity, error());
        let block_difference = parse_into!(u128, block_difference, error());
        let previous_fee = Permill::from_float(parse_into!(f64, previous_fee, error()));
        let min_fee = Permill::from_float(parse_into!(f64, min_fee, error()));
        let max_fee = Permill::from_float(parse_into!(f64, max_fee, error()));
        let decay = FixedU128::from_rational(parse_into!(u128, decay, error()), FixedU128::DIV);
        let amplification = FixedU128::from_rational(parse_into!(u128, amplification, error()), FixedU128::DIV);

        let entry = OracleEntry {
            amount_in,
            amount_out,
            liquidity,
        };

        let result = hydra_dx_math::dynamic_fees::recalculate_protocol_fee(
            entry,
            previous_fee,
            block_difference,
            FeeParams {
                min_fee,
                max_fee,
                decay,
                amplification,
            },
        );
        FixedU128::from(result).to_float().to_string()
    }

    #[cfg(test)]
    mod tests {
        use crate::omnipool::*;
        use sp_arithmetic::FixedPointNumber;

        #[test]
        fn rational_to_fixed_should_be_converted_by_bn_correctly() {
            let n = 2267311920182547u128;
            let d = 49639234739826304676022u128;
            /*
               const [n,d] = position.price.map(n => new BigNumber(n.toString()))
               const fixed_price = n.dividedBy(d).multipliedBy(BN_10.pow(18)).toFixed(0, ROUND_CEIL)
            */
            let fixed_price = parse_into!(FixedU128, "45675803264", ());
            let price = FixedU128::checked_from_rational(n, d).unwrap();
            assert_eq!(price, fixed_price);
        }

        #[test]
        // position 1 on rococo
        fn liquidity_out_should_equal_provided_case_1() {
            // Arrange
            let provided_amount = "10000000000000000000".to_string();
            let shares = "10074707027444081228".to_string();
            let position_price = "45675803264".to_string();

            let asset_reserve = "53403520037510677010114".to_string();
            let asset_hub_reserve = "2108586865957437".to_string();
            let asset_shares = "50010074707027444081228".to_string();

            // Act
            let lrna = calculate_liquidity_lrna_out(
                asset_reserve.clone(),
                asset_hub_reserve.clone(),
                asset_shares.clone(),
                provided_amount.clone(),
                shares.clone(),
                position_price.clone(),
                shares.clone(),
                "0".to_string(),
            );
            let out = calculate_liquidity_out(
                asset_reserve,
                asset_hub_reserve,
                asset_shares,
                provided_amount.clone(),
                shares.clone(),
                position_price,
                shares,
                "0".to_string(),
            );

            // Assert
            assert_eq!(lrna, 0.to_string());
            assert_eq!(out, "9976117319866596585");
        }

        #[test]
        // position 3 on rococo
        fn liquidity_out_should_equal_provided_case_2() {
            // Arrange
            let provided_amount = "100000000000".to_string();
            let shares = "93781953587".to_string();
            let position_price = "22566115813746724172".to_string();

            let asset_reserve = "93635371032830".to_string();
            let asset_hub_reserve = "2112986626989987".to_string();
            let asset_shares = "87813080203587".to_string();

            // Act
            let lrna = calculate_liquidity_lrna_out(
                asset_reserve.clone(),
                asset_hub_reserve.clone(),
                asset_shares.clone(),
                provided_amount.clone(),
                shares.clone(),
                position_price.clone(),
                shares.clone(),
                "0".to_string(),
            );
            let out = calculate_liquidity_out(
                asset_reserve,
                asset_hub_reserve,
                asset_shares,
                provided_amount.clone(),
                shares.clone(),
                position_price,
                shares,
                "0".to_string(),
            );

            // Assert
            assert_eq!(lrna, 0.to_string());
            assert_eq!(out, "99999999998");
        }
    }

    #[wasm_bindgen]
    pub fn calculate_out_given_in(
        asset_in_reserve: String,
        asset_in_hub_reserve: String,
        asset_in_shares: String,
        asset_out_reserve: String,
        asset_out_hub_reserve: String,
        asset_out_shares: String,
        amount_in: String,
        asset_fee: String,
        protocol_fee: String,
    ) -> String {
        let reserve_in = parse_into!(u128, asset_in_reserve, error());
        let hub_reserve_in = parse_into!(u128, asset_in_hub_reserve, error());
        let shares_in = parse_into!(u128, asset_in_shares, error());

        let reserve_out = parse_into!(u128, asset_out_reserve, error());
        let hub_reserve_out = parse_into!(u128, asset_out_hub_reserve, error());
        let shares_out = parse_into!(u128, asset_out_shares, error());

        let amount = parse_into!(u128, amount_in, error());
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, error()));
        let protocol_fee = Permill::from_float(parse_into!(f64, protocol_fee, error()));

        let asset_in = AssetReserveState {
            reserve: reserve_in,
            hub_reserve: hub_reserve_in,
            shares: shares_in,
            ..Default::default()
        };

        let asset_out = AssetReserveState {
            reserve: reserve_out,
            hub_reserve: hub_reserve_out,
            shares: shares_out,
            ..Default::default()
        };

        let state_changes = if let Some(r) = hydra_dx_math::omnipool::calculate_sell_state_changes(
            &asset_in,
            &asset_out,
            amount,
            asset_fee,
            protocol_fee,
            0u128,
        ) {
            r
        } else {
            return error();
        };

        (*state_changes.asset_out.delta_reserve).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_out_given_lrna_in(
        asset_reserve: String,
        asset_hub_reserve: String,
        asset_shares: String,
        amount_in: String,
        asset_fee: String,
    ) -> String {
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let shares = parse_into!(u128, asset_shares, error());

        let amount = parse_into!(u128, amount_in, error());
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, error()));

        let asset = AssetReserveState {
            reserve,
            hub_reserve,
            shares,
            ..Default::default()
        };

        let state_changes = if let Some(r) = hydra_dx_math::omnipool::calculate_sell_hub_state_changes(
            &asset,
            amount,
            asset_fee,
            I129 {
                value: 0,
                negative: false,
            },
            1_000_000u128, // This is not relevant here,but it cant be 0
        ) {
            r
        } else {
            return error();
        };

        (*state_changes.asset.delta_reserve).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_in_given_out(
        asset_in_reserve: String,
        asset_in_hub_reserve: String,
        asset_in_shares: String,
        asset_out_reserve: String,
        asset_out_hub_reserve: String,
        asset_out_shares: String,
        amount_out: String,
        asset_fee: String,
        protocol_fee: String,
    ) -> String {
        let reserve_in = parse_into!(u128, asset_in_reserve, error());
        let hub_reserve_in = parse_into!(u128, asset_in_hub_reserve, error());
        let shares_in = parse_into!(u128, asset_in_shares, error());

        let reserve_out = parse_into!(u128, asset_out_reserve, error());
        let hub_reserve_out = parse_into!(u128, asset_out_hub_reserve, error());
        let shares_out = parse_into!(u128, asset_out_shares, error());

        let amount = parse_into!(u128, amount_out, error());
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, error()));
        let protocol_fee = Permill::from_float(parse_into!(f64, protocol_fee, error()));

        let asset_in = AssetReserveState {
            reserve: reserve_in,
            hub_reserve: hub_reserve_in,
            shares: shares_in,
            ..Default::default()
        };

        let asset_out = AssetReserveState {
            reserve: reserve_out,
            hub_reserve: hub_reserve_out,
            shares: shares_out,
            ..Default::default()
        };

        let state_changes = if let Some(r) = hydra_dx_math::omnipool::calculate_buy_state_changes(
            &asset_in,
            &asset_out,
            amount,
            asset_fee,
            protocol_fee,
            0u128,
        ) {
            r
        } else {
            return error();
        };

        (*state_changes.asset_in.delta_reserve).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_lrna_in_given_out(
        asset_reserve: String,
        asset_hub_reserve: String,
        asset_shares: String,
        amount_out: String,
        asset_fee: String,
    ) -> String {
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let shares = parse_into!(u128, asset_shares, error());

        let amount = parse_into!(u128, amount_out, error());
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, error()));

        let asset = AssetReserveState {
            reserve,
            hub_reserve,
            shares,
            ..Default::default()
        };

        let state_changes = if let Some(r) = hydra_dx_math::omnipool::calculate_buy_for_hub_asset_state_changes(
            &asset,
            amount,
            asset_fee,
            I129 {
                value: 0,
                negative: false,
            },
            1_000_000u128, // This is not relevant here,but it cant be 0
        ) {
            r
        } else {
            return error();
        };

        (*state_changes.asset.delta_hub_reserve).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price(
        asset_a_reserve: String,
        asset_a_hub_reserve: String,
        asset_b_reserve: String,
        asset_b_hub_reserve: String,
    ) -> String {
        let reserve_a = parse_into!(u128, asset_a_reserve, error());
        let hub_reserve_a = parse_into!(u128, asset_a_hub_reserve, error());
        let reserve_b = parse_into!(u128, asset_b_reserve, error());
        let hub_reserve_b = parse_into!(u128, asset_b_hub_reserve, error());

        let asset_a = AssetReserveState {
            reserve: reserve_a,
            hub_reserve: hub_reserve_a,
            ..Default::default()
        };

        let asset_b = AssetReserveState {
            reserve: reserve_b,
            hub_reserve: hub_reserve_b,
            ..Default::default()
        };

        if let Some(result) = hydra_dx_math::omnipool::calculate_spot_sprice(&asset_a, &asset_b, None) {
            result.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_spot_price_with_fee(
        asset_a_reserve: String,
        asset_a_hub_reserve: String,
        asset_b_reserve: String,
        asset_b_hub_reserve: String,
        protocol_fee: String,
        asset_fee: String
    ) -> String {
        let reserve_a = parse_into!(u128, asset_a_reserve, error());
        let hub_reserve_a = parse_into!(u128, asset_a_hub_reserve, error());
        let reserve_b = parse_into!(u128, asset_b_reserve, error());
        let hub_reserve_b = parse_into!(u128, asset_b_hub_reserve, error());

        let protocol_fee = Permill::from_float(parse_into!(f64, protocol_fee, error()));
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, error()));

        let asset_a = AssetReserveState {
            reserve: reserve_a,
            hub_reserve: hub_reserve_a,
            ..Default::default()
        };

        let asset_b = AssetReserveState {
            reserve: reserve_b,
            hub_reserve: hub_reserve_b,
            ..Default::default()
        };


        if let Some(result) = hydra_dx_math::omnipool::calculate_spot_sprice(&asset_a, &asset_b, Some((protocol_fee, asset_fee))) {
            result.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_lrna_spot_price(asset_reserve: String, asset_hub_reserve: String) -> String {
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());

        let asset = AssetReserveState {
            reserve,
            hub_reserve,
            ..Default::default()
        };

        if let Some(result) = hydra_dx_math::omnipool::calculate_lrna_spot_sprice(&asset, None) {
            result.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_lrna_spot_price_with_fee(asset_reserve: String, asset_hub_reserve: String, asset_fee: String) -> String {
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, error()));

        let asset = AssetReserveState {
            reserve,
            hub_reserve,
            ..Default::default()
        };

        if let Some(result) = hydra_dx_math::omnipool::calculate_lrna_spot_sprice(&asset, Some(asset_fee)) {
            result.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_cap_difference(
        asset_reserve: String,
        asset_hub_reserve: String,
        asset_cap: String,
        total_hub_reserve: String,
    ) -> String {
        let asset_hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let asset_reserve = parse_into!(u128, asset_reserve, error());
        let asset_cap = parse_into!(u128, asset_cap, error());
        let total_hub_reserve = parse_into!(u128, total_hub_reserve, error());

        let asset_state = AssetReserveState {
            reserve: asset_reserve,
            hub_reserve: asset_hub_reserve,
            ..Default::default()
        };

        if let Some(result) =
            hydra_dx_math::omnipool::calculate_cap_difference(&asset_state, asset_cap, total_hub_reserve)
        {
            result.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn verify_asset_cap(
        asset_hub_reserve: String,
        asset_cap: String,
        hub_added: String,
        total_hub_reserve: String,
    ) -> bool {
        let asset_hub_reserve = parse_into!(u128, asset_hub_reserve, false);
        let asset_cap = parse_into!(u128, asset_cap, false);
        let total_hub_reserve = parse_into!(u128, total_hub_reserve, false);
        let hub_added = parse_into!(u128, hub_added, false);

        let asset_state = AssetReserveState {
            hub_reserve: asset_hub_reserve,
            ..Default::default()
        };

        if let Some(result) =
            hydra_dx_math::omnipool::verify_asset_cap(&asset_state, asset_cap, hub_added, total_hub_reserve)
        {
            result
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn calculate_tvl_cap_difference(
        asset_reserve: String,
        asset_hub_reserve: String,
        stable_asset_reserve: String,
        stable_asset_hub_reserve: String,
        tvl_cap: String,
        total_hub_reserve: String,
    ) -> String {
        let asset_hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let asset_reserve = parse_into!(u128, asset_reserve, error());
        let stable_asset_hub_reserve = parse_into!(u128, stable_asset_hub_reserve, error());
        let stable_asset_reserve = parse_into!(u128, stable_asset_reserve, error());
        let tvl_cap = parse_into!(u128, tvl_cap, error());
        let total_hub_reserve = parse_into!(u128, total_hub_reserve, error());

        let asset_state = AssetReserveState {
            reserve: asset_reserve,
            hub_reserve: asset_hub_reserve,
            ..Default::default()
        };

        let stable_asset_state = AssetReserveState {
            reserve: stable_asset_reserve,
            hub_reserve: stable_asset_hub_reserve,
            ..Default::default()
        };

        if let Some(result) = hydra_dx_math::omnipool::calculate_tvl_cap_difference(
            &asset_state,
            &stable_asset_state,
            tvl_cap,
            total_hub_reserve,
        ) {
            result.to_string()
        } else {
            error()
        }
    }

    #[wasm_bindgen]
    pub fn calculate_liquidity_hub_in(
        asset_reserve: String,
        asset_hub_reserve: String,
        asset_shares: String,
        amount_in: String,
    ) -> String {
        let amount = parse_into!(u128, amount_in, error());
        let reserve = parse_into!(u128, asset_reserve, error());
        let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
        let shares = parse_into!(u128, asset_shares, error());

        let state = AssetReserveState {
            reserve,
            hub_reserve,
            shares,
            ..Default::default()
        };

        if let Some(state_changes) = hydra_dx_math::omnipool::calculate_add_liquidity_state_changes(
            &state,
            amount,
            I129 {
                value: 0u128,
                negative: false,
            },
            0u128,
        ) {
            (*state_changes.asset.delta_hub_reserve).to_string()
        } else {
            error()
        }
    }

    const SELL: u8 = 0b0000_0001;
    const BUY: u8 = 0b0000_0010;
    const ADD_LIQUIDITY: u8 = 0b0000_0100;
    const REMOVE_LIQUIDITY: u8 = 0b0000_1000;

    #[derive(Debug, Copy, Clone)]
    pub struct Tradability {
        bits: u8,
    }

    impl Tradability {
        pub fn new(bits: u8) -> Self {
            Self { bits }
        }
        pub fn can_sell(&self) -> bool {
            (self.bits & SELL) == SELL
        }
        pub fn can_buy(&self) -> bool {
            (self.bits & BUY) == BUY
        }
        pub fn can_add_liquidity(&self) -> bool {
            (self.bits & ADD_LIQUIDITY) == ADD_LIQUIDITY
        }

        pub fn can_remove_liquidity(&self) -> bool {
            (self.bits & REMOVE_LIQUIDITY) == REMOVE_LIQUIDITY
        }
    }

    #[wasm_bindgen]
    pub fn is_sell_allowed(bits: u8) -> bool {
        Tradability::new(bits).can_sell()
    }

    #[wasm_bindgen]
    pub fn is_buy_allowed(bits: u8) -> bool {
        Tradability::new(bits).can_buy()
    }

    #[wasm_bindgen]
    pub fn is_add_liquidity_allowed(bits: u8) -> bool {
        Tradability::new(bits).can_add_liquidity()
    }

    #[wasm_bindgen]
    pub fn is_remove_liquidity_allowed(bits: u8) -> bool {
        Tradability::new(bits).can_remove_liquidity()
    }

    #[test]
    fn tradability_should_work_correctly() {
        let t = Tradability::new(15);
        assert!(t.can_sell());
        assert!(t.can_buy());
        assert!(t.can_add_liquidity());
        assert!(t.can_remove_liquidity());

        let t = Tradability::new(1);
        assert!(t.can_sell());
        assert!(!t.can_buy());
        assert!(!t.can_add_liquidity());
        assert!(!t.can_remove_liquidity());

        let t = Tradability::new(3);
        assert!(t.can_sell());
        assert!(t.can_buy());
        assert!(!t.can_add_liquidity());
        assert!(!t.can_remove_liquidity());

        let t = Tradability::new(4);
        assert!(!t.can_sell());
        assert!(!t.can_buy());
        assert!(t.can_add_liquidity());
        assert!(!t.can_remove_liquidity());

        let t = Tradability::new(7);
        assert!(t.can_sell());
        assert!(t.can_buy());
        assert!(t.can_add_liquidity());
        assert!(!t.can_remove_liquidity());

        let t = Tradability::new(8);
        assert!(!t.can_sell());
        assert!(!t.can_buy());
        assert!(!t.can_add_liquidity());
        assert!(t.can_remove_liquidity());

        let t = Tradability::new(0);
        assert!(!t.can_sell());
        assert!(!t.can_buy());
        assert!(!t.can_add_liquidity());
        assert!(!t.can_remove_liquidity());
    }

    #[test]
    fn recalculate_asset_fee_should_work_correctly() {
        let result = recalculate_asset_fee(
            "25".to_string(),
            "20".to_string(),
            "1000".to_string(),
            "0.1".to_string(),
            "1".to_string(),
            "0.01".to_string(),
            "0.3".to_string(),
            "0".to_string(),
            "2000000000000000000".to_string(),
        );
        assert_eq!(result, "0.09");
    }

    #[test]
    fn recalculate_protocol_fee_should_work_correctly() {
        let result = recalculate_protocol_fee(
            "25".to_string(),
            "20".to_string(),
            "1000".to_string(),
            "0.1".to_string(),
            "1".to_string(),
            "0.01".to_string(),
            "0.3".to_string(),
            "0".to_string(),
            "2000000000000000000".to_string(),
        );
        assert_eq!(result, "0.11");
    }

    #[test]
    fn calculate_spot_price_should_work() {
        let result = calculate_spot_price(
            "2000".to_string(),
            "500".to_string(),
            "1000".to_string(),
            "125".to_string(),
        );
        assert_eq!(result, "2000000000000000000");

        let result = calculate_spot_price(
            "2000".to_string(),
            "0".to_string(),
            "1000".to_string(),
            "125".to_string(),
        );
        assert_eq!(result, "0");
    }

    #[test]
    fn calculate_spot_price_with_fee_should_work() {
        let result = calculate_spot_price_with_fee(
            "2000".to_string(),
            "500".to_string(),
            "1000".to_string(),
            "125".to_string(),
            "0.01".to_string(),
            "0.03".to_string(),
        );
        assert_eq!(result, "1920600000000000000");

        let result = calculate_spot_price_with_fee(
            "2000".to_string(),
            "500".to_string(),
            "0".to_string(),
            "125".to_string(),
            "0.01".to_string(),
            "0.03".to_string(),
        );
        assert_eq!(result, "0");
    }


    #[test]
    fn calculate_lrna_spot_price_should_work() {
        let result = calculate_lrna_spot_price(
            "2000".to_string(),
            "500".to_string(),
        );
        assert_eq!(result, "4000000000000000000");

        let result = calculate_lrna_spot_price(
            "2000".to_string(),
            "0".to_string(),
        );
        assert_eq!(result, "-1");
    }

    #[test]
    fn calculate_lrna_spot_price_with_fee_should_work() {
        let result = calculate_lrna_spot_price_with_fee(
            "2000".to_string(),
            "500".to_string(),
            "0.01".to_string(),
        );
        assert_eq!(result, "3960000000000000000");

        let result = calculate_lrna_spot_price_with_fee(
            "2000".to_string(),
            "0".to_string(),
            "0.01".to_string(),
        );
        assert_eq!(result, "-1");
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

#[cfg(feature = "ema")]
pub mod ema {
    pub use super::*;
    use hydra_dx_math::ema::EmaPrice;
    use hydra_dx_math::types::{Balance, Fraction};
    use sp_arithmetic::FixedU128;

    /// Calculate the iterated exponential moving average for the given prices.
    /// + `iterations` is the number of iterations of the EMA to calculate (expected to be a serialized `u32`).
    /// + `prev_n` and `prev_d` are the previous oracle value, `incoming_n` and `incoming_d` are the new value to
    ///   integrate (expected to be serialized `u128` values).
    /// + `smoothing` is the smoothing factor of the EMA (expected to be a serialized `u128` that gets interpreted as a
    ///   `Fraction`).
    ///
    /// Returns the new oracle value as a serialized `FixedU128` (lower precision than the input).
    #[wasm_bindgen]
    pub fn low_precision_iterated_price_ema(
        iterations: String,
        prev_n: String,
        prev_d: String,
        incoming_n: String,
        incoming_d: String,
        smoothing: String,
    ) -> String {
        let Ok(iterations) = iterations.parse::<u32>() else { return error() };
        let Ok(prev_n) = prev_n.parse::<u128>() else { return error() };
        let Ok(prev_d) = prev_d.parse::<u128>() else { return error() };
        let prev = EmaPrice::new(prev_n, prev_d);
        let Ok(incoming_n) = incoming_n.parse::<u128>() else { return error() };
        let Ok(incoming_d) = incoming_d.parse::<u128>() else { return error() };
        let incoming = EmaPrice::new(incoming_n, incoming_d);
        let Ok(smoothing) = smoothing.parse::<u128>().map(Fraction::from_bits) else { return error() };
        let price = hydra_dx_math::ema::iterated_price_ema(iterations, prev, incoming, smoothing);
        FixedU128::from_rational(price.n, price.d).to_string()
    }

    /// Calculate the iterated exponential moving average for the given balances.
    /// + `iterations` is the number of iterations of the EMA to calculate (expected to be a serialized `u32`).
    /// + `prev` is the previous oracle value, `incoming` is the new value to integrate (expected to be serialized
    ///   `u128` values).
    /// + `smoothing` is the smoothing factor of the EMA (expected to be a serialized `u128` that gets interpreted as a
    ///   `Fraction`).
    ///
    /// Returns the new oracle value as a serialized `u128`.
    #[wasm_bindgen]
    pub fn iterated_balance_ema(iterations: String, prev: String, incoming: String, smoothing: String) -> String {
        let Ok(iterations) = iterations.parse::<u32>() else { return error() };
        let Ok(prev) = prev.parse::<Balance>() else { return error() };
        let Ok(incoming) = incoming.parse::<Balance>() else { return error() };
        let Ok(smoothing) = smoothing.parse::<u128>().map(Fraction::from_bits) else { return error() };
        let balance = hydra_dx_math::ema::iterated_balance_ema(iterations, prev, incoming, smoothing);
        balance.to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use hydra_dx_math::ema::smoothing_from_period;

        #[test]
        fn iterated_price_ema_should_work() {
            let smoothing = smoothing_from_period(7);
            let start_price = EmaPrice::new(4, 1);
            let incoming_price = EmaPrice::new(8, 1);
            let next_price = low_precision_iterated_price_ema(
                1u32.to_string(),
                start_price.n.to_string(),
                start_price.d.to_string(),
                incoming_price.n.to_string(),
                incoming_price.d.to_string(),
                smoothing.to_bits().to_string(),
            );
            let expected = FixedU128::from((5, 1)).to_string();
            assert_eq!(next_price, expected);
        }

        #[test]
        fn iterated_balance_ema_should_work() {
            let smoothing = smoothing_from_period(7);
            let start = 4;
            let incoming = 8;
            let res = iterated_balance_ema(
                1u32.to_string(),
                start.to_string(),
                incoming.to_string(),
                smoothing.to_bits().to_string(),
            );
            let expected = 5;
            let res = res.parse::<u128>().unwrap();
            assert_eq!(res, expected);
        }
    }
}

#[cfg(feature = "staking")]
pub mod staking {
    use super::*;
    use sp_arithmetic::{FixedU128, Perbill, Permill};

    macro_rules! parse_into {
        ($x:ty, $y:expr, $e:expr) => {{
            let r = if let Some(x) = $y.parse::<$x>().ok() {
                x
            } else {
                return $e;
            };
            r
        }};
    }

    #[wasm_bindgen]
    pub fn calculate_accumulated_rps(
        current_reward_per_stake: String,
        pending_rewards: String,
        total_stake: String,
    ) -> String {
        let current_rps = FixedU128::from_inner(to_u128!(current_reward_per_stake));
        let (pending_rewards, total_stake) = to_u128!(pending_rewards, total_stake);

        match hydra_dx_math::staking::calculate_accumulated_rps(current_rps, pending_rewards, total_stake) {
            Some(rps) => rps.to_string(),
            None => error(),
        }
    }

    #[wasm_bindgen]
    pub fn calculate_slashed_points(
        points: String,
        current_stake: String,
        stake_increase: String,
        stake_weight: String,
        min_slash_point: String
    ) -> String {
        let (points, current_stake, stake_increase, min_slash_point) = to_u128!(points, current_stake, stake_increase, min_slash_point);
        let stake_weight = stake_weight.parse::<u8>().unwrap_or(0);

        match hydra_dx_math::staking::calculate_slashed_points(points, current_stake, stake_increase, stake_weight, min_slash_point) {
            Some(slashed) => slashed.to_string(),
            None => error(),
        }
    }

    #[wasm_bindgen]
    pub fn calculate_period_number(period_length: String, block_number: String) -> String {
        let (period_length, block_number) = to_u128!(period_length, block_number);
        let period_length = match std::num::NonZeroU128::try_from(period_length) {
            Ok(v) => v,
            Err(_) => return error(),
        };

        hydra_dx_math::staking::calculate_period_number(period_length, block_number).to_string()
    }

    #[wasm_bindgen]
    pub fn calculate_points(
        position_created_at: String,
        now: String,
        time_points_per_period: String,
        time_points_weight: String,
        action_points: String,
        action_points_weight: String,
        slashed_points: String,
    ) -> String {
        let (position_created_at, now, action_points, slashed_points) =
            to_u128!(position_created_at, now, action_points, slashed_points);
        let time_points_per_period = time_points_per_period.parse::<u8>().unwrap_or(0);
        let time_points_weight = Permill::from_float(parse_into!(f64, time_points_weight, error()));
        let action_points_weight = Perbill::from_float(parse_into!(f64, action_points_weight, error()));

        match hydra_dx_math::staking::calculate_points(
            position_created_at,
            now,
            time_points_per_period,
            time_points_weight,
            action_points,
            action_points_weight,
            slashed_points,
        ) {
            Some(v) => v.to_string(),
            None => error(),
        }
    }

    #[wasm_bindgen]
    pub fn sigmoid(x: String, a: String, b: String) -> String {
        let (x, a) = to_u128!(x, a);
        let a = FixedU128::from_inner(a);
        let b = b.parse::<u32>().unwrap_or(0);

        match hydra_dx_math::staking::sigmoid(x, a, b) {
            Some(v) => v.to_string(),
            None => error(),
        }
    }

    #[wasm_bindgen]
    pub fn calculate_rewards(accumulated_reward_per_stake: String, reward_per_stake: String, stake: String) -> String {
        let (accumulated_rps, rps, stake) = to_u128!(accumulated_reward_per_stake, reward_per_stake, stake);
        let accumulated_rps = FixedU128::from_inner(accumulated_rps);
        let rps = FixedU128::from_inner(rps);

        match hydra_dx_math::staking::calculate_rewards(accumulated_rps, rps, stake) {
            Some(v) => v.to_string(),
            None => error(),
        }
    }

    #[wasm_bindgen]
    pub fn calculate_percentage_amount(amount: String, percentage: String) -> String {
        let (amount, percentage) = to_u128!(amount, percentage);
        let percentage = FixedU128::from_inner(percentage);

        hydra_dx_math::staking::calculate_percentage_amount(amount, percentage).to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn calculate_accumulated_rps_should_work() {
            let rps_now = FixedU128::from((1_234_512_341_u128, 10_000_000_u128)).to_string();

            let calculated = calculate_accumulated_rps(
                rps_now,
                10_000_000_000_000_000_000_000_u128.to_string(),
                987_886_878_000_000_000_000_u128.to_string(),
            );

            let expected = FixedU128::from_inner(133_573_850_588_484_220_963_u128).to_string();
            assert_eq!(calculated, expected);
        }

        #[test]
        fn calculate_slashed_points_should_work() {
            let points = 10_000_000.to_string();

            assert_eq!(
                calculate_slashed_points(
                    points.clone(),
                    1_000_000_000_000_000_u128.to_string(),
                    1_000_000_000_000_000_u128.to_string(),
                    1.to_string(),
                    0.to_string()
                ),
                10_000_000.to_string()
            );

            assert_eq!(
                calculate_slashed_points(
                    points.clone(),
                    1_000_000_000_000_000_u128.to_string(),
                    1_000_000_000_000_000_u128.to_string(),
                    2.to_string(),
                    0.to_string()
                ),
                5_000_000.to_string()
            );

            assert_eq!(
                calculate_slashed_points(
                    points,
                    10_000_000_000_000_000_000_u128.to_string(),
                    1_000_000_000_000_u128.to_string(),
                    1.to_string(),
                    0.to_string()
                ),
                1.to_string()
            );

            assert_eq!(
                calculate_slashed_points(
                    0.to_string(),
                    1_000_000_000_000_000_u128.to_string(),
                    1_000_000_000_000_000_000_000_u128.to_string(),
                    1.to_string(),
                    0.to_string()
                ),
                0.to_string()
            );
        }

        #[test]
        fn calculate_period_number_should_work() {
            assert_eq!(
                calculate_period_number(1_u128.to_string(), 12_341_u128.to_string()),
                12_341_u128.to_string()
            );

            assert_eq!(
                calculate_period_number(1_000_u128.to_string(), 12_341_u128.to_string()),
                12_u128.to_string()
            );

            assert_eq!(
                calculate_period_number(1_000_u128.to_string(), 1_u128.to_string()),
                0_u128.to_string()
            );

            assert_eq!(
                calculate_period_number(82_u128.to_string(), 12_341_u128.to_string()),
                150_u128.to_string()
            );
        }

        #[test]
        fn calculate_points_should_work() {
            let time_points_per_period = 2_u8.to_string();
            let action_points = 100_u128.to_string();

            assert_eq!(
                calculate_points(
                    39_u128.to_string(),
                    42_u128.to_string(),
                    time_points_per_period.clone(),
                    "0.6".to_string(),
                    action_points,
                    "0.4".to_string(),
                    0.to_string()
                ),
                43.to_string()
            );

            let action_points = 0_u128.to_string();
            assert_eq!(
                calculate_points(
                    40_u128.to_string(),
                    82_u128.to_string(),
                    time_points_per_period.clone(),
                    "0.6".to_string(),
                    action_points,
                    "0.4".to_string(),
                    0.to_string()
                ),
                50.to_string()
            );

            let action_points = 1_000_000_u128.to_string();
            assert_eq!(
                calculate_points(
                    150_u128.to_string(),
                    192_u128.to_string(),
                    time_points_per_period,
                    "0.8".to_string(),
                    action_points,
                    "0.1".to_string(),
                    200.to_string()
                ),
                99_867.to_string()
            );
        }

        #[test]
        fn sigmoid_should_work() {
            let a = 8_000_000_000_000_000_u128.to_string();
            let b = 2.to_string();

            assert_eq!(
                sigmoid(0.to_string(), a.clone(), b.clone()),
                FixedU128::from(0_u128).to_string()
            );

            assert_eq!(
                sigmoid(1.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(2_047_999_995_u128).to_string()
            );

            assert_eq!(
                sigmoid(10.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(20_479_580_578_189_u128).to_string()
            );

            assert_eq!(
                sigmoid(538.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(994_205_484_888_725_524_u128).to_string()
            );

            assert_eq!(
                sigmoid(1_712_904.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(999_999_999_999_999_943_u128).to_string()
            );

            let a = 250_000_000_000_000_000_u128.to_string();
            let b = 9_340_000.to_string();

            assert_eq!(
                sigmoid(0.to_string(), a.clone(), b.clone()),
                FixedU128::from(0).to_string()
            );

            assert_eq!(
                sigmoid(1.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(418_228_051_u128).to_string()
            );

            assert_eq!(
                sigmoid(10.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(4_182_263_022_521_u128).to_string()
            );

            assert_eq!(
                sigmoid(538.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(972_251_695_722_892_328_u128).to_string()
            );

            assert_eq!(
                sigmoid(500_000.to_string(), a.clone(), b.clone()),
                FixedU128::from_inner(999_999_999_999_961_743_u128).to_string()
            );
        }

        #[test]
        fn calculate_rewards_should_work() {
            let accumulated_rps = 23_423_523_230_000_000_000_u128.to_string();
            let rps = 23_423_000_000_000_000_000_u128.to_string();
            let amount = 1_000_000_000_000_000_u128.to_string();

            assert_eq!(
                calculate_rewards(accumulated_rps, rps, amount.clone()),
                523_230_000_000_u128.to_string()
            );

            let accumulated_rps = 23_423_523_230_000_000_000_u128.to_string();
            let rps = 19_423_000_000_000_000_000_u128.to_string();
            assert_eq!(
                calculate_rewards(accumulated_rps, rps, amount).to_string(),
                4_000_523_230_000_000_u128.to_string()
            );
        }

        #[test]
        fn calculate_percentage_amount_work() {
            assert_eq!(
                calculate_percentage_amount(3_000_000_u128.to_string(), 500_000_000_000_000_000_u128.to_string()),
                1_500_000_u128.to_string()
            );

            assert_eq!(
                calculate_percentage_amount(3_000_000_u128.to_string(), 0.to_string()),
                0.to_string()
            );

            assert_eq!(
                calculate_percentage_amount(3_000_000_u128.to_string(), 1_000_000_000_000_000_000_u128.to_string()),
                3_000_000_u128.to_string()
            );

            assert_eq!(
                calculate_percentage_amount(
                    3_000_000_u128.to_string(),
                    FixedU128::from_float(0.13264959).into_inner().to_string()
                ),
                397_948_u128.to_string()
            );
        }
    }
}
