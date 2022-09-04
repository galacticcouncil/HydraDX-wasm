#!/bin/bash
cp build/$1/bundler/hydra_dx_wasm_bg.js packages/$1/index.esm.js
cp build/$1/bundler/hydra_dx_wasm.d.ts packages/$1/index.d.ts
cp build/$1/bundler/hydra_dx_wasm_bg.wasm packages/$1/hydra_dx_wasm_bg.wasm
cp build/$1/bundler/hydra_dx_wasm_bg.wasm.d.ts packages/$1/hydra_dx_wasm_bg.wasm.d.ts
cp build/$1/nodejs/hydra_dx_wasm.js packages/$1/index.js