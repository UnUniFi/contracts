use crate::state::{TOTAL_DEPOSIT, TOTAL_SHARE};
use cosmwasm_std::{Deps, StdResult, Uint128};
use strategy::v1::msgs::{TotalShareResp, TotalDepositResp};

#[cfg(not(feature = "library"))]
pub fn query_total_share(deps: Deps) -> StdResult<TotalShareResp> {
    let total_share = TOTAL_SHARE.may_load(deps.storage)?
        .unwrap_or_else(|| Uint128::zero());

    Ok( TotalShareResp {
        total_share: total_share
    })
}

#[cfg(not(feature = "library"))]
pub fn query_total_deposit(deps: Deps) -> StdResult<TotalDepositResp> {
    let total_deposit = TOTAL_DEPOSIT.may_load(deps.storage)?
        .unwrap_or_else(|| Uint128::zero());

    Ok( TotalDepositResp {
        total_deopsit: total_deposit
    })
}

