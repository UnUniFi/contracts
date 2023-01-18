// use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountIn;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapMsgReplyState {
    pub original_sender: Addr,
}

// TODO: better struct
pub const DEPOSITOR_SHARE: Map<&Addr, Uint128> = Map::new("depositor_share");
pub const SWAP_REPLY_STATES: Map<u64, SwapMsgReplyState> = Map::new("swap_reply_states");
