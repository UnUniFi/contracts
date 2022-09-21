use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};
use yield_farming::asset::AssetInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub unbond_period: u64,
    pub is_freeze: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardPool {
    pub reward_token: AssetInfo,
    pub acc_reward_per_share: Uint128, // Accumulated Rewards per share, times 1e12. See below.
}

pub const CONFIG: Item<Config> = Item::new("config");

// value = total lp amount
pub const TOTAL_DEPOSITS: Item<Uint128> = Item::new("TOTAL_DEPOSITS");

pub const REWARD_POOLS: Item<Vec<RewardPool>> = Item::new("REWARD_POOLS");

// key = user address
// value = lp amount
pub const USER_DEPOSITS: Map<String, Uint128> = Map::new("USER_DEPOSITS");

// key = pool token addr + user addr
// value = user reward debt amount
pub const USER_REWARD_DEBTS: Map<String, Uint128> = Map::new("USER_REWARD_DEBT");

// key = pool token addr + user addr
// value = user pending reward amount
pub const USER_PENDING_REWARDS: Map<String, Uint128> = Map::new("USER_PENDING_REWARDS");
