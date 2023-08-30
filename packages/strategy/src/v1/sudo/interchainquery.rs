use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct KvIcqCallbackData {
    pub connection_id: String,
    pub chain_id: String,
    pub query_prefix: String,
    pub query_key: Binary,
    pub data: Binary,
}
