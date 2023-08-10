use crate::error::ContractError;
use crate::helpers::{exit_pool_to_any, join_swap_extern_amount_in_to_any};
use crate::state::{Config, CONFIG};
use cosmwasm_std::{Env, Response, StdError, Storage};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgExitPool, MsgJoinSwapExternAmountIn};
use prost_types::Any;
use ununifi_binding::v0::binding::UnunifiMsg;

use super::helpers::determine_ica_amounts;

pub fn execute_ica_join_swap_extern_amount_in(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;

    let mut msgs: Vec<Any> = vec![];
    if !config.host_config.free_quote_amount.is_zero() {
        let msg = MsgJoinSwapExternAmountIn {
            sender: config.ica_account.to_string(),
            share_out_min_amount: "1".to_string(),
            pool_id: config.host_config.pool_id,
            token_in: Some(OsmosisCoin {
                denom: config.host_config.quote_denom,
                amount: config.host_config.free_quote_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = join_swap_extern_amount_in_to_any(msg) {
            msgs.push(msg_any);
        }
    }
    if !config.host_config.free_base_amount.is_zero() {
        let msg = MsgJoinSwapExternAmountIn {
            sender: config.ica_account.to_string(),
            share_out_min_amount: "1".to_string(),
            pool_id: config.host_config.pool_id,
            token_in: Some(OsmosisCoin {
                denom: config.host_config.base_denom,
                amount: config.host_config.free_base_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = join_swap_extern_amount_in_to_any(msg) {
            msgs.push(msg_any);
        }
    }

    if msgs.len() > 0 {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "join_swap_extern_amount_in".to_string(),
            msgs,
        )?);
    }
    return Ok(Response::new());
}

pub fn execute_ica_remove_liquidity(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_remove_lp = ica_amounts.to_remove_lp;
    if to_remove_lp.is_zero() {
        return Ok(Response::new());
    }

    config.host_config.pending_lp_removal_amount = to_remove_lp;
    CONFIG.save(store, &config)?;

    let mut tokens_out: Vec<OsmosisCoin> = vec![
        OsmosisCoin {
            denom: config.host_config.quote_denom,
            amount: "1".to_string(),
        },
        OsmosisCoin {
            denom: config.host_config.base_denom.to_string(),
            amount: "1".to_string(),
        },
    ];
    tokens_out.sort_by_key(|d| d.denom.to_string());

    let msg = MsgExitPool {
        sender: config.ica_account.to_string(),
        share_in_amount: to_remove_lp.to_string(),
        pool_id: config.host_config.pool_id,
        token_out_mins: tokens_out,
    };
    if let Ok(msg_any) = exit_pool_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "remove_liquidity".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}
