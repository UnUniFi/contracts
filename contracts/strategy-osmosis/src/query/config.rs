use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::DepositDenomResp;

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

pub fn query_deposit_denom(deps: Deps) -> StdResult<DepositDenomResp> {
    let config = CONFIG.load(deps.storage)?;
    Ok(DepositDenomResp {
        denom: config.controller_deposit_denom,
    })
}
