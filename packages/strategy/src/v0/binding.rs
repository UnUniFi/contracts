use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg, IbcTimeout};

#[cw_serde]
pub enum UnunifiMsg {
    SubmitICQRequest {
        connection_id: String,
        chain_id: String,
        query_prefix: String,
        query_key: Binary,
    },
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
