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

## Test

```bash
npx serve html
```

## Prepare Assets

Glue and convert tarot into one image

```bash
cd tarot
montage *.jpg -tile 8x10 -geometry +0+0 ../classic.png
```
