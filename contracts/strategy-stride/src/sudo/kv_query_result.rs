use crate::error::ContractError;
use cosmwasm_std::{Binary, DepsMut, Env, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(
    deps: DepsMut,
    env: Env,
    _connection_id: String,
    _chain_id: String,
    query_prefix: String,
    query_key: Binary,
    data: Binary,
) -> Result<Response<UnunifiMsg>, ContractError> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_key,
        )
        .as_str(),
    );

    // TODO: write handler for Stride hostzone query

    Ok(Response::new())
}
