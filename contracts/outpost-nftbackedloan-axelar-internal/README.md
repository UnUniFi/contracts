# Outpost Nft Backed Loan Receiver

## Dependency

- IBC Hooks
- NFT Factory

## Overview

External chains send ICS20 packets with memo data for IBC Hooks to UnUniFi chain, and IBC Hooks on UnUniFi chain calls this contract.
For example, Axelar can send packets following the GMP from EVM chains.
CW721 chains like Stargaze also can send ICS20 packets with memo data.

## How to works

### ListNft

- Mint NFT with class id `nftfactory/{sender}/{class_id}` with `nftfactory` module as `SubMsg`.
  - `class_id` is a contract address if it is an ERC721 NFT.
- If the reply of above `SubMsg` is `Ok`, then send a `MsgListNft` of `nftbackedloan` module.

### Repay, EndListing

- Send a message of `nftbackedloan` module as `Msg`.

### Borrow, WithdrawNft

- Send a message of `nftbackedloan` module as `SubMsg`.
- If the reply of above `SubMsg` is `Ok`, then send back a message.
