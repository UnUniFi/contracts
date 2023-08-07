use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};

use crate::types::ChannelInfo;

#[cw_serde]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    pub deposit_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        unbond_period: Option<u64>,
        deposit_denom: Option<String>,
    },
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    AddRewards(AddRewardsMsg),
    IbcTransferToHost(IbcTransferToHostMsg),
    IbcTransferToController(IbcTransferToControllerMsg),
    IcaAddLiquidity(IcaAddLiquidityMsg),
    IcaRemoveLiquidity(IcaRemoveLiquidityMsg),
    IcaSwapRewardsToTwoTokens(IcaSwapRewardsToTwoTokensMsg),
    IcaSwapTwoTokensToDepositToken(IcaSwapTwoTokensToDepositTokenMsg),
    IcaSwapDepositTokenToTwoTokens(IcaSwapDepositTokenToTwoTokensMsg),
    StoreIcaUnlockedBalances(StoreIcaUnlockedBalancesMsg),
}

#[cw_serde]
pub struct IbcTransferToHostMsg {
    pub ica_channel_id: String,
    pub channel_id: String,
    pub denom: String,
    pub amount: Uint128,
    pub timeout: u64,
}

#[cw_serde]
pub struct IbcTransferToControllerMsg {
    pub channel_id: String,
    pub denom: String,
    pub amount: Uint128,
    pub timeout: u64,
}

#[cw_serde]
pub struct IcaAddLiquidityMsg {
    pub channel_id: String,
    pub denom: String,
    pub amount: Uint128,
    pub timeout: u64,
    pub val_addr: String, // TODO: temporary for MsgDelegate test - should remove
}

#[cw_serde]
pub struct IcaRemoveLiquidityMsg {
    pub channel_id: String,
    pub denom: String,
    pub amount: Uint128,
    pub timeout: u64,
}

#[cw_serde]
pub struct IcaSwapRewardsToTwoTokensMsg {
    pub channel_id: String,
    pub denom: String,
    pub amount: Uint128,
    pub timeout: u64,
}

#[cw_serde]
pub struct IcaSwapTwoTokensToDepositTokenMsg {
    pub channel_id: String,
    pub denom: String,
    pub amount: Uint128,
    pub timeout: u64,
}

#[cw_serde]
pub struct IcaSwapDepositTokenToTwoTokensMsg {
    pub channel_id: String,
    pub denom: String,
    pub amount: Uint128,
    pub timeout: u64,
}

#[cw_serde]
pub struct StoreIcaUnlockedBalancesMsg {
    pub coins: Vec<Coin>,
}

#[cw_serde]
pub struct StakeMsg {}

#[cw_serde]
pub struct UnstakeMsg {
    pub amount: Uint128,
}

#[cw_serde]
pub struct AddRewardsMsg {}

#[cw_serde]
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
    // TODO: add more queries to get exact state of the contract
}

#[cw_serde]
pub struct ListChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[cw_serde]
pub struct ChannelResponse {
    /// Information on the channel's connection
    pub info: ChannelInfo,
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
