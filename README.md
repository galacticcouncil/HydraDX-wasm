# HydraDX Math Wasm

## Install wasm-pack

`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`


## Build

`make build-release`

## Build package & publish to NPM

1. `make build-package`
2. `npm publish --workspaces --access public`
3. `make clean-package`