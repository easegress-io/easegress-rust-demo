# easegress-rust-demo
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-demo.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-demo?ref=badge_shield)

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


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-demo.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-demo?ref=badge_large)