use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::DepositDenomResp;

#[cfg(not(feature = "library"))]
pub fn query_deposit_denom(deps: Deps) -> StdResult<DepositDenomResp> {
    let params: Params = PARAMS.load(deps.storage)?;
    Ok(DepositDenomResp {
        denom: params.deposit_denom,
    })
}
