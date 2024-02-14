use crate::state::{Params, PARAMS, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::DepositDenomResp;

pub fn query_params(deps: Deps) -> StdResult<Params> {
    let params = PARAMS.load(deps.storage)?;
    Ok(params)
}

pub fn query_deposit_denom(deps: Deps) -> StdResult<DepositDenomResp> {
    let params = PARAMS.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    Ok(DepositDenomResp {
        denom: params.denom.to_string(),
        deposit_denom: params.ls_denom.to_string(),
        deposit_denom_rate: state.ls_redemption_rate.to_string(),
        target_chain_id: params.chain_id,
        target_chain_denom: params.ls_denom_native,
        target_chain_addr: "".to_string(),
    })
}
