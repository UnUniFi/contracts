#!/bin/bash

set -o errexit -o nounset
cd $SCRIPT_DIR/../../

# compile stuff
cargo wasm

# In case to produce much smaller .wasm file
RUSTFLAGS='-C link-arg=-s'

RUST_BACKTRACE=1

# optimize to reduce code size. need docker installed for arm cpu.
# docker run --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   cosmwasm/rust-optimizer:0.12.11
  
# incase for m1
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.11
