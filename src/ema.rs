pub use super::*;
use hydra_dx_math::ema::EmaPrice;
use hydra_dx_math::types::{Balance, Fraction};
use sp_arithmetic::FixedU128;
use wasm_bindgen::prelude::*;

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
    let Ok(iterations) = iterations.parse::<u32>() else {
        return error();
    };
    let Ok(prev_n) = prev_n.parse::<u128>() else {
        return error();
    };
    let Ok(prev_d) = prev_d.parse::<u128>() else {
        return error();
    };
    let prev = EmaPrice::new(prev_n, prev_d);
    let Ok(incoming_n) = incoming_n.parse::<u128>() else {
        return error();
    };
    let Ok(incoming_d) = incoming_d.parse::<u128>() else {
        return error();
    };
    let incoming = EmaPrice::new(incoming_n, incoming_d);
    let Ok(smoothing) = smoothing.parse::<u128>().map(Fraction::from_bits) else {
        return error();
    };
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
    let Ok(iterations) = iterations.parse::<u32>() else {
        return error();
    };
    let Ok(prev) = prev.parse::<Balance>() else {
        return error();
    };
    let Ok(incoming) = incoming.parse::<Balance>() else {
        return error();
    };
    let Ok(smoothing) = smoothing.parse::<u128>().map(Fraction::from_bits) else {
        return error();
    };
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
