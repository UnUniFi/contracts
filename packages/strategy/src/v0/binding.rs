use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg, IbcTimeout};

#[cw_serde]
pub enum UnunifiMsg {
    /// Unfortunately `json:submit_i_c_q_request` is registered in chain side.
    /// To modify this problem, this message will be renamed in v1.
    #[serde(rename = "submit_i_c_q_request")]
    SubmitICQRequest {
        connection_id: String,
        chain_id: String,
        query_prefix: String,
        query_key: Binary,
    },
    #[serde(rename = "ibc_transfer")]
    IbcTransfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout: IbcTimeout,
    },
}

impl From<UnunifiMsg> for CosmosMsg<UnunifiMsg> {
    fn from(msg: UnunifiMsg) -> CosmosMsg<UnunifiMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for UnunifiMsg {}
