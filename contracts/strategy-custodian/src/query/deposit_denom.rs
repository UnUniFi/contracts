use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::Decimal;
use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::DepositDenomResp;

#[cfg(not(feature = "library"))]
pub fn query_deposit_denom(deps: Deps) -> StdResult<DepositDenomResp> {
    let params: Params = PARAMS.load(deps.storage)?;
    Ok(DepositDenomResp {
        denom: params.deposit_denom.to_string(),
        deposit_denom: params.deposit_denom.to_string(),
        deposit_denom_rate: Decimal::one().to_string(),
        target_chain_id: "".to_string(),
        target_chain_denom: "".to_string(),
        target_chain_addr: "".to_string(),
    })
}
