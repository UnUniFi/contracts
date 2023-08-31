use crate::state::{Params, PARAMS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::DepositDenomResp;

pub fn query_params(deps: Deps) -> StdResult<Params> {
    let params = PARAMS.load(deps.storage)?;
    Ok(params)
}

pub fn query_deposit_denom(deps: Deps) -> StdResult<DepositDenomResp> {
    let params = PARAMS.load(deps.storage)?;
    Ok(DepositDenomResp {
        denom: params.controller_deposit_denom,
    })
}
