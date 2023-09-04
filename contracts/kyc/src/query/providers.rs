use crate::state::PROVIDERS;
use crate::types::Provider;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_providers(deps: Deps) -> StdResult<Vec<Provider>> {
    let providers = PROVIDERS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| -> StdResult<_> { Ok(item?.1) })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(providers)
}
