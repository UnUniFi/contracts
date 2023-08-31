use crate::error::ContractError;
use crate::helpers::{exit_pool_to_any, join_swap_extern_amount_in_to_any};
use crate::state::{PARAMS, STATE};
use cosmwasm_std::{Env, Response, StdError, Storage};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgExitPool, MsgJoinSwapExternAmountIn};
use prost_types::Any;
use ununifi_binding::v1::binding::UnunifiMsg;

use super::helpers::determine_ica_amounts;

pub fn execute_ica_join_swap_extern_amount_in(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(store)?;
    let state = STATE.load(store)?;

    let mut msgs: Vec<Any> = vec![];
    if !state.free_quote_amount.is_zero() {
        let msg = MsgJoinSwapExternAmountIn {
            sender: params.ica_account.to_string(),
            share_out_min_amount: "1".to_string(),
            pool_id: params.pool_id,
            token_in: Some(OsmosisCoin {
                denom: params.quote_denom,
                amount: state.free_quote_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = join_swap_extern_amount_in_to_any(msg) {
            msgs.push(msg_any);
        }
    }
    if !state.free_base_amount.is_zero() {
        let msg = MsgJoinSwapExternAmountIn {
            sender: params.ica_account.to_string(),
            share_out_min_amount: "1".to_string(),
            pool_id: params.pool_id,
            token_in: Some(OsmosisCoin {
                denom: params.base_denom,
                amount: state.free_base_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = join_swap_extern_amount_in_to_any(msg) {
            msgs.push(msg_any);
        }
    }

    if msgs.len() > 0 {
        return Ok(send_ica_tx(
            env,
            params.ica_channel_id,
            params.transfer_timeout,
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
    let params = PARAMS.load(store)?;
    let mut state = STATE.load(store)?;
    let ica_amounts = determine_ica_amounts(params.to_owned(), state.to_owned());
    let to_remove_lp = ica_amounts.to_remove_lp;
    if to_remove_lp.is_zero() {
        return Ok(Response::new());
    }

    state.pending_lp_removal_amount = to_remove_lp;
    STATE.save(store, &state)?;

    let mut tokens_out: Vec<OsmosisCoin> = vec![
        OsmosisCoin {
            denom: params.quote_denom,
            amount: "1".to_string(),
        },
        OsmosisCoin {
            denom: params.base_denom.to_string(),
            amount: "1".to_string(),
        },
    ];
    tokens_out.sort_by_key(|d| d.denom.to_string());

    let msg = MsgExitPool {
        sender: params.ica_account.to_string(),
        share_in_amount: to_remove_lp.to_string(),
        pool_id: params.pool_id,
        token_out_mins: tokens_out,
    };
    if let Ok(msg_any) = exit_pool_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            params.ica_channel_id,
            params.transfer_timeout,
            "remove_liquidity".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "failure in conversion from proto to any: MsgExitPool".to_string(),
    }))
}
