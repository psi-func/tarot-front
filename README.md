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
+#or
+python3 -m http.server 3000
```

## Prepare Assets

Glue and convert tarot into one image

```bash
cd tarot
montage *.jpg -tile 8x10 -geometry +0+0 ../classic.png

# better result
montage $(ls -1 *.jpg | sort -g) -tile 8X10 -geometry +0+0 -resize 240X400! -gravity center -crop 240X400+0+0 ../classic.tmp.png
convert ../classic.tmp.png -depth 8 ../classic.png # due to issue https://github.com/bevyengine/bevy/issues/4532
for f in ./tarot/*.jpg; do convert $f -resize 240X400! -depth 8 ./tmp/$(basename "$f" .jpg).png; done
```
