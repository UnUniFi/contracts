use crate::state::{CONFIG, DEPOSITS};
use crate::types::Config;
use cosmwasm_std::{Deps, StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let deposit = DEPOSITS.load(deps.storage, addr)?;
    let redemption_rate_multiplier = Uint128::from(1000000u128);
    Ok(deposit.amount * config.redemption_rate / redemption_rate_multiplier)
}
