use crate::state::VERIFICATIONS;
use crate::types::Verification;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_verifications(deps: Deps, address: String) -> StdResult<Vec<Verification>> {
    let address = deps.api.addr_validate(&address)?;
    let verifications = VERIFICATIONS
        .prefix(address)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| -> StdResult<_> { Ok(item?.1) })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(verifications)
}
