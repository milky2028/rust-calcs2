## Required Tools
- wasm-pack
- rust
- cargo

## Run tests

```
wasm-pack test --nodejs
```

## Build for release

Default build is for bundlers.

```
wasm-pack build --release [--target web|nodejs]
```