use crate::binding::{BALANCES_PREFIX, BANK_STORE_KEY, GAMM_STORE_KEY, POOLS_PREFIX};
use crate::epoch::execute_epoch;
use crate::error::ContractError;
use crate::helpers::{decode_and_convert, length_prefix};
use crate::state::{Config, EpochCallSource, CONFIG, HOST_LP_RATE_MULTIPLIER};
use cosmwasm_std::{Binary, DepsMut, Env, Response, Storage, Uint128};
use osmosis_std::types::osmosis::gamm::v1beta1::Pool as OsmosisBalancerPool;
use prost::Message;
use prost_types::Any;
use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
use std::str::FromStr;
use ununifi_msg::v0::binding::UnunifiMsg;

pub fn sudo_transfer_callback(
    deps: DepsMut,
    env: Env,
    denom: String,
    amount: String,
    sender: String,
    receiver: String,
    memo: String,
    success: bool,
) -> Result<Response<UnunifiMsg>, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: sudo_transfer_callback received",).as_str());
    execute_epoch(deps, env, EpochCallSource::TransferCallback, success, None)?;
    let res = Response::new().add_attribute("action", "ibc_transfer_callback".to_string());
    return Ok(res);
}

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(
    deps: DepsMut,
    env: Env,
    connection_id: String,
    chain_id: String,
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

    let mut config: Config = CONFIG.load(deps.storage)?;
    let converted_addr_bytes = decode_and_convert(&config.ica_account.as_str())?;
    let base_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.base_denom.to_string(),
    )?;
    let quote_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.quote_denom.to_string(),
    )?;
    let lp_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.lp_denom.to_string(),
    )?;

    if query_prefix == BANK_STORE_KEY {
        if query_key == base_balance_key {}
        let mut amount = Uint128::from(0u128);
        if data.len() > 0 {
            // TODO: to update if Osmosis update Cosmos version to v0.47
            let balance: ProtoCoin = ProtoCoin::decode(data.as_slice())?;
            amount = Uint128::from_str(balance.amount.as_str())?;
        }
        if query_key == base_balance_key {
            config.host_config.free_base_amount = amount;
        } else if query_key == quote_balance_key {
            config.host_config.free_quote_amount = amount;
        } else if query_key == lp_balance_key {
            config.host_config.free_lp_amount = amount;
        }
    } else {
        // GAMM_STORE_KEY
        let any: Any = Any::decode(data.as_slice())?;
        let pool: OsmosisBalancerPool = OsmosisBalancerPool::decode(any.value.as_slice())?;
        let mut base_amount = Uint128::from(0u128);
        let mut total_share = Uint128::from(0u128);
        for pool_asset in pool.pool_assets {
            if let Some(token) = pool_asset.token {
                if token.denom == config.host_config.base_denom.to_string() {
                    base_amount = Uint128::from_str(token.amount.as_str())?;
                    break;
                }
            }
        }
        if let Some(total_shares) = pool.total_shares {
            total_share = Uint128::from_str(total_shares.amount.as_str())?;
        }
        config.host_config.lp_redemption_rate =
            base_amount * Uint128::from(2u128) * HOST_LP_RATE_MULTIPLIER / total_share;
    }

    config.pending_icq -= 1;
    CONFIG.save(deps.storage, &config)?;

    if config.pending_icq == 0 {
        execute_epoch(deps, env, EpochCallSource::IcqCallback, true, None)?;
    }

    Ok(Response::default())
}

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
    env: Env,
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
