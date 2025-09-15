use wasm_bindgen::prelude::*;

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
    min_slash_point: String,
) -> String {
    let (points, current_stake, stake_increase, min_slash_point) =
        to_u128!(points, current_stake, stake_increase, min_slash_point);
    let stake_weight = stake_weight.parse::<u8>().unwrap_or(0);

    match hydra_dx_math::staking::calculate_slashed_points(
        points,
        current_stake,
        stake_increase,
        stake_weight,
        min_slash_point,
    ) {
        Some(slashed) => slashed.to_string(),
        None => error(),
    }
}

#[wasm_bindgen]
pub fn calculate_period_number(period_length: String, block_number: String, six_sec_block_since: String) -> String {
    let (period_length, block_number, six_sec_block_since) = to_u128!(period_length, block_number, six_sec_block_since);
    let period_length = match std::num::NonZeroU128::try_from(period_length) {
        Ok(v) => v,
        Err(_) => return error(),
    };

    let six_sec_block_since = match std::num::NonZeroU128::try_from(six_sec_block_since) {
        Ok(v) => v,
        Err(_) => return error(),
    };

    hydra_dx_math::staking::calculate_period_number(period_length, block_number, six_sec_block_since).to_string()
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
            calculate_period_number(1_u128.to_string(), 12_341_u128.to_string(), 999_999u128.to_string()),
            12_341_u128.to_string()
        );

        assert_eq!(
            calculate_period_number(1_000_u128.to_string(), 12_341_u128.to_string(), 999_999u128.to_string()),
            12_u128.to_string()
        );

        assert_eq!(
            calculate_period_number(1_000_u128.to_string(), 1_u128.to_string(), 999_999u128.to_string()),
            0_u128.to_string()
        );

        assert_eq!(
            calculate_period_number(82_u128.to_string(), 12_341_u128.to_string(), 999_999u128.to_string()),
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
