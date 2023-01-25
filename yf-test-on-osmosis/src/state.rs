// use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountIn;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Option<Addr>,
    pub pool_id: u64,
    pub deposit_token_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DepositInfo {
    // TODO: deposited_token_amount is the amount of token that the depositor deposited
    // which is not sure how to be efficiently used yet
    pub deposited_token_amount: Uint128,
    pub share_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapJoinMsgReplyState {
    pub original_sender: Addr,
    pub deposited_token_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExitSwapMsgReplyState {
    pub original_sender: Addr,
}

// TODO: better struct
pub const CONFIG: Item<Config> = Item::new("config");
pub const DEPOSITOR_SHARE: Map<&Addr, Uint128> = Map::new("depositor_share");
pub const SWAP_JOIN_REPLY_STATES: Map<u64, SwapJoinMsgReplyState> =
    Map::new("swap_join_reply_states");
pub const EXIT_SWAP_REPLY_STATES: Map<u64, ExitSwapMsgReplyState> =
    Map::new("exit_swap_reply_states");
