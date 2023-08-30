use crate::error::ContractError;
use crate::helpers::{begin_unlocking_msg_to_any, lock_tokens_msg_to_any};
use crate::state::{CONFIG, STATE};
use cosmwasm_std::{Env, Response, StdError, Storage, Uint128};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::shim::Duration;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::lockup::{MsgBeginUnlocking, MsgLockTokens};
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_ica_bond_liquidity(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(store)?;
    let state = STATE.load(store)?;
    let share_out_amount = state.pending_bond_lp_amount;
    if share_out_amount.is_zero() {
        return Ok(Response::new());
    }

    let mut tokens_in: Vec<OsmosisCoin> = vec![
        OsmosisCoin {
            denom: config.quote_denom,
            amount: state.free_quote_amount.to_string(),
        },
        OsmosisCoin {
            denom: config.base_denom.to_string(),
            amount: state.free_base_amount.to_string(),
        },
    ];
    tokens_in.sort_by_key(|d| d.denom.to_string());

    let msg = MsgLockTokens {
        owner: config.ica_account.to_string(),
        coins: vec![OsmosisCoin {
            denom: config.lp_denom,
            amount: share_out_amount.to_string(),
        }],
        duration: Some(Duration {
            seconds: config.unbond_period as i64,
            nanos: 0,
        }),
    };
    if let Ok(msg_any) = lock_tokens_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "bond_lp_tokens".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "failure in conversion from proto to any: MsgLockTokens".to_string(),
    }))
}

pub fn execute_ica_begin_unbonding_lp_tokens(
    store: &mut dyn Storage,
    env: Env,
    unbonding_lp_amount: Uint128,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(store)?;
    let state = STATE.load(store)?;
    if unbonding_lp_amount.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgBeginUnlocking {
        owner: config.ica_account.to_string(),
        id: state.lock_id,
        coins: vec![OsmosisCoin {
            denom: config.lp_denom,
            amount: unbonding_lp_amount.to_string(),
        }],
    };
    if let Ok(msg_any) = begin_unlocking_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "begin_unbonding_lp".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "failure in conversion from proto to any: MsgBeginUnlocking".to_string(),
    }))
}
