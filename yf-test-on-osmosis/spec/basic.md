# Basic Spec

## Concern

One of the remaining concerns is how to distribute (calculate) reward in proportion to the depositor's share.

1. Create contract instance using contract creation factory-like feature if cosmwasm has it and set contracts for each user which has funcitons to join, exit fund and retrieve reward

2. make users sign txs to instantiate contract which has join, exit fund and retrieve reward on Osmosis and create ibc-channel for each contract with one original contract (?) on UnUniFi 

# Unbonding from lockup

> If the liquidity provider begins the unbonding process for their 2 week bonded LP shares, they will earn rewards for all three bonding periods during the first day of unbonding.
>
>After the first day passes, they will only receive rewards for the 1 day and 1 week lockup periods. After seven days pass, they will only receive the 1 day rewards until the 2 weeks is complete and their LP shares are unlocked. The below chart is a visual example of what was just explained.

refs:
https://docs.osmosis.zone/osmosis-core/modules/lockup



