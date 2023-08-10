use crate::error::{ContractError, NoDeposit};
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{
    DepositInfo, Unbonding, DEPOSITS, HOST_LP_RATE_MULTIPLIER, STAKE_RATE_MULTIPLIER, STATE,
    UNBONDINGS,
};

use cosmwasm_std::{Addr, DepsMut, Response, StdResult, Uint128};
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let unstake_amount = amount * STAKE_RATE_MULTIPLIER / state.redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(unstake_amount)?,
                });
            }
            Err(NoDeposit {}.into())
        },
    )?;

    let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
    if unbondings.len() as u32 >= UNBONDING_ITEM_LIMIT {
        return Err(ContractError::UnbondingItemLimitReached {});
    }

    let unbonding = &Unbonding {
        id: state.last_unbonding_id + 1,
        sender: sender.to_owned(),
        amount: amount * HOST_LP_RATE_MULTIPLIER / state.lp_redemption_rate,
        pending_start: false,
        start_time: 0u64,
        marked: false,
    };
    UNBONDINGS.save(deps.storage, unbonding.id, unbonding)?;

    // increase last unbonding id
    // NOTE: eventually, we should remove these params from config because it's simply double counting
    state.last_unbonding_id += 1;
    state.unbonding_lp_amount += unbonding.amount;
    if state.bonded_lp_amount < unbonding.amount {
        state.bonded_lp_amount = Uint128::from(0u128);
    } else {
        state.bonded_lp_amount = state
            .bonded_lp_amount
            .checked_sub(unbonding.amount)
            .unwrap_or(Uint128::from(0u128));
    }
    state.total_shares = state
        .total_shares
        .checked_sub(unstake_amount)
        .unwrap_or(Uint128::from(0u128));

    STATE.save(deps.storage, &state)?;

    let rsp = Response::new()
        .add_attribute("sender", sender.to_string())
        .add_attribute("amount", amount);
    Ok(rsp)
}
