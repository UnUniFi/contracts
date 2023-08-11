use crate::state::{DEPOSITS, STAKE_RATE_MULTIPLIER, STATE};
use cosmwasm_std::{Deps, StdResult, Uint128};

pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let state = STATE.load(deps.storage)?;
    let deposit: crate::state::DepositInfo = DEPOSITS.load(deps.storage, addr)?;
    Ok(deposit.amount * state.redemption_rate / STAKE_RATE_MULTIPLIER)
}
