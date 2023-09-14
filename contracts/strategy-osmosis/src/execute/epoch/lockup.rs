use crate::error::ContractError;
use crate::helpers::{
    begin_unlocking_msg_to_any, lock_and_superfluid_delegate_msg_to_any, lock_tokens_msg_to_any,
    superfluid_undelegate_and_unbond_lock_to_any,
};
use crate::state::{CONFIG, STATE};
use cosmwasm_std::{Env, Response, StdError, Storage, Uint128};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::shim::Duration;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::lockup::{MsgBeginUnlocking, MsgLockTokens};
use osmosis_std::types::osmosis::superfluid::{
    MsgLockAndSuperfluidDelegate, MsgSuperfluidUndelegateAndUnbondLock,
};
use ununifi_binding::v0::binding::UnunifiMsg;

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

    // requires superfluid delegation only for the first time
    if config.superfluid_validator != "".to_string()
        && state.bonded_lp_amount == Uint128::from(0u128)
    {
        let msg = MsgLockAndSuperfluidDelegate {
            sender: config.ica_account.to_string(),
            coins: vec![OsmosisCoin {
                denom: config.lp_denom,
                amount: share_out_amount.to_string(),
            }],
            val_addr: config.superfluid_validator,
        };
        if let Ok(msg_any) = lock_and_superfluid_delegate_msg_to_any(msg) {
            return Ok(send_ica_tx(
                env,
                config.ica_channel_id,
                config.transfer_timeout,
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

    if config.superfluid_validator != "".to_string() {
        // - If superfluid lock, execute MsgSuperfluidUndelegateAndUnbondLock
        let msg = MsgSuperfluidUndelegateAndUnbondLock {
            sender: config.ica_account.to_string(),
            lock_id: state.lock_id,
            coin: Some(OsmosisCoin {
                denom: config.lp_denom,
                amount: unbonding_lp_amount.to_string(),
            }),
        };
        if let Ok(msg_any) = superfluid_undelegate_and_unbond_lock_to_any(msg) {
            return Ok(send_ica_tx(
                env,
                config.ica_channel_id,
                config.transfer_timeout,
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
