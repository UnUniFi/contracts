use cosmwasm_std::{Decimal, Deps, StdResult};
use strategy::v0::FeeResp;

#[cfg(not(feature = "library"))]
pub fn query_fee(_: Deps) -> StdResult<FeeResp> {
    Ok(FeeResp {
        deposit_fee_rate: Decimal::zero(),
        withdraw_fee_rate: Decimal::zero(),
        interest_fee_rate: Decimal::zero(),
    })
}
