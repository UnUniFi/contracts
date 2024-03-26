use crate::error::ContractError;
use crate::helpers::{POOLS_PREFIX, STAKEIBC_STORE_KEY};
use crate::state::{PARAMS, STATE};
use cosmwasm_std::{Binary, Env, Response, Storage};
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn create_host_zone_key(pool_id: u64) -> Result<Vec<u8>, ContractError> {
    let mut pool_key: Vec<u8> = vec![POOLS_PREFIX];
    pool_key.extend_from_slice(pool_id.to_be_bytes().as_slice());

    Ok(pool_key)
}

// Submit the ICQ for the withdrawal account balance
pub fn submit_icq_for_redemption(
    store: &mut dyn Storage,
    _env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(store)?;
    let zone_pool_key = create_host_zone_key(0u64)?;

    let mut msgs = vec![UnunifiMsg::RequestKvIcq {
        chain_id: params.chain_id.to_string(),
        connection_id: params.connection_id.to_string(),
        query_prefix: STAKEIBC_STORE_KEY.to_string(),
        query_key: Binary(zone_pool_key),
    }];

    // Note: bonded lp and unbonding lp token balances are managed without icq on contract side
    let resp = Response::new()
        .add_messages(msgs)
        .add_attribute("action", "submit_icq")
        .add_attribute("ls_denom", params.ls_denom.to_string());
    return Ok(resp);
}
