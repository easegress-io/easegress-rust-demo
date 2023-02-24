# easegress-rust-demo
A Rust demo for Easegress WASM

## Build

1. Add `wasm32-unknown-unknown` target.

```bash
rustup target add wasm32-unknown-unknown 
```

2. Build with this command

```bash
cargo build --target wasm32-unknown-unknown --release
```

If success, it will generate `easegress_demo.wasm` at the `target/wasm32-unknown-unknown/release` folder.

## Note
If implement in your repo, please check `[lib]` and `[dependencies]` in `Cargo.toml`.
