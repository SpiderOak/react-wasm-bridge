# react-wasm-bridge

Build react components with wasm modules (currently only with rust).

## Using react-wasm-bridge in your project

### Rust-side

Install rust nightly and set up a new rust library. Make sure you set the
`crate-type` to `"cdylib"` and add both `wasm-bindgen` and `react-wasm-bridge`
as dependencies.

Your module should export a function called `render` that takes two parameters:
`state` and `factory`.

```
#[wasm_bindgen]
pub fn render(state: &State, factory: &Builder) -> JsValue {
    ...
}
```

`state` is an object that holds the component props. You'll use `factory` to
generate elements and return a root elem (details TBD).

Build the module (`cargo +nightly build --target wasm32-unknown-unknown`) and
run it through `wasm-bindgen` (`wasm-bindgen
target/wasm32-unknown-unknown/$(MODE)/hello.wasm --out-dir .` or similar).

### React-side

Add `react-wasm-bridge` to your react project.  Then load your wasm module and
pass it to the `<ReactWasmBridge/>` component as the `module` prop. Any other
props will be passed to the underlying rust module into its `render` function
(strings and numbers only).

## Building the library

To build a transpiled version of the library, make sure you have all of these
things:

- yarn
- rust nightly
- wasm-bindgen
- Firefox

and run `make`.  The transpiled version will be output into `dist/`.

## Building the examples

Run `make examples` and it'll build all of them. Or just go into each example
directory to make them individually.

### Going faster

By default, the examples build in debug mode. Use `make run MODE=release` and
it'll build in release mode. If you've already built it, you'll have to `make
clean` or it won't rebuild.
