use cosmwasm_std::IbcEndpoint;
use cosmwasm_std::{Coin, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::DepositToken;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    pub transfer_timeout: u64,
    pub controller_deposit_denom: String,
    pub base_denom: String,
    pub quote_denom: String,
    pub chain_id: String,
    pub pool_id: u64,
    pub lp_denom: String,
    pub transfer_channel_id: String,
    pub controller_transfer_channel_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    ExecuteEpoch(ExecuteEpochMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdateConfigMsg {
    pub owner: Option<String>,
    pub deposit_token: Option<DepositToken>,
    pub unbond_period: Option<u64>,
    pub pool_id: Option<u64>,
    pub ica_channel_id: Option<String>,
    pub lp_denom: Option<String>,
    pub phase: Option<Phase>,
    pub phase_step: Option<PhaseStep>,
    pub transfer_timeout: Option<u64>,
    pub transfer_channel_id: Option<String>,
    pub quote_denom: Option<String>,
    pub base_denom: Option<String>,
    pub controller_deposit_denom: Option<String>,
    pub controller_transfer_channel_id: Option<String>,
    pub chain_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IcqBalanceCallbackMsg {
    pub coins: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IbcTransferCallbackMsg {}

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
    State {},
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Phase {
    Deposit,
    Withdraw,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PhaseStep {
    // Deposit operations
    IbcTransferToHost,
    IbcTransferToHostCallback,
    RequestICQAfterIbcTransferToHost,
    ResponseICQAfterIbcTransferToHost,
    AddLiquidity,
    AddLiquidityCallback,
    BondLiquidity,
    BondLiquidityCallback,
    RequestICQAfterBondLiquidity,
    ResponseICQAfterBondLiquidity,
    BeginUnbondingForPendingRequests,
    BeginUnbondingForPendingRequestsCallback,
    CheckMaturedUnbondings,

    // Withdraw operations
    RemoveLiquidity,
    RemoveLiquidityCallback,
    RequestICQAfterRemoveLiquidity,
    ResponseICQAfterRemoveLiquidity,
    SwapTwoTokensToDepositToken,
    SwapTwoTokensToDepositTokenCallback,
    RequestICQAfterSwapTwoTokensToDepositToken,
    ResponseICQAfterSwapTwoTokensToDepositToken,
    IbcTransferToController,
    IbcTransferToControllerCallback,
    RequestICQAfterIbcTransferToController,
    ResponseICQAfterIbcTransferToController,
    DistributeToUnbondedUsers,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
    /// interchain account address
    pub address: String,
}
