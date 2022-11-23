use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};
use yield_farming::{error::ContractError, farming::ChannelInfo};

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

// Used to pass info from the ibc_packet_receive to the reply handler
pub const REPLY_ARGS: Item<ReplyArgs> = Item::new("reply_args");

pub const LOCKUP: Map<(&str, &str), String> = Map::new("lockups");

/// static info on one channel that doesn't change
pub const CHANNEL_INFO: Map<&str, ChannelInfo> = Map::new("channel_info");

/// indexed by (channel_id, denom) maintaining the balance of the channel in that currency
pub const CHANNEL_STATE: Map<(&str, &str), ChannelState> = Map::new("channel_state");

pub const TEMP_SENDER: Item<String> = Item::new("TEMP_SENDER");
pub const USER_LOCKS: Map<String, Vec<LockInfo>> = Map::new("USER_LOCKS");
pub const USER_UNLOCKS: Map<String, Vec<UnlockInfo>> = Map::new("USER_UNLOCKS");

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

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct ChannelState {
    pub outstanding: Uint128,
    pub total_sent: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ReplyArgs {
    pub channel: String,
    pub denom: String,
    pub amount: Uint128,
    pub our_chain: bool,
}

pub fn join_ibc_paths(path_a: &str, path_b: &str) -> String {
    format!("{}/{}", path_a, path_b)
}

pub fn increase_channel_balance(
    storage: &mut dyn Storage,
    channel: &str,
    denom: &str,
    amount: Uint128,
) -> Result<(), ContractError> {
    CHANNEL_STATE.update(storage, (channel, denom), |orig| -> StdResult<_> {
        let mut state = orig.unwrap_or_default();
        state.outstanding += amount;
        state.total_sent += amount;
        Ok(state)
    })?;
    Ok(())
}

pub fn reduce_channel_balance(
    storage: &mut dyn Storage,
    channel: &str,
    denom: &str,
    amount: Uint128,
) -> Result<(), ContractError> {
    CHANNEL_STATE.update(
        storage,
        (channel, denom),
        |orig| -> Result<_, ContractError> {
            // this will return error if we don't have the funds there to cover the request (or no denom registered)
            let mut cur = orig.ok_or(ContractError::InsufficientFunds {})?;
            cur.outstanding = cur
                .outstanding
                .checked_sub(amount)
                .or(Err(ContractError::InsufficientFunds {}))?;
            Ok(cur)
        },
    )?;
    Ok(())
}

// this is like increase, but it only "un-subtracts" (= adds) outstanding, not total_sent
// calling `reduce_channel_balance` and then `undo_reduce_channel_balance` should leave state unchanged.
pub fn undo_reduce_channel_balance(
    storage: &mut dyn Storage,
    channel: &str,
    denom: &str,
    amount: Uint128,
) -> Result<(), ContractError> {
    CHANNEL_STATE.update(storage, (channel, denom), |orig| -> StdResult<_> {
        let mut state = orig.unwrap_or_default();
        state.outstanding += amount;
        Ok(state)
    })?;
    Ok(())
}
