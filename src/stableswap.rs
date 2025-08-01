use crate::utils::error;
use crate::{to_u128, to_u32};
use hydra_dx_math::stableswap::types::AssetReserve;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use serde::Deserialize;
use sp_arithmetic::{FixedPointNumber, Permill};
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

fn default_pegs(size: usize) -> Vec<(u128, u128)> {
    let mut pegs = Vec::with_capacity(size);
    for _ in 0..size {
        pegs.push((1, 1));
    }
    pegs
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
    let pegs = default_pegs(balances.len());

    let result = hydra_dx_math::stableswap::calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        idx_in.unwrap(),
        idx_out.unwrap(),
        amount_in,
        amplification,
        fee,
        &pegs,
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
    let pegs = default_pegs(balances.len());

    let result = hydra_dx_math::stableswap::calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        idx_in.unwrap(),
        idx_out.unwrap(),
        amount_out,
        amplification,
        fee,
        &pegs,
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

    let pegs = default_pegs(balances.len());

    let result = hydra_dx_math::stableswap::calculate_shares::<D_ITERATIONS>(
        &balances,
        &updated_balances,
        amplification,
        issuance,
        fee,
        &pegs,
    );

    if let Some(r) = result {
        r.0.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_spot_price_with_fee(
    pool_id: String,
    reserves: String,
    amplification: String,
    asset_in: String,
    asset_out: String,
    share_issuance: String,
    fee: String,
) -> String {
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);

    let balances: Vec<(u32, AssetReserve)> = reserves
        .clone()
        .into_iter()
        .map(|v| (v.asset_id, AssetReserve::new(v.amount, v.decimals)))
        .collect();
    let amplification = parse_into!(u128, amplification);
    let (pool_id, asset_in, asset_out) = to_u32!(pool_id, asset_in, asset_out);
    let min_trade_limit = 1_000; //We use the same MinTradingLimit we have configured to stableswap runtime
    let fee = Permill::from_float(parse_into!(f64, fee));
    let issuance = parse_into!(u128, share_issuance);

    let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
    let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);
    if idx_in.is_none() && idx_out.is_none() {
        return error();
    }

    let pegs = default_pegs(balances.len());

    let result = hydra_dx_math::stableswap::calculate_spot_price(
        pool_id,
        balances,
        amplification,
        asset_in,
        asset_out,
        issuance,
        min_trade_limit,
        Some(fee),
        &pegs,
    );

    if let Some(r) = result {
        //Temp fix to return data correctly, reserve it when this issue `https://github.com/galacticcouncil/hydration-node/issues/1009` is fixed in runtime
        if let Some(price) = r.reciprocal() {
            price.to_string()
        } else {
            error()
        }
    } else {
        error()
    }
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

    let pegs = default_pegs(balances.len());

    let result = hydra_dx_math::stableswap::calculate_shares_for_amount::<D_ITERATIONS>(
        &balances,
        idx_in.unwrap(),
        amount_in,
        amplification,
        issuance,
        fee,
        &pegs,
    );

    if let Some(r) = result {
        r.0.to_string()
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
    let pegs = default_pegs(balances.len());

    let result = hydra_dx_math::stableswap::calculate_add_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        shares,
        idx_in.unwrap(),
        issuance,
        amplification,
        fee,
        &pegs,
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
    let pegs = default_pegs(balances.len());

    let result = hydra_dx_math::stableswap::calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        shares_out,
        idx_out.unwrap(),
        issuance,
        amplification,
        fee,
        &pegs,
    );

    if let Some(r) = result {
        r.0.to_string()
    } else {
        error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            100000002.to_string(),
            data.to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
            "555555".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "27466702117023532371705781081".to_string());

        let result = calculate_spot_price_with_fee(
            100000002.to_string(),
            "0".to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
            "555555".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "-1".to_string());
    }

    #[test]
    fn calculate_spot_price_between_share_and_stable_with_fee_should_work() {
        let data = r#"
        [{
            "asset_id": 1,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 2,
            "amount":"400000000000",
            "decimals": 12
        }
        ]"#;

        let result = calculate_spot_price_with_fee(
            0.to_string(),
            data.to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
            "2000000000".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "200000000000000000000".to_string());

        let result = calculate_spot_price_with_fee(
            1.to_string(),
            data.to_string(),
            100.to_string(),
            "9".to_string(),
            "0".to_string(),
            "2000000000".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "-1".to_string());
    }

    #[test]
    fn calculate_spot_price_between_stable_and_share_with_fee_should_work() {
        let data = r#"
        [{
            "asset_id": 2,
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
            0.to_string(),
            data.to_string(),
            100.to_string(),
            "0".to_string(),
            "1".to_string(),
            "648395944517198603232".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "11494252873563218390".to_string());

        let result = calculate_spot_price_with_fee(
            0.to_string(),
            data.to_string(),
            100.to_string(),
            "0".to_string(),
            "9999".to_string(),
            "648395944517198603232".to_string(),
            "0.01".to_string(),
        );

        assert_eq!(result, "-1".to_string());
    }
}
