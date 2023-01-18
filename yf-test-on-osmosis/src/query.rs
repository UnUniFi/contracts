#[cfg(not(feature = "library"))]
use crate::msg::GetDepositorShareAmountResponse;
use crate::state::DEPOSITOR_SHARE;
use cosmwasm_std::{Deps, StdResult};

pub fn query_depositor_share_amount(
    deps: Deps,
    depositor: &str,
) -> StdResult<GetDepositorShareAmountResponse> {
    let depositor = deps.api.addr_validate(depositor)?;
    let depositor_share_amount = DEPOSITOR_SHARE.load(deps.storage, &depositor)?;

    Ok(GetDepositorShareAmountResponse {
        share_amount: depositor_share_amount,
    })
}
