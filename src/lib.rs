use sp_arithmetic::FixedPointNumber;
use sp_arithmetic::FixedU128;
use std::collections::HashMap;
use std::ffi::{CStr, CString};

extern crate libc;

fn error() -> *const libc::c_char {
    CString::new("-1".to_string()).unwrap().into_raw()
}

use hydra_dx_math::omnipool::types::AssetReserveState;
use sp_arithmetic::Permill;

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

#[macro_export]
macro_rules! to_u32 {
        ($($x:expr),+) => (
            {($($x.parse::<u32>().unwrap_or(0)),+)}
        );
    }

use hydra_dx_math::stableswap::types::AssetReserve;

use serde::Deserialize;
use serde_aux::prelude::*;

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
    #[serde(deserialize_with = "deserialize_number_from_string")]
    asset_id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    amount: u128,
}

#[no_mangle]
pub extern "C" fn op_calc_spot_price(
    asset_a_reserve: *const libc::c_char,
    asset_a_hub_reserve: *const libc::c_char,
    asset_b_reserve: *const libc::c_char,
    asset_b_hub_reserve: *const libc::c_char,
) -> *const libc::c_char {
    //TODO: handle unwraps
    let ar = std::str::from_utf8(unsafe { CStr::from_ptr(asset_a_reserve) }.to_bytes()).unwrap();
    let ahr = std::str::from_utf8(unsafe { CStr::from_ptr(asset_a_hub_reserve) }.to_bytes()).unwrap();
    let br = std::str::from_utf8(unsafe { CStr::from_ptr(asset_b_reserve) }.to_bytes()).unwrap();
    let bhr = std::str::from_utf8(unsafe { CStr::from_ptr(asset_b_hub_reserve) }.to_bytes()).unwrap();

    let reserve_a = parse_into!(u128, ar, error());
    let hub_reserve_a = parse_into!(u128, ahr, error());
    let reserve_b = parse_into!(u128, br, error());
    let hub_reserve_b = parse_into!(u128, bhr, error());

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

    if let Some(result) = hydra_dx_math::omnipool::calculate_spot_price(&asset_a, &asset_b, None) {
        CString::new(result.to_string()).unwrap().into_raw()
    } else {
        error()
    }
}

#[no_mangle]
pub extern "C" fn op_calc_buy_state_changes(
    asset_in_reserve: *const libc::c_char,
    asset_in_hub_reserve: *const libc::c_char,
    asset_out_reserve: *const libc::c_char,
    asset_out_hub_reserve: *const libc::c_char,
    amount_out: *const libc::c_char,
    asset_fee: *const libc::c_char,
    protocol_fee: *const libc::c_char,
    m: *const libc::c_char,
) -> *const libc::c_char {
    //TODO: handle unwraps
    let air = std::str::from_utf8(unsafe { CStr::from_ptr(asset_in_reserve) }.to_bytes()).unwrap();
    let aihr = std::str::from_utf8(unsafe { CStr::from_ptr(asset_in_hub_reserve) }.to_bytes()).unwrap();
    let aor = std::str::from_utf8(unsafe { CStr::from_ptr(asset_out_reserve) }.to_bytes()).unwrap();
    let aohr = std::str::from_utf8(unsafe { CStr::from_ptr(asset_out_hub_reserve) }.to_bytes()).unwrap();
    let ao = std::str::from_utf8(unsafe { CStr::from_ptr(amount_out) }.to_bytes()).unwrap();
    let af = std::str::from_utf8(unsafe { CStr::from_ptr(asset_fee) }.to_bytes()).unwrap();
    let pf = std::str::from_utf8(unsafe { CStr::from_ptr(protocol_fee) }.to_bytes()).unwrap();
    let m = std::str::from_utf8(unsafe { CStr::from_ptr(m) }.to_bytes()).unwrap();

    let e = error();

    let reserve_in = parse_into!(u128, air, e);
    let hub_reserve_in = parse_into!(u128, aihr, e);

    let reserve_out = parse_into!(u128, aor, e);
    let hub_reserve_out = parse_into!(u128, aohr, e);

    let amount = parse_into!(u128, ao, e);
    let asset_fee = Permill::from_rational(parse_into!(u32, af, e), 1_000_000);
    let protocol_fee = Permill::from_rational(parse_into!(u32, pf, e), 1_000_000);
    let m = Permill::from_rational(parse_into!(u32, m, e), 1_000_000);

    let asset_in = AssetReserveState {
        reserve: reserve_in,
        hub_reserve: hub_reserve_in,
        ..Default::default()
    };

    let asset_out = AssetReserveState {
        reserve: reserve_out,
        hub_reserve: hub_reserve_out,
        ..Default::default()
    };

    let state_changes = if let Some(r) =
        hydra_dx_math::omnipool::calculate_buy_state_changes(&asset_in, &asset_out, amount, asset_fee, protocol_fee, m)
    {
        r
    } else {
        return e;
    };

    CString::new(
        [
            (*state_changes.asset_in.delta_reserve).to_string(),
            (*state_changes.asset_in.delta_hub_reserve).to_string(),
            (*state_changes.asset_out.delta_reserve).to_string(),
            (*state_changes.asset_out.delta_hub_reserve).to_string(),
            (state_changes.fee.protocol_fee).to_string(),
            (state_changes.fee.asset_fee).to_string(),
        ]
        .join("#"),
    )
    .unwrap()
    .into_raw()
}

