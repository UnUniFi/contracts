use std::time::Duration;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ListNft(ListNftMsg),
}

#[cw_serde]
pub struct ListNftMsg {
    pub ununifi_address: String,
    pub cw721_contract: String,
    pub token_id: String,
    pub bid_denom: String,
    pub min_deposit_rate: Decimal,
    pub min_bid_period: Duration,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
