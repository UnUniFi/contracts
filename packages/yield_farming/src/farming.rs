use cosmwasm_std::{IbcEndpoint, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::amount::Amount;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub unlock_period: u64,
    /// Default timeout for ics20 packets, specified in seconds
    pub default_timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        unlock_period: Option<u64>,
    },
    UpdateFreezeFlag {
        freeze_flag: bool,
    },
    Swap(SwapMsg),
    JoinPool(JoinPoolMsg),
    ExitPool(ExitPoolMsg),
    CreateLockup(CreateLockupMsg),
    LockTokens(LockTokensMsg),
    ClaimReward(ClaimTokensMsg),
    ClaimAllRewards {},
    StartUnlockTokens(UnlockTokensMsg),
    ClaimUnlockedTokens {},
    SwapReward {
        source_token: String, // denom or contract addr
        dest_token: String,   // denom or contract addr
    },
    AutoCompoundRewards {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapMsg {
    pub channel: String,
    pub pool: Uint64,
    pub token_out: String,
    pub min_amount_out: Uint128,
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JoinPoolMsg {
    pub channel: String,
    pub pool: Uint64,
    pub share_min_out: Uint128,
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExitPoolMsg {
    pub channel: String,
    pub token_out: String,
    pub min_amount_out: Uint128,
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateLockupMsg {
    pub channel: String,
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LockTokensMsg {
    pub channel: String,
    pub timeout: Option<u64>,
    pub duration: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimTokensMsg {
    pub channel: String,
    pub timeout: Option<u64>,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UnlockTokensMsg {
    pub channel: String,
    pub timeout: Option<u64>,
    pub lock_id: Uint64,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferMsg {
    /// The local channel to send the packets on
    pub channel: String,
    /// The remote address to send to.
    /// Don't use HumanAddress as this will likely have a different Bech32 prefix than we use
    /// and cannot be validated locally
    pub remote_address: String,
    /// How long the packet lives in seconds. If not specified, use default_timeout
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    /// Show all channels we have connected to. Return type is ListChannelsResponse.
    ListChannels {},
    /// Returns the details of the name channel, error if not created.
    /// Return type: ChannelResponse.
    Channel {
        id: String,
    },
    /// Returns the lockup address of the channel and owner, empty if not created.
    /// Return type: LockupResponse.
    Lockup {
        channel: String,
        owner: String,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub unlock_period: u64,
    pub is_freeze: bool,
    pub default_timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ListChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ChannelResponse {
    /// Information on the channel's connection
    pub info: ChannelInfo,
    /// How many tokens we currently have pending over this channel
    pub balances: Vec<Amount>,
    /// The total number of tokens that have been sent over this channel
    /// (even if many have been returned, so balance is low)
    pub total_sent: Vec<Amount>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LockupResponse {
    pub owner: String,
    pub address: String,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
