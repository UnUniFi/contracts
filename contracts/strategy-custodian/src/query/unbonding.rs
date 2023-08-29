use cosmwasm_std::{Deps, StdResult, Uint128};

#[cfg(not(feature = "library"))]
pub fn query_unbonding(_: Deps, _: String) -> StdResult<Uint128> {
    Ok(Uint128::from(0u128))
}