#[no_mangle]
pub extern "C" fn op_calc_sell_state_changes(
    asset_in_reserve: *const libc::c_char,
    asset_in_hub_reserve: *const libc::c_char,
    asset_out_reserve: *const libc::c_char,
    asset_out_hub_reserve: *const libc::c_char,
    amount_in: *const libc::c_char,
    asset_fee: *const libc::c_char,
    protocol_fee: *const libc::c_char,
    m: *const libc::c_char,
) -> *const libc::c_char {
    //TODO: handle unwraps
    let air = std::str::from_utf8(unsafe { CStr::from_ptr(asset_in_reserve) }.to_bytes()).unwrap();
    let aihr = std::str::from_utf8(unsafe { CStr::from_ptr(asset_in_hub_reserve) }.to_bytes()).unwrap();
    let aor = std::str::from_utf8(unsafe { CStr::from_ptr(asset_out_reserve) }.to_bytes()).unwrap();
    let aohr = std::str::from_utf8(unsafe { CStr::from_ptr(asset_out_hub_reserve) }.to_bytes()).unwrap();
    let ai = std::str::from_utf8(unsafe { CStr::from_ptr(amount_in) }.to_bytes()).unwrap();
    let af = std::str::from_utf8(unsafe { CStr::from_ptr(asset_fee) }.to_bytes()).unwrap();
    let pf = std::str::from_utf8(unsafe { CStr::from_ptr(protocol_fee) }.to_bytes()).unwrap();
    let m = std::str::from_utf8(unsafe { CStr::from_ptr(m) }.to_bytes()).unwrap();

    let e = error();

    let reserve_in = parse_into!(u128, air, e);
    let hub_reserve_in = parse_into!(u128, aihr, e);

    let reserve_out = parse_into!(u128, aor, e);
    let hub_reserve_out = parse_into!(u128, aohr, e);

    let amount = parse_into!(u128, ai, e);
    let asset_fee = Permill::from_rational(parse_into!(u32, af, e), 1_000_000);
    let protocol_fee = Permill::from_rational(parse_into!(u32, pf, e), 1_000_000);
    let m = Permill::from_rational(parse_into!(u32, m, e), 1_000_000);

    let asset_in = AssetReserveState {
        reserve: reserve_in,
        hub_reserve: hub_reserve_in,
        ..Default::default()
    };

    let asset_out = AssetReserveState {
        reserve: reserve_out,
        hub_reserve: hub_reserve_out,
        ..Default::default()
    };

    let state_changes = if let Some(r) =
        hydra_dx_math::omnipool::calculate_sell_state_changes(&asset_in, &asset_out, amount, asset_fee, protocol_fee, m)
    {
        r
    } else {
        return e;
    };

    CString::new(
        [
            (*state_changes.asset_in.delta_reserve).to_string(),
            (*state_changes.asset_in.delta_hub_reserve).to_string(),
            (*state_changes.asset_out.delta_reserve).to_string(),
            (*state_changes.asset_out.delta_hub_reserve).to_string(),
            (state_changes.fee.protocol_fee).to_string(),
            (state_changes.fee.asset_fee).to_string(),
        ]
        .join("#"),
    )
    .unwrap()
    .into_raw()
}

