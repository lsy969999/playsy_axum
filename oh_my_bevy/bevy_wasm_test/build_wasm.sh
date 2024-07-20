#!/bin/bash

name="bevy_wasm_test"

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name $name \
  --out-dir wasm \
  --target web target/wasm32-unknown-unknown/release/$name.wasm

wasm-opt -Oz --output optimized.wasm wasm/${name}_bg.wasm
mv optimized.wasm wasm/${name}_bg.wasm

rm -rf ../../static/game/${name}
cp -r ./wasm ../../static/game/${name}
