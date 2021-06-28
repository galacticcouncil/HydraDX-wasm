.PHONY: build-release 
build-release: build-web-release build-nodejs-release build-bundler-release

.PHONY: build-web-release 
build-web-release: 
	wasm-pack build --release --target web --out-dir ./build/xyk/web -- --features xyk
	wasm-pack build --release --target web --out-dir ./build/lbp/web -- --features lbp
	rm ./build/xyk/web/.gitignore ./build/lbp/web/.gitignore

.PHONY: build-nodejs-release 
build-nodejs-release: 
	wasm-pack build --release --target nodejs --out-dir ./build/xyk/nodejs -- --features xyk
	wasm-pack build --release --target nodejs --out-dir ./build/lbp/nodejs -- --features lbp
	rm ./build/xyk/nodejs/.gitignore ./build/lbp/nodejs/.gitignore

.PHONY: build-bundler-release 
build-bundler-release: 
	wasm-pack build --release --target bundler --out-dir ./build/xyk/bundler -- --features xyk
	wasm-pack build --release --target bundler --out-dir ./build/lbp/bundler -- --features lbp
	rm ./build/xyk/bundler/.gitignore ./build/lbp/bundler/.gitignore

.PHONY: tests
tests: 
	cargo test

