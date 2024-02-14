use crate::state::{DepositToken, Params, PARAMS};
use cosmwasm_std::Decimal;

#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::DepositDenomResp;

pub fn query_params(deps: Deps) -> StdResult<Params> {
    let params = PARAMS.load(deps.storage)?;
    Ok(params)
}

pub fn query_deposit_denom(deps: Deps) -> StdResult<DepositDenomResp> {
    let params = PARAMS.load(deps.storage)?;
    let mut target_deposit_denom = params.base_denom;
    if params.deposit_token == DepositToken::Quote {
        target_deposit_denom = params.quote_denom;
    }

    Ok(DepositDenomResp {
        denom: params.controller_deposit_denom.to_string(),
        deposit_denom: params.controller_deposit_denom.to_string(),
        deposit_denom_rate: Decimal::one().to_string(),
        target_chain_id: params.chain_id,
        target_chain_denom: target_deposit_denom,
        target_chain_addr: params.ica_account,
    })
}
