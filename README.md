# HydraDX-wasm

HydraDX-wasm is a WebAssembly library that exposes the core math utilities of the **Hydration** protocol for use in JavaScript/TypeScript and other environments. **Hydration** itself is a cross-chain liquidity protocol built on Substrate and deployed on Polkadot, aiming to aggregate liquidity across multiple assets through a unique *Omnipool* design. This project packages HydraDX's internal math library – a collection of utilities to perform liquidity pool calculations conveniently – into WebAssembly, allowing developers to run the same calculations off-chain (e.g. in web browsers or Node.js) with high performance and accuracy.

Using HydraDX-wasm, you can compute outcomes of trades, pool share valuations, fees, and other financial invariants exactly as the HydraDX runtime does on-chain. This is useful for building front-end interfaces, simulators, or off-chain routing algorithms that need to mirror on-chain logic. In short, **HydraDX-wasm is a Wasm wrapper for HydraDX pools math**, making HydraDX's advanced AMM (Automated Market Maker) formulas accessible to any application.

## Features

HydraDX-wasm supports a wide range of DeFi math calculations used in HydraDX (and its sister network Basilisk) pools. Key modules and formulas include:

- **XYK (Constant Product AMM)** – functions for the classic *x \* y = k* liquidity pool model. Use these to calculate swap outcomes, required input for a desired output, spot price, etc., in constant product pools (similar to Uniswap's model).
- **LBP (Liquidity Bootstrapping Pool)** – math for Balancer-style bootstrapping pools where weights change over time, typically used for token launches. This helps compute price evolution and swap outcomes in LBP auctions.
- **StableSwap (Stablecoin AMM)** – formulas for pools of assets with similar pegs (e.g. USDC/USDT), offering low slippage swaps. Use these utilities to calculate trades and pool invariants in a stable swap curve (Curve Finance's model).
- **Omnipool** – specialized calculations for HydraDX's one-of-a-kind multi-asset pool (the Omnipool) which holds many assets in a single pool. These cover pricing with a shared pool and the protocol-owned HDX "LRNA" asset, ensuring you can compute cross-asset swaps through the Omnipool logic.
- **Liquidity Mining** – utilities to compute rewards distribution, yields, or required locks for liquidity mining programs (reward incentives for liquidity providers).
- **EMA (Exponential Moving Average)** – math functions to maintain exponential moving averages, used by HydraDX for things like time-weighted average price or other indicators.
- **Staking** – calculations related to staking mechanisms or reward accumulation (used in HydraDX/Basilisk for certain incentivization logic).

All these capabilities are built-in. Under the hood, the library uses fixed-point arithmetic and rigorous checks (via Substrate's `sp-arithmetic`) to ensure precision and safety. The Rust crate powering this (hydra\_dx\_math) is the same code running on-chain, so you get full parity with on-chain calculations. Each module can be enabled or disabled via feature flags in Rust (e.g. `xyk`, `lbp`, `stableswap`, etc.), and the WebAssembly build can include one or all of them as needed.

## Project Structure

HydraDX-wasm now uses a modular file structure, with each feature (XYK, Omnipool, etc.) in its own file. This makes the codebase more maintainable and allows for targeted feature compilation. The structure is as follows:

- `src/lib.rs` - Main entry point that combines all modules
- `src/utils.rs` - Common utilities and helper functions
- `src/xyk.rs` - XYK pool implementation
- `src/lbp.rs` - Liquidity Bootstrapping Pool implementation
- `src/stableswap.rs` - StableSwap pool implementation
- `src/stableswap_drift.rs` - StableSwap drift calculations
- `src/liquidity_mining.rs` - Liquidity Mining calculations
- `src/omnipool.rs` - Omnipool implementation
- `src/ema.rs` - Exponential Moving Average calculations
- `src/staking.rs` - Staking reward calculations
- `src/fee.rs` - Fee calculations

### Building with Specific Features

You can build the project with only the features you need using Cargo's feature flags:

```sh
# Build with only XYK and Omnipool features
cargo build --release --features "xyk omnipool"

# Build with all features
cargo build --release --features "xyk lbp stableswap stableswap-drift liquidity-mining omnipool ema staking"
```

A test script is also provided to verify builds with different feature combinations:

```sh
# Make the script executable
chmod +x build-test.sh

# Run the test script
./build-test.sh
```

This will build and test various feature combinations to ensure everything works correctly.

## Installation

HydraDX-wasm can be used in two ways: by building the library from source, or by installing pre-built NPM packages for specific math modules. Below are instructions for both approaches.

### Prerequisites

If building from source, you need a Rust development environment. Ensure you have **Rust** (edition 2021 or later) and **Cargo** installed. You will also need [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) – a tool for building Rust into Wasm – which you can install with a one-liner:

```sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Having **Node.js** and npm/yarn is recommended if you plan to use the output in a Node or web project, or if you choose to install via NPM.

### Building from Source

To build the HydraDX-wasm library from source, clone this repository and run the build command:

```sh
git clone https://github.com/galacticcouncil/HydraDX-wasm.git
cd HydraDX-wasm

# Build the WebAssembly package (release mode)
make build-release
```

This will compile the Rust source into WebAssembly binaries for multiple targets (bundler, Node.js, and web). The output will appear in the `build/` directory.

### Using via NPM Packages

For convenience, the HydraDX math modules are also published as separate NPM packages under the `@galacticcouncil` scope. If you prefer not to build from source, you can install these pre-compiled packages directly into your project:

```sh
npm install @galacticcouncil/math-xyk
npm install @galacticcouncil/math-lbp
npm install @galacticcouncil/math-stableswap
npm install @galacticcouncil/math-omnipool
npm install @galacticcouncil/math-liquidity-mining
npm install @galacticcouncil/math-ema
npm install @galacticcouncil/math-staking
```

## Usage Example

```js
import { calculate_out_given_in } from '@galacticcouncil/math-xyk';  

const reserveA = "10000";
const reserveB = "5000";
const amountIn = "1000";

const amountOut = calculate_out_given_in(reserveA, reserveB, amountIn);
console.log(`Swapping ${amountIn} of A for B yields approximately ${amountOut} of B`);
```

## Contributing

Contributions to HydraDX-wasm are welcome! If you have an idea for improvement or find a bug, feel free to reach out or create a pull request.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](./LICENSE) file for details.

---

*HydraDX-wasm is part of the Hydration open-source ecosystem. You can find more about HydraDX on the *[*official website*](https://hydration.net/)* and in the *[*documentation*](https://docs.hydration.net)*. This repository specifically deals with off-chain math utilities.*

