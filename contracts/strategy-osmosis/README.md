# Osmosis strategy status groups

```rs
// Token transfer to host
// Affected states:
// - controller_stacked_amount_to_deposit
// - controller_pending_transfer_amount
// - controller deposit token amount
// - host deposit token amount
IbcTransferToHost
IbcTransferToHostCallback
// Timeout: can check with controller_pending_transfer_amount if not executed at this step
// Fine to revert back to IbcTransferToHost
RequestIcqAfterIbcTransferToHost
ResponseIcqAfterIbcTransferToHost

// External incentives sell
SellExternTokens
SellExternTokensCallback
RequestIcqAfterSellExternTokens
ResponseIcqAfterSellExternTokens

// Add and bond liquidity
// Affected states:
// - pending_bond_lp_amount
// - bonded_lp_amount
AddLiquidity
AddLiquidityCallback // Timeout: Fine to reset to AddLiquidity
BondLiquidity
BondLiquidityCallback // Timeout: Fine to revert back to BondLiquidity status without any change on pending_bond_lp_amount
RequestIcqAfterBondLiquidity
ResponseIcqAfterBondLiquidity

// Unbonding initialization
// Affected states:
// - unbondings |unbonding.start_time, unbonding.pending_start|
// - unbond_request_lp_amount
// - unbonding_lp_amount
// - bonded_lp_amount
BeginUnbondingForPendingRequests
BeginUnbondingForPendingRequestsCallback // Timeout: Fine to revert back to BeginUnbondingForPendingRequests

// Check if it needs to do the transition to withdraw phase
CheckMaturedUnbondings

// Remove liquidity
// Affected states:
// - unbondings |unbonding.marked|
// - pending_lp_removal_amount
// - unbonding_lp_amount
RemoveLiquidity
RemoveLiquidityCallback // Timeout: Fine to revert back to RemoveLiquidity
RequestIcqAfterRemoveLiquidity
ResponseIcqAfterRemoveLiquidity

// Swap removed liquidity to deposit token
// - host deposit token amount
SwapTwoTokensToDepositToken
SwapTwoTokensToDepositTokenCallback // Timeout: Fine to revert back to SwapTwoTokensToDepositToken
RequestIcqAfterSwapTwoTokensToDepositToken
ResponseIcqAfterSwapTwoTokensToDepositToken

// Token transfer to controller
// - host deposit token amount
// - controller deposit token amount
IbcTransferToController
IbcTransferToControllerCallback  // Timeout: Fine to revert back to IbcTransferToController
RequestIcqAfterIbcTransferToController
ResponseIcqAfterIbcTransferToController

// Distribution to users
// Affected states
// - unbondings (remove marked unbondings after distribution)
// - total_withdrawn
// - controller deposit token amount
DistributeToUnbondedUsers
```

## Admin state transition notes

- Phase transition between Deposit & Withdraw should be careful and should carefully consider if anything would be broken.

- Phase step transition should check if the affecting states are safe.
