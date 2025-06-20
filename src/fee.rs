use crate::to_u128;
use wasm_bindgen::prelude::*;

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