#[no_mangle]
pub extern "C" fn sswap_calc_spot_price(
    pool_id: *const libc::c_char,
    reserves: *const libc::c_char,
    amplification: *const libc::c_char,
    asset_in: *const libc::c_char,
    asset_out: *const libc::c_char,
    share_issuance: *const libc::c_char,
    fee: *const libc::c_char,
    trade_amount: *const libc::c_char,
) -> *const libc::c_char {
    let pool_id = std::str::from_utf8(unsafe { CStr::from_ptr(pool_id) }.to_bytes()).unwrap();
    let reserves = std::str::from_utf8(unsafe { CStr::from_ptr(reserves) }.to_bytes()).unwrap();
    let amplification = std::str::from_utf8(unsafe { CStr::from_ptr(amplification) }.to_bytes()).unwrap();
    let asset_in = std::str::from_utf8(unsafe { CStr::from_ptr(asset_in) }.to_bytes()).unwrap();
    let asset_out = std::str::from_utf8(unsafe { CStr::from_ptr(asset_out) }.to_bytes()).unwrap();
    let share_issuance = std::str::from_utf8(unsafe { CStr::from_ptr(share_issuance) }.to_bytes()).unwrap();
    let fee = std::str::from_utf8(unsafe { CStr::from_ptr(fee) }.to_bytes()).unwrap();
    let trade_amount = std::str::from_utf8(unsafe { CStr::from_ptr(trade_amount) }.to_bytes()).unwrap();

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
    let amplification = parse_into!(u128, amplification, error());
    let (pool_id, asset_in, asset_out) = to_u32!(pool_id, asset_in, asset_out);
    let fee = Permill::from_rational(parse_into!(u32, fee, error()), 1_000_000);
    let issuance = parse_into!(u128, share_issuance, error());
    let trade_amount = parse_into!(u128, trade_amount, error());

    let fee = if fee.is_zero() { None } else { Some(fee) };

    //NOTE: stableswap returns spot price in differnt denomination than omnipool so I'm flipping it
    let result = hydra_dx_math::stableswap::calculate_spot_price(
        pool_id,
        balances,
        amplification,
        asset_in,
        asset_out,
        issuance,
        trade_amount,
        fee,
    )
    .and_then(|p| p.reciprocal());

    if let Some(result) = result {
        CString::new(result.to_string()).unwrap().into_raw()
    } else {
        error()
    }
}

