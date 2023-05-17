console.log("lbp");
const lbp = require('@galacticcouncil/math-lbp');
console.log(" bindings", Object.keys(lbp).length);

console.log("liquidity-mining");
const lm = require('@galacticcouncil/math-liquidity-mining');
console.log(" bindings", Object.keys(lm).length);

console.log("stableswap");
const stableswap = require('@galacticcouncil/math-stableswap');
console.log(" bindings", Object.keys(stableswap).length);

console.log("xyk");
const xyk = require('@galacticcouncil/math-xyk');
console.log(" bindings", Object.keys(xyk).length);

console.log("omnipool");
const omnipool = require('@galacticcouncil/math-omnipool');
console.log(" bindings", Object.keys(omnipool).length);

console.log("ema");
const ema = require('@galacticcouncil/math-ema');
console.log(" bindings", Object.keys(ema).length);