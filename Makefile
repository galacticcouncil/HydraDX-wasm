.PHONY: build-release 
build-release: build-web-release build-nodejs-release build-bundler-release

.PHONY: build-web-release 
build-web-release: 
	wasm-pack build --release --target web --out-dir ./build/web
	rm ./build/web/.gitignore

.PHONY: build-nodejs-release 
build-nodejs-release: 
	wasm-pack build --release --target nodejs --out-dir ./build/nodejs
	rm ./build/nodejs/.gitignore

.PHONY: build-bundler-release 
build-bundler-release: 
	wasm-pack build --release --target bundler --out-dir ./build/bundler
	rm ./build/bundler/.gitignore

.PHONY: tests
tests: 
	cargo test

