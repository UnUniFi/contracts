use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

#[cw_serde]
pub enum UnunifiMsg {
    #[serde(rename = "deputy_list_nft")]
    DeputyListNft {
        lister: String,
        class_id: String,
        token_id: String,
        bid_denom: String,
        min_deposit_rate: String, // cosmos.Dec
        min_bid_period: String, // google.protobuf.Duration
    },
}

impl From<UnunifiMsg> for CosmosMsg<UnunifiMsg> {
    fn from(msg: UnunifiMsg) -> CosmosMsg<UnunifiMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for UnunifiMsg {}