use crate::state::{BONDEDS, TOTAL_DEPOSIT, TOTAL_SHARE};
use cosmwasm_std::{Deps, StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let addr = deps.api.addr_validate(&addr)?;
    let bonded = BONDEDS.load(deps.storage, addr)?;

    let total_deposit = TOTAL_DEPOSIT.load(deps.storage)?;
    let total_share = TOTAL_SHARE.load(deps.storage)?;

    let amount = if total_share.is_zero() {
        Uint128::zero()
    } else {
        bonded.share * total_deposit / total_share
    };

    Ok(amount)
}
