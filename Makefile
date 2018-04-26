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

dist/index.js: node_modules src/*.js src/hello/hello.js
	npx webpack

node_modules: package.json yarn.lock
	yarn

src/hello/hello.js: src/hello/src/*.rs
	cd src/hello && cargo +nightly build $(CARGO_BUILD_FLAGS) --target wasm32-unknown-unknown
	cd src/hello && wasm-bindgen target/wasm32-unknown-unknown/$(MODE)/hello.wasm --out-dir .

.PHONY: clean
clean:
	rm src/hello/hello.js dist/index.js dist/*.wasm
