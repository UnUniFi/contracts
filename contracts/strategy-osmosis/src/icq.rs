use crate::error::ContractError;
use crate::helpers::{
    decode_and_convert, length_prefix, BALANCES_PREFIX, BANK_STORE_KEY, GAMM_STORE_KEY,
    POOLS_PREFIX,
};
use crate::state::{PARAMS, STATE};
use cosmwasm_std::{Binary, Env, Response, Storage};
use ununifi_binding::v1::binding::UnunifiMsg;

/// Creates balances Cosmos-SDK storage prefix for account with **addr**
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/bank/types/key.go#L55
pub fn create_account_balances_prefix<AddrBytes: AsRef<[u8]>>(
    addr: AddrBytes,
) -> Result<Vec<u8>, ContractError> {
    let mut prefix: Vec<u8> = vec![BALANCES_PREFIX];
    prefix.extend_from_slice(length_prefix(addr)?.as_slice());

    Ok(prefix)
}

/// Creates **denom** balance Cosmos-SDK storage key for account with **addr**
pub fn create_account_denom_balance_key<AddrBytes: AsRef<[u8]>, S: AsRef<str>>(
    addr: AddrBytes,
    denom: S,
) -> Result<Vec<u8>, ContractError> {
    let mut account_balance_key = create_account_balances_prefix(addr)?;
    account_balance_key.extend_from_slice(denom.as_ref().as_bytes());

    Ok(account_balance_key)
}

pub fn create_pool_key(pool_id: u64) -> Result<Vec<u8>, ContractError> {
    let mut pool_key: Vec<u8> = vec![POOLS_PREFIX];
    pool_key.extend_from_slice(pool_id.to_be_bytes().as_slice());

    Ok(pool_key)
}

// Submit the ICQ for the withdrawal account balance
pub fn submit_icq_for_host(
    store: &mut dyn Storage,
    _env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut state = STATE.load(store)?;
    state.pending_icq = 4u64;
    STATE.save(store, &state)?;

    let params = PARAMS.load(store)?;
    let converted_addr_bytes = decode_and_convert(&params.ica_account.as_str())?;

    let base_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        params.base_denom.to_string(),
    )?;
    let quote_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        params.quote_denom.to_string(),
    )?;
    let lp_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        params.lp_denom.to_string(),
    )?;
    let gamm_pool_key = create_pool_key(params.pool_id.to_owned())?;

    let msgs = vec![
        UnunifiMsg::RequestKvIcq {
            chain_id: params.chain_id.to_string(),
            connection_id: params.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(base_balance_key),
        },
        UnunifiMsg::RequestKvIcq {
            chain_id: params.chain_id.to_string(),
            connection_id: params.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(quote_balance_key),
        },
        UnunifiMsg::RequestKvIcq {
            chain_id: params.chain_id.to_string(),
            connection_id: params.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(lp_balance_key),
        },
        UnunifiMsg::RequestKvIcq {
            chain_id: params.chain_id.to_string(),
            connection_id: params.ica_connection_id.to_string(),
            query_prefix: GAMM_STORE_KEY.to_string(),
            query_key: Binary(gamm_pool_key),
        },
    ];

    // Note: bonded lp and unbonding lp token balances are managed without icq on contract side
    let resp = Response::new()
        .add_messages(msgs)
        .add_attribute("action", "submit_icq")
        .add_attribute("base_denom", params.base_denom.to_string())
        .add_attribute("quote_denom", params.quote_denom.to_string())
        .add_attribute("lp_denom", params.lp_denom.to_string())
        .add_attribute("pool_id", params.pool_id.to_string());
    return Ok(resp);
}
