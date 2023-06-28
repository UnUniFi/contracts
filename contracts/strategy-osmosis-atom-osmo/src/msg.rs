use crate::state::ChannelInfo;
use cosmwasm_std::{Coin, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    pub deposit_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        unbond_period: Option<u64>,
        deposit_denom: Option<String>,
        lp_redemption_rate: Option<Uint128>,
    },
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    ExecuteEpoch(ExecuteEpochMsg),
    IbcTransferToHost(IbcTransferToHostMsg),
    IbcTransferToController(IbcTransferToControllerMsg),
    IcaAddAndBondLiquidity(IcaAddAndBondLiquidityMsg),
    IcaRemoveLiquidity(IcaRemoveLiquidityMsg),
    IcaSwapRewardsToTwoTokens(IcaSwapRewardsToTwoTokensMsg),
    IcaSwapTwoTokensToDepositToken(IcaSwapTwoTokensToDepositTokenMsg),
    IcaSwapBalanceToTwoTokens(IcaSwapBalanceToTwoTokensMsg),
    IcaBeginUnbondLpTokens(IcaBeginUnbondLpTokensMsg),
    StoreIcaUnlockedBalances(StoreIcaUnlockedBalancesMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StoreIcaUnlockedBalancesMsg {
    pub coins: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakeMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UnstakeMsg {
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExecuteEpochMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IbcTransferToHostMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IbcTransferToControllerMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcaAddAndBondLiquidityMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcaRemoveLiquidityMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcaSwapRewardsToTwoTokensMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcaSwapTwoTokensToDepositTokenMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcaSwapBalanceToTwoTokensMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcaBondLpTokensMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcaBeginUnbondLpTokensMsg {
    pub unbonding_lp_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Bonded {
        addr: String,
    },
    Unbonding {
        addr: String,
    },
    Fee {},
    /// Show all channels connected to.
    ListChannels {},
    /// Returns the details of the name channel, error if not created.
    Channel {
        id: String,
    },
    Unbondings {},
    // TODO: add more queries to get exact state of the contract
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct FeeInfo {
    pub deposit_fee_rate: Decimal,
    pub withdraw_fee_rate: Decimal,
    pub interest_fee_rate: Decimal,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChannelResponse {
    /// Information on the channel's connection
    pub info: ChannelInfo,
}
