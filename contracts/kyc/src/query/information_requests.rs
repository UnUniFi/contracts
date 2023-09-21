use crate::state::INFORMATION_REQUESTS;
use crate::types::InformationRequest;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_information_requests(
    deps: Deps,
    address: String,
) -> StdResult<Vec<InformationRequest>> {
    let address = deps.api.addr_validate(&address)?;
    let information_requests = INFORMATION_REQUESTS
        .prefix(address)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| -> StdResult<_> { Ok(item?.1) })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(information_requests)
}
