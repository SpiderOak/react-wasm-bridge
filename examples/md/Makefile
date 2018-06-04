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

dist/index.js: node_modules index.js md.js ../../dist/index.js
	npx webpack

../../dist/index.js:
	make -C ../..

node_modules: package.json yarn.lock

yarn.lock:
	yarn

md.js: src/*.rs
	cargo +nightly build $(CARGO_BUILD_FLAGS) --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/$(MODE)/md.wasm --out-dir .

.PHONY: clean
clean:
	-rm md.js md_bg.wasm md.d.ts dist/index.js dist/*.wasm
