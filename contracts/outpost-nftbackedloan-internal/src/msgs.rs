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

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
