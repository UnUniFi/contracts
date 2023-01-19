// use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountIn;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapJoinMsgReplyState {
    pub original_sender: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExitSwapMsgReplyState {
    pub original_sender: Addr,
    pub token_out_denom: String,
}

// TODO: better struct
pub const DEPOSITOR_SHARE: Map<&Addr, Uint128> = Map::new("depositor_share");
pub const SWAP_JOIN_REPLY_STATES: Map<u64, SwapJoinMsgReplyState> =
    Map::new("swap_join_reply_states");
pub const EXIT_SWAP_REPLY_STATES: Map<u64, ExitSwapMsgReplyState> =
    Map::new("exit_swap_reply_states");
