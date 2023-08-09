use crate::error::ContractError;
use crate::helpers::{
    decode_and_convert, length_prefix, BALANCES_PREFIX, BANK_STORE_KEY, GAMM_STORE_KEY,
    POOLS_PREFIX,
};
use crate::state::{Config, CONFIG};
use cosmwasm_std::{Binary, Env, Response, Storage};
use ununifi_binding::v0::binding::UnunifiMsg;

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
    let mut config: Config = CONFIG.load(store)?;
    config.pending_icq = 4u64;
    CONFIG.save(store, &config)?;

    let converted_addr_bytes = decode_and_convert(&config.ica_account.as_str())?;

    let base_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.base_denom,
    )?;
    let quote_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.quote_denom,
    )?;
    let lp_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.lp_denom,
    )?;
    let gamm_pool_key = create_pool_key(config.host_config.pool_id)?;

    let msgs = vec![
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(base_balance_key),
        },
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(quote_balance_key),
        },
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(lp_balance_key),
        },
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: GAMM_STORE_KEY.to_string(),
            query_key: Binary(gamm_pool_key),
        },
    ];

    // Note: bonded lp and unbonding lp token balance could be managed without icq on contract side
    let resp = Response::new().add_messages(msgs);
    return Ok(resp);
}
