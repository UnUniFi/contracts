use std::time::Duration;

use crate::types::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    pub deposit_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig { owner: Option<String> },
    ListNft(ListNftMsg),
    SendBackNft(SendBackMsg),
}

#[cw_serde]
pub struct ListNftMsg {
    pub sender: String,
    pub source_chain: String,
    pub class_id: String,
    pub token_id: String,
    pub ununifi_address: String,
    pub bid_denom: String,
    pub min_deposit_rate: Decimal,
    pub min_bid_period: Duration,
}

#[cw_serde]
pub struct SendBackMsg {
    pub origin_chain: String,
    pub class_id: String,
    pub token_id: String,
    pub origin_address: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
