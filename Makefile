# Features list
FEATURES := xyk lbp stableswap stableswap-drift liquidity-mining omnipool ema staking hsm
TARGETS := web nodejs bundler

# Main build release target
.PHONY: build-release
build-release: $(foreach feature,$(FEATURES),release-$(feature))

# Individual feature release targets
$(foreach feature,$(FEATURES),$(eval .PHONY: release-$(feature)))
$(foreach feature,$(FEATURES),$(eval release-$(feature): $(foreach target,$(TARGETS),build-$(feature)-$(target))))

# Target-specific build rules for each feature
$(foreach feature,$(FEATURES),$(foreach target,$(TARGETS),$(eval .PHONY: build-$(feature)-$(target))))
$(foreach feature,$(FEATURES),$(foreach target,$(TARGETS),$(eval build-$(feature)-$(target): ; \
	wasm-pack build --release --target $(target) --out-dir ./build/$(feature)/$(target) -- --features $(feature) ; \
	rm ./build/$(feature)/$(target)/.gitignore ./build/$(feature)/$(target)/LICENSE ./build/$(feature)/$(target)/README.md)))

# Legacy targets for backward compatibility
.PHONY: build-web-release 
build-web-release: $(foreach feature,$(FEATURES),build-$(feature)-web)

.PHONY: build-nodejs-release 
build-nodejs-release: $(foreach feature,$(FEATURES),build-$(feature)-nodejs)

.PHONY: build-bundler-release 
build-bundler-release: $(foreach feature,$(FEATURES),build-$(feature)-bundler)

.PHONY: test
test:
	cargo test --all-features

