use cosmwasm_std::{Addr, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub unlock_period: u64,
    pub is_freeze: bool,
    pub default_timeout: u64,
    pub init_channel: bool,
    /// Default remote denom for send standalone actions
    pub default_remote_denom: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardPool {
    pub reward_token: String,          // denom or contract addr
    pub acc_reward_per_share: Uint128, // Accumulated Rewards per share, times 1e12. See below.
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DepositInfo {
    pub channel: String,
    pub timeout: Option<u64>,
    pub duration: Uint64,
    pub sender: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct LockInfo {
    pub lock_id: u64,
    pub denom: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct UnlockInfo {
    pub lock_id: u64,
    pub start_time: u64,
}

pub fn join_ibc_paths(path_a: &str, path_b: &str) -> String {
    format!("{}/{}", path_a, path_b)
}