#[no_mangle]
pub extern "C" fn sswap_calc_amplification(
    init_ampl: *const libc::c_char,
    final_ampl: *const libc::c_char,
    init_block: *const libc::c_char,
    final_block: *const libc::c_char,
    current_block: *const libc::c_char,
) -> *const libc::c_char {
    let ia = std::str::from_utf8(unsafe { CStr::from_ptr(init_ampl) }.to_bytes()).unwrap();
    let fa = std::str::from_utf8(unsafe { CStr::from_ptr(final_ampl) }.to_bytes()).unwrap();
    let ib = std::str::from_utf8(unsafe { CStr::from_ptr(init_block) }.to_bytes()).unwrap();
    let fb = std::str::from_utf8(unsafe { CStr::from_ptr(final_block) }.to_bytes()).unwrap();
    let cb = std::str::from_utf8(unsafe { CStr::from_ptr(current_block) }.to_bytes()).unwrap();

    let initial_amplification = parse_into!(u128, ia, error());
    let final_amplification = parse_into!(u128, fa, error());
    let initial_block = parse_into!(u128, ib, error());
    let final_block = parse_into!(u128, fb, error());
    let current_block = parse_into!(u128, cb, error());

    let r = hydra_dx_math::stableswap::calculate_amplification(
        initial_amplification,
        final_amplification,
        initial_block,
        final_block,
        current_block,
    );

    CString::new(r.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn sswap_calc_add_one_asset(
    reserves: *const libc::c_char,
    shares: *const libc::c_char,
    asset_in: *const libc::c_char,
    amplification: *const libc::c_char,
    share_issuance: *const libc::c_char,
    fee: *const libc::c_char,
) -> *const libc::c_char {
    let r = std::str::from_utf8(unsafe { CStr::from_ptr(reserves) }.to_bytes()).unwrap();
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&r);
    let shares = std::str::from_utf8(unsafe { CStr::from_ptr(shares) }.to_bytes()).unwrap();
    let amplification = std::str::from_utf8(unsafe { CStr::from_ptr(amplification) }.to_bytes()).unwrap();
    let share_issuance = std::str::from_utf8(unsafe { CStr::from_ptr(share_issuance) }.to_bytes()).unwrap();
    let fee = std::str::from_utf8(unsafe { CStr::from_ptr(fee) }.to_bytes()).unwrap();
    let asset_in = std::str::from_utf8(unsafe { CStr::from_ptr(asset_in) }.to_bytes()).unwrap();

    let shares = parse_into!(u128, shares, error());
    let amplification = parse_into!(u128, amplification, error());
    let issuance = parse_into!(u128, share_issuance, error());
    let fee = Permill::from_rational(parse_into!(u32, fee, error()), 1_000_000);
    let asset_in = parse_into!(u32, asset_in, error());

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

    let result = hydra_dx_math::stableswap::calculate_add_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        shares,
        idx_in.unwrap(),
        issuance,
        amplification,
        fee,
    );

    if let Some(r) = result {
        CString::new(r.0.to_string()).unwrap().into_raw()
    } else {
        error()
    }
}

#[no_mangle]
pub extern "C" fn sswap_calc_withdraw_one_asset(
    reserves: *const libc::c_char,
    shares: *const libc::c_char,
    asset_out: *const libc::c_char,
    amplification: *const libc::c_char,
    share_issuance: *const libc::c_char,
    fee: *const libc::c_char,
) -> *const libc::c_char {
    let r = std::str::from_utf8(unsafe { CStr::from_ptr(reserves) }.to_bytes()).unwrap();
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&r);
    let shares = std::str::from_utf8(unsafe { CStr::from_ptr(shares) }.to_bytes()).unwrap();
    let amplification = std::str::from_utf8(unsafe { CStr::from_ptr(amplification) }.to_bytes()).unwrap();
    let share_issuance = std::str::from_utf8(unsafe { CStr::from_ptr(share_issuance) }.to_bytes()).unwrap();
    let fee = std::str::from_utf8(unsafe { CStr::from_ptr(fee) }.to_bytes()).unwrap();
    let asset_out = std::str::from_utf8(unsafe { CStr::from_ptr(asset_out) }.to_bytes()).unwrap();

    let shares = parse_into!(u128, shares, error());
    let amplification = parse_into!(u128, amplification, error());
    let issuance = parse_into!(u128, share_issuance, error());
    let fee = Permill::from_rational(parse_into!(u32, fee, error()), 1_000_000);
    let asset_out = parse_into!(u32, asset_out, error());

    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);

    let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);
    if idx_out.is_none() {
        return error();
    }

    let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();

    let result = hydra_dx_math::stableswap::calculate_withdraw_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        shares,
        idx_out.unwrap(),
        issuance,
        amplification,
        fee,
    );

    if let Some(r) = result {
        CString::new([r.0.to_string(), r.1.to_string()].join("#"))
            .unwrap()
            .into_raw()
    } else {
        error()
    }
}

