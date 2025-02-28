#!/bin/sh

rm -rf ./bin
mkdir -p ./bin

cargo -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort build --manifest-path ./src/relescope-rs/Cargo.toml --release --target wasm32-unknown-unknown --features error-detail

du -h ./src/relescope-rs/target/wasm32-unknown-unknown/release/relescope_rs.wasm

wasm-opt ./src/relescope-rs/target/wasm32-unknown-unknown/release/relescope_rs.wasm \
    -o ./bin/plugin.wasm \
    -O3 \
    --flatten \
    --rereloop \
    -Oz \
    -c \

du -h ./bin/plugin.wasm

typst c ./tests/test.typ --root . ./test.pdf 1> /dev/null
rm -f ./test.pdf