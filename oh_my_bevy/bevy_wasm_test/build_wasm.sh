#!/bin/bash

name="bevy_wasm_test"

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name $name \
  --out-dir wasm \
  --target web target/wasm32-unknown-unknown/release/$name.wasm

# wasm build 에의해 생성되는 fetch를 추적가능한 fetch로 바꾸기
sed -i '' 's/input = fetch(input)/input = bevyProgressiveFetch(input)/' ./wasm/${name}.js

wasm-opt -Oz --output optimized.wasm wasm/${name}_bg.wasm
mv optimized.wasm wasm/${name}_bg.wasm

rm -rf ../../static/game/${name}
cp -r ./wasm ../../static/game/${name}
