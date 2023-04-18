extern crate core;

use wasm_bindgen::prelude::*;

macro_rules! to_u128 {
    ($($x:expr),+) => (
        {($($x.parse::<u128>().unwrap_or(0)),+)}
    );
}

fn error() -> String {
    "-1".to_string()
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
    fn outin_price_works() {
        assert_eq!(
            lbp::calculate_out_given_in(
                String::from("1000"),
                String::from("2000"),
                String::from("1000"),
                String::from("2000"),
                String::from("500"),
            ),
            "358"
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
            "782"
        );
        assert_eq!(
            lbp::calculate_in_given_out(
                String::from("0"),
                String::from("1"),
                String::from("1000"),
                String::from("2000"),
                String::from("0"),
            ),
            "9"
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

/*
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
        let result = hydra_dx_math::stableswap::calculate_in_given_out::<D_ITERATIONS, Y_ITERATIONS>(
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
*/

#[cfg(feature = "liquidity-mining")]
pub mod liquidity_mining {
    pub use super::*;
    use sp_arithmetic::fixed_point::FixedU128;

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
}

#[cfg(feature = "omnipool-deprecated")]
pub mod omnipool {
    pub use super::*;
    use hydra_dx_math::omnipool::types::{AssetReserveState, Position as OmnipoolPosition, I129};
    use sp_arithmetic::{FixedU128, Permill};

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
    pub struct MathResult {
        result: String,
        error: bool,
    }

    #[wasm_bindgen]
    impl MathResult {
        pub fn get_result(&self) -> String {
            self.result.clone()
        }

        pub fn is_error(&self) -> bool {
            self.error
        }
    }

    impl MathResult {
        fn error() -> Self {
            MathResult {
                result: "".to_string(),
                error: true,
            }
        }
    }

    #[wasm_bindgen]
    pub struct LiquidityOutResult {
        amount: String,
        lrna_amount: String,
        error: bool,
    }

    #[wasm_bindgen]
    impl LiquidityOutResult {
        pub fn get_asset_amount(&self) -> String {
            self.amount.clone()
        }

        pub fn get_lrna_amount(&self) -> String {
            self.lrna_amount.clone()
        }

        pub fn is_error(&self) -> bool {
            self.error
        }
    }

    impl LiquidityOutResult {
        fn error() -> Self {
            LiquidityOutResult {
                amount: "".to_string(),
                lrna_amount: "".to_string(),
                error: true,
            }
        }
    }

    #[wasm_bindgen]
    pub struct AssetState {
        reserve: String,
        hub_reserve: String,
        shares: String,
    }

    #[wasm_bindgen]
    impl AssetState {
        #[wasm_bindgen(constructor)]
        pub fn new(reserve: String, hub_reserve: String, shares: String) -> Self {
            Self {
                reserve,
                hub_reserve,
                shares,
            }
        }
    }

    impl TryFrom<AssetState> for AssetReserveState<u128> {
        type Error = ();

        fn try_from(value: AssetState) -> Result<Self, Self::Error> {
            let reserve = value.reserve.parse::<u128>().map_err(|_| ())?;
            let hub_reserve = value.hub_reserve.parse::<u128>().map_err(|_| ())?;
            let shares = value.shares.parse::<u128>().map_err(|_| ())?;

            Ok(Self {
                reserve,
                hub_reserve,
                shares,
                ..Default::default()
            })
        }
    }

    #[wasm_bindgen]
    pub struct Position {
        amount: String,
        shares: String,
        price: String,
    }

    #[wasm_bindgen]
    impl Position {
        #[wasm_bindgen(constructor)]
        pub fn new(amount: String, shares: String, price: String) -> Self {
            Self { amount, shares, price }
        }
    }

    impl TryFrom<Position> for OmnipoolPosition<u128> {
        type Error = ();

        fn try_from(value: Position) -> Result<Self, Self::Error> {
            let amount = value.amount.parse::<u128>().map_err(|_| ())?;
            let shares = value.shares.parse::<u128>().map_err(|_| ())?;
            let price = value.price.parse::<FixedU128>().map_err(|_| ())?;

            Ok(Self { amount, shares, price })
        }
    }

    #[wasm_bindgen]
    pub fn calculate_shares(asset_state: AssetState, amount_in: String) -> MathResult {
        let amount = parse_into!(u128, amount_in, MathResult::error());

        let state: AssetReserveState<u128> = if let Ok(value) = asset_state.try_into() {
            value
        } else {
            return MathResult::error();
        };

        let state_changes = if let Some(r) = hydra_dx_math::omnipool::calculate_add_liquidity_state_changes(
            &state,
            amount,
            I129 {
                value: 0u128,
                negative: false,
            },
            0u128,
        ) {
            r
        } else {
            return MathResult::error();
        };

        MathResult {
            result: (*state_changes.asset.delta_shares).to_string(),
            error: false,
        }
    }

    #[wasm_bindgen]
    pub fn calculate_liquidity_out(asset_state: AssetState, position: Position, shares: String) -> LiquidityOutResult {
        let shares_amount = parse_into!(u128, shares, LiquidityOutResult::error());

        let state: AssetReserveState<u128> = if let Ok(value) = asset_state.try_into() {
            value
        } else {
            return LiquidityOutResult::error();
        };

        let position: OmnipoolPosition<u128> = if let Ok(value) = position.try_into() {
            value
        } else {
            return LiquidityOutResult::error();
        };

        let state_changes = if let Some(r) = hydra_dx_math::omnipool::calculate_remove_liquidity_state_changes(
            &state,
            shares_amount,
            &position,
            I129 {
                value: 0u128,
                negative: false,
            },
            0u128,
        ) {
            r
        } else {
            return LiquidityOutResult::error();
        };

        LiquidityOutResult {
            amount: (*state_changes.asset.delta_shares).to_string(),
            lrna_amount: state_changes.lp_hub_amount.to_string(),
            error: false,
        }
    }

    #[wasm_bindgen]
    pub fn calculate_out_given_in(
        asset_in_state: AssetState,
        asset_out_state: AssetState,
        amount_in: String,
        asset_fee: String,
        protocol_fee: String,
    ) -> MathResult {
        let amount = parse_into!(u128, amount_in, MathResult::error());
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, MathResult::error()));
        let protocol_fee = Permill::from_float(parse_into!(f64, protocol_fee, MathResult::error()));

        let asset_in: AssetReserveState<u128> = if let Ok(value) = asset_in_state.try_into() {
            value
        } else {
            return MathResult::error();
        };
        let asset_out: AssetReserveState<u128> = if let Ok(value) = asset_out_state.try_into() {
            value
        } else {
            return MathResult::error();
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
            return MathResult::error();
        };

        MathResult {
            result: (*state_changes.asset_out.delta_reserve).to_string(),
            error: false,
        }
    }

    #[wasm_bindgen]
    pub fn calculate_in_given_out(
        asset_in_state: AssetState,
        asset_out_state: AssetState,
        amount_out: String,
        asset_fee: String,
        protocol_fee: String,
    ) -> MathResult {
        let amount = parse_into!(u128, amount_out, MathResult::error());
        let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, MathResult::error()));
        let protocol_fee = Permill::from_float(parse_into!(f64, protocol_fee, MathResult::error()));

        let asset_in: AssetReserveState<u128> = if let Ok(value) = asset_in_state.try_into() {
            value
        } else {
            return MathResult::error();
        };
        let asset_out: AssetReserveState<u128> = if let Ok(value) = asset_out_state.try_into() {
            value
        } else {
            return MathResult::error();
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
            return MathResult::error();
        };

        MathResult {
            result: (*state_changes.asset_in.delta_reserve).to_string(),
            error: false,
        }
    }

    const SELL: u8 = 0b0000_0001;
    const BUY: u8 = 0b0000_0010;
    const ADD_LIQUIDITY: u8 = 0b0000_0100;
    const REMOVE_LIQUIDITY: u8 = 0b0000_1000;

    #[wasm_bindgen]
    #[derive(Debug, Copy, Clone)]
    pub struct Tradability {
        bits: u8,
    }

    #[wasm_bindgen]
    impl Tradability {
        #[wasm_bindgen(constructor)]
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
}

#[cfg(feature = "omnipool")]
pub mod omnipool {
    pub use super::*;
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

        if let Some(result) = hydra_dx_math::omnipool::calculate_spot_sprice(&asset_a, &asset_b) {
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

        if let Some(result) = hydra_dx_math::omnipool::calculate_lrna_spot_sprice(&asset) {
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
