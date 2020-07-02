#!/bin/bash
cd `dirname $0`;
cd "./wasm_apps";
cargo build --target wasm32-unknown-unknown --release;
cd "../build";
cargo run;
cd "../";
cargo build --target thumbv7m-none-eabi;