use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::asset::{Asset, AssetInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    /// Default timeout for ics20 packets, specified in seconds
    pub default_timeout: u64,
    /// initial allowlist - all cw20 tokens we will send must be previously allowed
    pub allowlist: Vec<AllowMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllowMsg {
    pub contract: String,
    pub gas_limit: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        unbond_period: Option<u64>,
    },
    UpdateFreezeFlag {
        freeze_flag: bool,
    },
    DepositNativeToken {},
    ClaimReward {
        asset: Asset,
    },
    ClaimAllRewards {},
    StartUnbond {},
    ClaimUnbond {},
    SwapReward {
        source_token: AssetInfo,
        dest_token: AssetInfo,
    },
    AutoCompoundRewards {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub unbond_period: u64,
    pub is_freeze: bool,
    pub default_timeout: u64,
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