#[no_mangle]
pub extern "C" fn sswap_calc_share_price(
    reserves: *const libc::c_char,
    amplification: *const libc::c_char,
    issuance: *const libc::c_char,
    asset_id: *const libc::c_char,
) -> *const libc::c_char {
    let r = std::str::from_utf8(unsafe { CStr::from_ptr(reserves) }.to_bytes()).unwrap();
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&r);
    let amplification = std::str::from_utf8(unsafe { CStr::from_ptr(amplification) }.to_bytes()).unwrap();
    let issuance = std::str::from_utf8(unsafe { CStr::from_ptr(issuance) }.to_bytes()).unwrap();
    let asset_id = std::str::from_utf8(unsafe { CStr::from_ptr(asset_id) }.to_bytes()).unwrap();

    let amplification = parse_into!(u128, amplification, error());
    let issuance = parse_into!(u128, issuance, error());
    let asset_id = parse_into!(u32, asset_id, error());

    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);

    let idx_out = reserves.iter().position(|v| v.asset_id == asset_id);
    if idx_out.is_none() {
        return error();
    }

    let reserves: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();

    let d = hydra_dx_math::stableswap::calculate_d::<D_ITERATIONS>(&reserves, amplification);
    let result = hydra_dx_math::stableswap::calculate_share_price::<D_ITERATIONS>(
        &reserves,
        amplification,
        issuance,
        idx_out.unwrap(),
        d,
    );

    if let Some(r) = result {
        //WARN: this rounds mathematicaly, make sure it's correct
        CString::new(FixedU128::from_rational(r.0, r.1).to_string())
            .unwrap()
            .into_raw()
    } else {
        error()
    }
}

#[no_mangle]
pub extern "C" fn sswap_calc_shares(
    reserves: *const libc::c_char,
    assets: *const libc::c_char,
    amplification: *const libc::c_char,
    share_issuance: *const libc::c_char,
    fee: *const libc::c_char,
) -> *const libc::c_char {
    let reserves = std::str::from_utf8(unsafe { CStr::from_ptr(reserves) }.to_bytes()).unwrap();
    let assets = std::str::from_utf8(unsafe { CStr::from_ptr(assets) }.to_bytes()).unwrap();
    let amplification = std::str::from_utf8(unsafe { CStr::from_ptr(amplification) }.to_bytes()).unwrap();
    let share_issuance = std::str::from_utf8(unsafe { CStr::from_ptr(share_issuance) }.to_bytes()).unwrap();
    let fee = std::str::from_utf8(unsafe { CStr::from_ptr(fee) }.to_bytes()).unwrap();

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
    let amplification = parse_into!(u128, amplification, error());
    let issuance = parse_into!(u128, share_issuance, error());
    let fee = Permill::from_rational(parse_into!(u32, fee, error()), 1_000_000);

    let result = hydra_dx_math::stableswap::calculate_shares::<D_ITERATIONS>(
        &balances,
        &updated_balances,
        amplification,
        issuance,
        fee,
    );

    if let Some(r) = result {
        CString::new(r.0.to_string()).unwrap().into_raw()
    } else {
        error()
    }
}

#[no_mangle]
pub extern "C" fn sswap_calc_shares_for_amount(
    reserves: *const libc::c_char,
    asset_in: *const libc::c_char,
    amount: *const libc::c_char,
    amplification: *const libc::c_char,
    share_issuance: *const libc::c_char,
    fee: *const libc::c_char,
) -> *const libc::c_char {
    let reserves = std::str::from_utf8(unsafe { CStr::from_ptr(reserves) }.to_bytes()).unwrap();
    let asset_in = std::str::from_utf8(unsafe { CStr::from_ptr(asset_in) }.to_bytes()).unwrap();
    let amount = std::str::from_utf8(unsafe { CStr::from_ptr(amount) }.to_bytes()).unwrap();
    let amplification = std::str::from_utf8(unsafe { CStr::from_ptr(amplification) }.to_bytes()).unwrap();
    let share_issuance = std::str::from_utf8(unsafe { CStr::from_ptr(share_issuance) }.to_bytes()).unwrap();
    let fee = std::str::from_utf8(unsafe { CStr::from_ptr(fee) }.to_bytes()).unwrap();

    let asset_in = parse_into!(u32, asset_in, error());

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
    let amount_in = parse_into!(u128, amount, error());
    let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
    let amplification = parse_into!(u128, amplification, error());
    let issuance = parse_into!(u128, share_issuance, error());
    let fee = Permill::from_rational(parse_into!(u32, fee, error()), 1_000_000);

    let result = hydra_dx_math::stableswap::calculate_shares_for_amount::<D_ITERATIONS>(
        &balances,
        idx_in.unwrap(),
        amount_in,
        amplification,
        issuance,
        fee,
    );

    if let Some(r) = result {
        CString::new(r.0.to_string()).unwrap().into_raw()
    } else {
        error()
    }
}
