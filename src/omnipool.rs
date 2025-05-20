use wasm_bindgen::prelude::*;

pub use super::*;
use hydra_dx_math::dynamic_fees::types::{FeeParams, OracleEntry};
use hydra_dx_math::omnipool::types::{AssetReserveState, Position as OmnipoolPosition};
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

    if let Some(state_changes) = hydra_dx_math::omnipool::calculate_add_liquidity_state_changes(&state, amount) {
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
        withdrawal_fee,
    ) {
        state_changes.lp_hub_amount.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn recalculate_asset_fee(
    oracle_amount_in: String,
    oracle_amount_out: String,
    oracle_liquidity: String,
    oracle_period: String,
    current_asset_liquidity: String,
    previous_fee: String,
    block_difference: String,
    min_fee: String,
    max_fee: String,
    decay: String,
    amplification: String,
) -> String {
    // oracle entry
    let amount_in = parse_into!(u128, oracle_amount_in, error());
    let amount_out = parse_into!(u128, oracle_amount_out, error());
    let liquidity = parse_into!(u128, oracle_liquidity, error());
    let period = parse_into!(u128, oracle_period, error());

    let current_liquidity = parse_into!(u128, current_asset_liquidity, error());

    let block_difference = parse_into!(u128, block_difference, error());
    let previous_fee = Permill::from_float(parse_into!(f64, previous_fee, error()));
    let min_fee = Permill::from_float(parse_into!(f64, min_fee, error()));
    let max_fee = Permill::from_float(parse_into!(f64, max_fee, error()));
    let decay = FixedU128::from_rational(parse_into!(u128, decay, error()), FixedU128::DIV);
    let amplification = FixedU128::from_rational(parse_into!(u128, amplification, error()), FixedU128::DIV);

    let decay_factor = FixedU128::from_rational(2, period + 1);

    let entry = OracleEntry {
        amount_in,
        amount_out,
        liquidity,
        decay_factor,
    };

    let result = hydra_dx_math::dynamic_fees::recalculate_asset_fee(
        entry,
        current_liquidity,
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
    oracle_amount_in: String,
    oracle_amount_out: String,
    oracle_liquidity: String,
    oracle_period: String,
    current_asset_liquidity: String,
    previous_fee: String,
    block_difference: String,
    min_fee: String,
    max_fee: String,
    decay: String,
    amplification: String,
) -> String {
    let amount_in = parse_into!(u128, oracle_amount_in, error());
    let amount_out = parse_into!(u128, oracle_amount_out, error());
    let liquidity = parse_into!(u128, oracle_liquidity, error());
    let period = parse_into!(u128, oracle_period, error());

    let current_liquidity = parse_into!(u128, current_asset_liquidity, error());

    let block_difference = parse_into!(u128, block_difference, error());
    let previous_fee = Permill::from_float(parse_into!(f64, previous_fee, error()));
    let min_fee = Permill::from_float(parse_into!(f64, min_fee, error()));
    let max_fee = Permill::from_float(parse_into!(f64, max_fee, error()));
    let decay = FixedU128::from_rational(parse_into!(u128, decay, error()), FixedU128::DIV);
    let amplification = FixedU128::from_rational(parse_into!(u128, amplification, error()), FixedU128::DIV);

    let decay_factor = FixedU128::from_rational(2, period + 1);

    let entry = OracleEntry {
        amount_in,
        amount_out,
        liquidity,
        decay_factor,
    };

    let result = hydra_dx_math::dynamic_fees::recalculate_protocol_fee(
        entry,
        current_liquidity,
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
        Permill::zero(),
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

    let state_changes =
        if let Some(r) = hydra_dx_math::omnipool::calculate_sell_hub_state_changes(&asset, amount, asset_fee) {
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
        Permill::zero(),
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

    let state_changes = if let Some(r) =
        hydra_dx_math::omnipool::calculate_buy_for_hub_asset_state_changes(&asset, amount, asset_fee)
    {
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

    if let Some(result) = hydra_dx_math::omnipool::calculate_spot_price(&asset_a, &asset_b, None) {
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
    asset_fee: String,
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

    if let Some(result) =
        hydra_dx_math::omnipool::calculate_spot_price(&asset_a, &asset_b, Some((protocol_fee, asset_fee)))
    {
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

    if let Some(result) = hydra_dx_math::omnipool::calculate_lrna_spot_price(&asset, None) {
        result.to_string()
    } else {
        error()
    }
}

#[wasm_bindgen]
pub fn calculate_lrna_spot_price_with_fee(
    asset_reserve: String,
    asset_hub_reserve: String,
    asset_fee: String,
) -> String {
    let reserve = parse_into!(u128, asset_reserve, error());
    let hub_reserve = parse_into!(u128, asset_hub_reserve, error());
    let asset_fee = Permill::from_float(parse_into!(f64, asset_fee, error()));

    let asset = AssetReserveState {
        reserve,
        hub_reserve,
        ..Default::default()
    };

    if let Some(result) = hydra_dx_math::omnipool::calculate_lrna_spot_price(&asset, Some(asset_fee)) {
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

    if let Some(result) = hydra_dx_math::omnipool::calculate_cap_difference(&asset_state, asset_cap, total_hub_reserve)
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

    if let Some(state_changes) = hydra_dx_math::omnipool::calculate_add_liquidity_state_changes(&state, amount) {
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
        "9".to_string(),
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
        "9".to_string(),
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
    let result = calculate_lrna_spot_price("2000".to_string(), "500".to_string());
    assert_eq!(result, "4000000000000000000");

    let result = calculate_lrna_spot_price("2000".to_string(), "0".to_string());
    assert_eq!(result, "-1");
}

#[test]
fn calculate_lrna_spot_price_with_fee_should_work() {
    let result = calculate_lrna_spot_price_with_fee("2000".to_string(), "500".to_string(), "0.01".to_string());
    assert_eq!(result, "3960000000000000000");

    let result = calculate_lrna_spot_price_with_fee("2000".to_string(), "0".to_string(), "0.01".to_string());
    assert_eq!(result, "-1");
}
