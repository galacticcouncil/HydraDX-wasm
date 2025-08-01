extern crate core;

// Re-export macros
#[macro_use]
pub(crate) mod utils;
pub use utils::error;

#[cfg(feature = "xyk")]
pub mod xyk;

#[cfg(feature = "lbp")]
pub mod lbp;

#[cfg(feature = "stableswap")]
pub mod stableswap;

#[cfg(feature = "stableswap-drift")]
pub mod stableswap_drift;

#[cfg(feature = "liquidity-mining")]
pub mod liquidity_mining;

#[cfg(feature = "omnipool")]
pub mod omnipool;

#[cfg(feature = "ema")]
pub mod ema;

#[cfg(feature = "staking")]
pub mod staking;

// Fee module is available only when xyk or lbp features are enabled
#[cfg(any(feature = "xyk", feature = "lbp"))]
pub mod fee;

#[cfg(feature = "hsm")]
mod hsm;
