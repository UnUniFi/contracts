use crate::state::{Config, CONFIG, DEPOSITS, STAKE_RATE_MULTIPLIER};
use cosmwasm_std::{Deps, StdResult, Uint128};

pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let deposit = DEPOSITS.load(deps.storage, addr)?;
    Ok(deposit.amount * config.redemption_rate / STAKE_RATE_MULTIPLIER)
}
