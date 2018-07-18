MODE ?= debug
CARGO_BUILD_FLAGS =
ifeq ($(MODE),release)
  CARGO_BUILD_FLAGS += --release
endif

.PHONY: all
all: dist/index.js

.PHONY: run
run: all
	npm start

dist/index.js: node_modules index.js dom.js ../../dist/index.js
	npx webpack

../../dist/index.js:
	make -C ../..

node_modules: package.json yarn.lock

yarn.lock:
	yarn

dom.js: src/*.rs
	cargo +nightly build $(CARGO_BUILD_FLAGS) --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/$(MODE)/dom.wasm --out-dir .

.PHONY: clean
clean:
	-rm dom.js dom_bg.wasm dom.d.ts dist/index.js dist/*.wasm
