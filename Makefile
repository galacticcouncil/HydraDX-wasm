.PHONY: build-release 
build-release: build-web-release build-nodejs-release build-bundler-release

.PHONY: build-web-release 
build-web-release: 
	wasm-pack build --release --target web --out-dir ./build/xyk/web -- --features xyk
	wasm-pack build --release --target web --out-dir ./build/lbp/web -- --features lbp
	wasm-pack build --release --target web --out-dir ./build/stableswap/web -- --features stableswap
	wasm-pack build --release --target web --out-dir ./build/liquidity-mining/web -- --features liquidity-mining
	wasm-pack build --release --target web --out-dir ./build/omnipool/web -- --features omnipool
	rm ./build/xyk/web/.gitignore ./build/lbp/web/.gitignore ./build/stableswap/web/.gitignore ./build/liquidity-mining/web/.gitignore ./build/omnipool/web/.gitignore
	rm ./build/xyk/web/LICENSE ./build/lbp/web/LICENSE ./build/stableswap/web/LICENSE ./build/liquidity-mining/web/LICENSE ./build/omnipool/web/LICENSE
	rm ./build/xyk/web/README.md ./build/lbp/web/README.md ./build/stableswap/web/README.md ./build/liquidity-mining/web/README.md ./build/omnipool/web/README.md

.PHONY: build-nodejs-release 
build-nodejs-release: 
	wasm-pack build --release --target nodejs --out-dir ./build/xyk/nodejs -- --features xyk
	wasm-pack build --release --target nodejs --out-dir ./build/lbp/nodejs -- --features lbp
	wasm-pack build --release --target nodejs --out-dir ./build/stableswap/nodejs -- --features stableswap
	wasm-pack build --release --target nodejs --out-dir ./build/liquidity-mining/nodejs -- --features liquidity-mining
	wasm-pack build --release --target nodejs --out-dir ./build/omnipool/nodejs -- --features omnipool
	rm ./build/xyk/nodejs/.gitignore ./build/lbp/nodejs/.gitignore ./build/stableswap/nodejs/.gitignore ./build/liquidity-mining/nodejs/.gitignore ./build/omnipool/nodejs/.gitignore
	rm ./build/xyk/nodejs/LICENSE ./build/lbp/nodejs/LICENSE ./build/stableswap/nodejs/LICENSE ./build/liquidity-mining/nodejs/LICENSE  ./build/omnipool/nodejs/LICENSE 
	rm ./build/xyk/nodejs/README.md ./build/lbp/nodejs/README.md ./build/stableswap/nodejs/README.md ./build/liquidity-mining/nodejs/README.md ./build/omnipool/nodejs/README.md

.PHONY: build-bundler-release 
build-bundler-release: 
	wasm-pack build --release --target bundler --out-dir ./build/xyk/bundler -- --features xyk
	wasm-pack build --release --target bundler --out-dir ./build/lbp/bundler -- --features lbp
	wasm-pack build --release --target bundler --out-dir ./build/stableswap/bundler -- --features stableswap
	wasm-pack build --release --target bundler --out-dir ./build/liquidity-mining/bundler -- --features liquidity-mining
	wasm-pack build --release --target bundler --out-dir ./build/omnipool/bundler -- --features omnipool
	rm ./build/xyk/bundler/.gitignore ./build/lbp/bundler/.gitignore ./build/stableswap/bundler/.gitignore ./build/liquidity-mining/bundler/.gitignore ./build/omnipool/bundler/.gitignore
	rm ./build/xyk/bundler/LICENSE ./build/lbp/bundler/LICENSE ./build/stableswap/bundler/LICENSE ./build/liquidity-mining/bundler/LICENSE ./build/omnipool/bundler/LICENSE
	rm ./build/xyk/bundler/README.md ./build/lbp/bundler/README.md ./build/stableswap/bundler/README.md ./build/liquidity-mining/bundler/README.md ./build/omnipool/bundler/README.md 


.PHONY: build-package 
build-package: 
	bash ./build-package.sh xyk
	bash ./build-package.sh lbp
	bash ./build-package.sh stableswap
	bash ./build-package.sh liquidity-mining
	bash ./build-package.sh omnipool

.PHONY: clean-package 
clean-package: 
	bash ./clean-package.sh xyk
	bash ./clean-package.sh lbp
	bash ./clean-package.sh stableswap
	bash ./clean-package.sh liquidity-mining
	bash ./clean-package.sh omnipool

.PHONY: tests
tests: 
	cargo test --all-features

