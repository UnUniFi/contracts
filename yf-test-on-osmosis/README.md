# Test Contract for yield-farming on Osmosis of our yield-aggregator

## Run script

First, you need to run Osmosis chain by kicking off the script to deploy contract and test it out.

```shell
cd ci-scripts/local-osmo
./start.sh
```

And, you need to comment off one of the optimizer options in /ci-scripts/contract/compile_and_optimize.sh.

```shell
# optimize to reduce code size. need docker installed for arm cpu.
# for amd
# docker run --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   cosmwasm/rust-optimizer:0.12.11
  
# incase for m1
# docker run --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   cosmwasm/rust-optimizer-arm64:0.12.11
```

And then, it's ready to actually test contract on chain using Osmosis by running below script.

```shell
cd ci-scripts/contract
./exec.sh
```
