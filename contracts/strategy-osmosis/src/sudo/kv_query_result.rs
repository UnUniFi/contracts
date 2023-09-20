use crate::error::ContractError;
use crate::execute::epoch::epoch::execute_epoch;
use crate::helpers::{decode_and_convert, BANK_STORE_KEY};
use crate::icq::create_account_denom_balance_key;
use crate::state::{DepositToken, EpochCallSource, CONFIG, HOST_LP_RATE_MULTIPLIER, STATE};
use cosmwasm_std::{Binary, DepsMut, Env, Response, Uint128};
use osmosis_std::types::osmosis::gamm::v1beta1::Pool as OsmosisBalancerPool;
use prost::Message;
use prost_types::Any;
use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
use std::str::FromStr;
use ununifi_binding::v0::binding::UnunifiMsg;

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

    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;
    let converted_addr_bytes = decode_and_convert(&config.ica_account.as_str())?;
    let base_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.base_denom.to_string(),
    )?;
    let quote_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.quote_denom.to_string(),
    )?;
    let lp_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.lp_denom.to_string(),
    )?;

    let mut resp = Response::new().add_attribute("action", "sudo_kv_query_result");
    if query_prefix == BANK_STORE_KEY {
        if query_key == base_balance_key {}
        let mut amount = Uint128::from(0u128);
        if data.len() > 0 {
            // TODO: to update if Osmosis update Cosmos version to v0.47
            let balance: ProtoCoin = ProtoCoin::decode(data.as_slice())?;
            amount = Uint128::from_str(balance.amount.as_str())?;
        }
        if query_key == base_balance_key {
            state.free_base_amount = amount;
            resp = resp.add_attribute("free_base_amount", state.free_base_amount);
        } else if query_key == quote_balance_key {
            state.free_quote_amount = amount;
            resp = resp.add_attribute("free_quote_amount", state.free_quote_amount);
        } else if query_key == lp_balance_key {
            state.free_lp_amount = amount;
            resp = resp.add_attribute("free_lp_amount", state.free_lp_amount);
        }

        for (i, extern_token) in config.extern_tokens.iter().enumerate() {
            state.extern_token_amounts[i] = amount;
            resp = resp.add_attribute(
                format!("free_{}", extern_token.extern_token),
                state.free_lp_amount,
            );
        }
    } else {
        // GAMM_STORE_KEY
        let any: Any = Any::decode(data.as_slice())?;
        let pool: OsmosisBalancerPool = OsmosisBalancerPool::decode(any.value.as_slice())?;
        let mut host_deposit_denom = config.base_denom;
        if config.deposit_token == DepositToken::Quote {
            host_deposit_denom = config.quote_denom;
        }
        let mut deposit_denom_amount = Uint128::from(0u128);
        let mut total_share = Uint128::from(0u128);
        for pool_asset in pool.pool_assets {
            if let Some(token) = pool_asset.token {
                if token.denom == host_deposit_denom.to_string() {
                    deposit_denom_amount = Uint128::from_str(token.amount.as_str())?;
                    break;
                }
            }
        }
        if let Some(total_shares) = pool.total_shares {
            total_share = Uint128::from_str(total_shares.amount.as_str())?;
        }
        if !total_share.is_zero() {
            state.lp_redemption_rate =
                deposit_denom_amount * Uint128::from(2u128) * HOST_LP_RATE_MULTIPLIER / total_share;
            resp = resp.add_attribute("lp_redemption_rate", state.lp_redemption_rate);
        }
    }

    state.pending_icq -= 1;
    STATE.save(deps.storage, &state)?;

    if state.pending_icq == 0 {
        execute_epoch(deps, env, EpochCallSource::IcqCallback, true, None)?;
        resp = resp.add_attribute("execute_epoch", "icq_callback");
    }

    Ok(resp)
}
