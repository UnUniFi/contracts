use cosmwasm_std::{Coin, Uint128};
// use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // let the contract send a MsgJoinSwapExternAmountIn to join a single
    // token into a pool with one msg
    JoinSwapExtern {
        pool_id: u64,
        token_in: Coin,
        share_out_min_amount: String,
    },
    // let the contract send a MsgExitSwapExternAmountOut to exit a single
    // token from a pool with one msg
    ExitSwapShare {
        pool_id: u64,
        token_out_denom: String,
        share_in_amount: String,
        token_out_min_amount: String,
    }, // Lockup {
       //     timeout: Option<u64>,
       //     duration: Uint64,
       // }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    DepositorShareAmount { depositor: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetDepositorShareAmountResponse {
    pub share_amount: Uint128,
}
