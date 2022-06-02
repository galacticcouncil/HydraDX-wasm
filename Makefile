.PHONY: build-release 
build-release: build-web-release build-nodejs-release build-bundler-release

.PHONY: build-web-release 
build-web-release: 
	wasm-pack build --release --target web --out-dir ./build/xyk/web -- --features xyk
	wasm-pack build --release --target web --out-dir ./build/lbp/web -- --features lbp
	rm ./build/xyk/web/.gitignore ./build/lbp/web/.gitignore
	rm ./build/xyk/web/LICENSE ./build/lbp/web/LICENSE
	rm ./build/xyk/web/README.md ./build/lbp/web/README.md

.PHONY: build-nodejs-release 
build-nodejs-release: 
	wasm-pack build --release --target nodejs --out-dir ./build/xyk/nodejs -- --features xyk
	wasm-pack build --release --target nodejs --out-dir ./build/lbp/nodejs -- --features lbp
	rm ./build/xyk/nodejs/.gitignore ./build/lbp/nodejs/.gitignore
	rm ./build/xyk/nodejs/LICENSE ./build/lbp/nodejs/LICENSE
	rm ./build/xyk/nodejs/README.md ./build/lbp/nodejs/README.md

.PHONY: build-bundler-release 
build-bundler-release: 
	wasm-pack build --release --target bundler --out-dir ./build/xyk/bundler -- --features xyk
	wasm-pack build --release --target bundler --out-dir ./build/lbp/bundler -- --features lbp
	rm ./build/xyk/bundler/.gitignore ./build/lbp/bundler/.gitignore
	rm ./build/xyk/bundler/LICENSE ./build/lbp/bundler/LICENSE
	rm ./build/xyk/bundler/README.md ./build/lbp/bundler/README.md

.PHONY: tests
tests: 
	cargo test --all-features

