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

dist/index.js: node_modules index.js hello.js ../../dist/index.js
	npx webpack

../../dist/index.js:
	make -C ../..

node_modules: package.json yarn.lock

yarn.lock:
	yarn

hello.js: src/*.rs
	cargo +nightly build $(CARGO_BUILD_FLAGS) --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/$(MODE)/hello.wasm --out-dir .

.PHONY: clean
clean:
	-rm hello.js hello_bg.wasm hello.d.ts dist/index.js dist/*.wasm
