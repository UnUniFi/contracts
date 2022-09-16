use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub unbond_period: u64,
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
    Deposit {
        denom: String,
        amount: Uint128,
    },
    ClaimReward {
        denom: String,
        amount: Uint128,
    },
    ClaimAllRewards {},
    StartUnbond {},
    ClaimUnbond {},
    SwapReward {
        source_token: String,
        dest_token: String,
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
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
