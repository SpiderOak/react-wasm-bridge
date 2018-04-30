MODE ?= debug
CARGO_BUILD_FLAGS =
ifeq ($(MODE),release)
  CARGO_BUILD_FLAGS += --release
endif

.PHONY: all
all: dist/index.js

dist/index.js: node_modules src/*.js
	npx webpack

node_modules: package.json yarn.lock

yarn.lock:
	yarn

.PHONY: examples
examples:
	for X in examples/*; do make -C $$X; done

.PHONY: clean
clean:
	rm dist/index.js
