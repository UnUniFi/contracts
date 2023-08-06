use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct KvIcqCallbackData {
    connection_id: String,
    chain_id: String,
    query_prefix: String,
    query_key: Binary,
    data: Binary,
}
