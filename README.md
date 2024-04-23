# Tarot frontend

## Setup

Require node

```bash
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```

## Build

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./html/wasm --target web ./target/wasm32-unknown-unknown/release/tarot-front.wasm
```
