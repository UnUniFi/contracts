# Basic Spec

## Concern

One of the remaining concerns is how to distribute (calculate) reward in proportion to the depositor's share.

1. Create contract instance using contract creation factory-like feature if cosmwasm has it and set contracts for each user which has funcitons to join, exit fund and retrieve reward

2. make users sign txs to instantiate contract which has join, exit fund and retrieve reward on Osmosis and create ibc-channel for each contract with one original contract (?) on UnUniFi 

Also, the other one is how to treat lockup duration ux.   
If the LPT isn't locked, it doesn't earn incentives aside trading fee. If it's locked, user have to send two txs to withdraw deposit because LPT has to wait for the unbonding period. First one is for unbonding LPT and second one is to exit pool.   

## Unbonding from lockup

There's lockup system in Osmosis, which users lockup the LP token to earn additional reward aside the trading fee.   
The addinial rewards are , for example, just $OSMO as an incentive program which is managed by the on-chain governance. The amount of the reward for each pool is determined by the vote periodically.   
There are three options for the period of the unbonding of the locked token (LPT). Each has unique reward rate, longer, then bigger, of course.   
The locked LPT can earn the defined reward in proportion to the share rate of the LPT in a pool constantly around 15:00 UTC until user decides to unbond the LPT. In the period of the unbonding tiem, the LPT still earns reward, but in a different rate of the previous option.   
For instance,
> If the liquidity provider begins the unbonding process for their 2 week bonded LP shares, they will earn rewards for all three bonding periods during the first day of unbonding.
>
> After the first day passes, they will only receive rewards for the 1 day and 1 week lockup periods. After seven days pass, they will only receive the 1 day rewards until the 2 weeks is complete and their LP shares are unlocked. The below chart is a visual example of what was just explained.

refs:
https://docs.osmosis.zone/osmosis-core/modules/lockup

**Without lockup, user can only earn trading fee.**   

## External Reward distribution

There's a gauge system which is managed by `pool-incenitves` and `incentives` modules for external party to distribute token for certain pools in Osmosis.   
Gauge is the container of the token which any third-party can provide to be distributed through the gauge for liquidity providers in a pool.   
When a pool is created using `GAMM` module, three different gauges are created by `pool-incentives` module. They have different unbonding period for each.   
In the current param, there are 1, 7 and 14 days.   
Each Gauge is able to have the unique epoch, but mostly a time a day at the same time of the distribution of the minted OSMO.

