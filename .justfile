default:
  just --list

clippy:
  cargo clippy --all-targets -- -D warnings

check:
  cargo check --all-targets

fix:
  cargo fix --lib -p area_25_5 --allow-dirty

test:
  cargo test --tests

build-web:
  cargo build --profile wasm-release --target wasm32-unknown-unknown --features "web"

bind-web:
  wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "area_25_5" \
    ./target/wasm32-unknown-unknown/wasm-release/area_25_5.wasm

optimise-web:
  ~/binaryen-version_118/bin/wasm-opt -Oz -o ./out/area_25_5_bg.wasm ./out/area_25_5_bg.wasm

web: build-web bind-web optimise-web
  cp -R ./assets ./out
  cp ./index.html ./out
  zip game.zip -r ./out 

run-web:
  npx http-server ./out -o

