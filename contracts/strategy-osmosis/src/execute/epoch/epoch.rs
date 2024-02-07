use crate::error::ContractError;
use crate::helpers::query_balance;
use crate::msgs::Phase;
use crate::state::{
    DepositToken, EpochCallSource, HOST_LP_RATE_MULTIPLIER, PARAMS, STAKE_RATE_MULTIPLIER, STATE,
};
use cosmwasm_std::{DepsMut, Env, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

use super::deposit::execute_deposit_phase_epoch;
use super::withdraw::execute_withdraw_phase_epoch;

pub fn execute_epoch(
    deps: DepsMut,
    env: Env,
    called_from: EpochCallSource,
    success: bool,
    ret: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;
    if let Ok(balance) = query_balance(
        &deps.querier,
        env.contract.address.to_owned(),
        params.controller_deposit_denom.to_string(),
    ) {
        state.controller_free_amount = balance;
        STATE.save(deps.storage, &state)?;
    }

    // recalculate redemption rate on every icq callback
    if called_from == EpochCallSource::IcqCallback {
        if state.total_shares.is_zero() {
            state.redemption_rate = STAKE_RATE_MULTIPLIER;
        } else {
            // active tvl is not unbonding tvl that is allocated to shares
            let mut active_tvl = (state.bonded_lp_amount - state.unbond_request_lp_amount)
                * state.lp_redemption_rate
                / HOST_LP_RATE_MULTIPLIER;
            active_tvl += state.controller_stacked_amount_to_deposit
                + state.controller_pending_transfer_amount;
            if params.phase == Phase::Deposit {
                if params.deposit_token == DepositToken::Quote {
                    active_tvl += state.free_quote_amount;
                } else {
                    active_tvl += state.free_base_amount;
                }
            }
            state.redemption_rate = active_tvl * STAKE_RATE_MULTIPLIER / state.total_shares;
        }
        STATE.save(deps.storage, &state)?;
    }

    if params.phase == Phase::Withdraw {
        return execute_withdraw_phase_epoch(deps, env, called_from, success, ret);
    }
    return execute_deposit_phase_epoch(deps, env, called_from, success, ret);
}
