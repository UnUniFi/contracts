#!/bin/bash

set -o errexit -o nounset

# compile stuff
cargo wasm

# In case to produce much smaller .wasm file
RUSTFLAGS='-C link-arg=-s'

RUST_BACKTRACE=1

cd $SCRIPT_DIR/../

# optimize to reduce code size. need docker installed for arm cpu.
# docker run --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   cosmwasm/rust-optimizer:0.12.10
  
# incase for m1
# docker run --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   cosmwasm/rust-optimizer-arm64:0.12.8
