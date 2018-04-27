## LET'S GO

If you have all of these things:

- yarn
- rust nightly
- wasm-bindgen
- Firefox

You can probably just do `make run` and then open Firefox to http://localhost:8080/.

## LET'S GO (FASTER)

Do `make run MODE=release` and it'll build in release mode. If you've already
built it, you'll have to `make clean` or it won't rebuild.
