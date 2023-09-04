use crate::{state::PARAMS, types::Params};
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::FeeResp;

#[cfg(not(feature = "library"))]
pub fn query_fee(deps: Deps) -> StdResult<FeeResp> {
    let params: Params = PARAMS.load(deps.storage)?;

    Ok(FeeResp {
        performance_fee_rate: params.performance_fee_rate,
        withdraw_fee_rate: params.withdraw_fee_rate,
        min_withdraw_fee: params.min_withdraw_fee,
        max_withdraw_fee: params.max_withdraw_fee,
    })
}
