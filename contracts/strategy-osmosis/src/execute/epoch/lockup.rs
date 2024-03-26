use crate::error::ContractError;
use crate::helpers::{
    begin_unlocking_msg_to_any, lock_and_superfluid_delegate_msg_to_any, lock_tokens_msg_to_any,
    superfluid_undelegate_and_unbond_lock_to_any,
};
use crate::state::{PARAMS, STATE};
use cosmwasm_std::{Env, Response, StdError, Storage, Uint128};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::shim::Duration;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::lockup::{MsgBeginUnlocking, MsgLockTokens};
use osmosis_std::types::osmosis::superfluid::{
    MsgLockAndSuperfluidDelegate, MsgSuperfluidUndelegateAndUnbondLock,
};
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn should_lock_and_superfluid_delegate(
    superfluid_validator: String,
    bonded_lp_amount: Uint128,
    automate_superfluid: bool,
) -> bool {
    superfluid_validator != "".to_string() && bonded_lp_amount.is_zero() && automate_superfluid
}

pub fn execute_ica_bond_liquidity(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(store)?;
    let state = STATE.load(store)?;
    let share_out_amount = state.pending_bond_lp_amount;
    if share_out_amount.is_zero() {
        return Ok(Response::new());
    }

    // requires superfluid delegation only for the first time
    if should_lock_and_superfluid_delegate(
        params.superfluid_validator.to_string(),
        state.bonded_lp_amount,
        params.automate_superfluid,
    ) {
        let msg = MsgLockAndSuperfluidDelegate {
            sender: params.ica_account.to_string(),
            coins: vec![OsmosisCoin {
                denom: params.lp_denom,
                amount: share_out_amount.to_string(),
            }],
            val_addr: params.superfluid_validator,
        };
        if let Ok(msg_any) = lock_and_superfluid_delegate_msg_to_any(msg) {
            return Ok(send_ica_tx(
                env,
                params.ica_channel_id,
                params.transfer_timeout,
                "bond_lp_tokens".to_string(),
                vec![msg_any],
            )?);
        }
        return Err(ContractError::Std(StdError::SerializeErr {
            source_type: "proto_any_conversion".to_string(),
            msg: "failure in conversion from proto to any: MsgLockAndSuperfluidDelegate"
                .to_string(),
        }));
    }

    let msg = MsgLockTokens {
        owner: params.ica_account.to_string(),
        coins: vec![OsmosisCoin {
            denom: params.lp_denom,
            amount: share_out_amount.to_string(),
        }],
        duration: Some(Duration {
            seconds: params.unbond_period as i64,
            nanos: 0,
        }),
    };
    if let Ok(msg_any) = lock_tokens_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            params.ica_channel_id,
            params.transfer_timeout,
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
    begin_unbonding_lp_amount: Uint128,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(store)?;
    let state = STATE.load(store)?;
    if begin_unbonding_lp_amount.is_zero() {
        return Ok(Response::new());
    }

    if params.superfluid_validator != "".to_string() {
        // - If superfluid lock, execute MsgSuperfluidUndelegateAndUnbondLock
        let msg = MsgSuperfluidUndelegateAndUnbondLock {
            sender: params.ica_account.to_string(),
            lock_id: state.lock_id,
            coin: Some(OsmosisCoin {
                denom: params.lp_denom,
                amount: begin_unbonding_lp_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = superfluid_undelegate_and_unbond_lock_to_any(msg) {
            return Ok(send_ica_tx(
                env,
                params.ica_channel_id,
                params.transfer_timeout,
                "begin_unbonding_lp".to_string(),
                vec![msg_any],
            )?);
        }
        return Err(ContractError::Std(StdError::SerializeErr {
            source_type: "proto_any_conversion".to_string(),
            msg: "failure in conversion from proto to any: MsgBeginUnlocking".to_string(),
        }));
    }

    // - If normal lock, execute MsgBeginUnlocking
    let msg = MsgBeginUnlocking {
        owner: params.ica_account.to_string(),
        id: state.lock_id,
        coins: vec![OsmosisCoin {
            denom: params.lp_denom,
            amount: begin_unbonding_lp_amount.to_string(),
        }],
    };
    if let Ok(msg_any) = begin_unlocking_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            params.ica_channel_id,
            params.transfer_timeout,
            "begin_unbonding_lp".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "failure in conversion from proto to any: MsgBeginUnlocking".to_string(),
    }))
}
