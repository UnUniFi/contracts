use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SendBack(SendBackMsg),
}

#[cw_serde]
pub struct SendBackMsg {
    pub class_id: String,
    pub token_id: String,
    pub origin_address: String,
}

pub struct DeputyListNftMsg {
    pub lister: String,
    pub class_id: String,
    pub token_id: String,
    pub bid_denom: String,
    pub min_deposit_rate: String, // cosmos.Dec
    pub min_bid_period: String,   // google.protobuf.Duration
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
