use crate::{state::PARAMS, types::Params};
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_params(deps: Deps) -> StdResult<Params> {
    let config: Params = PARAMS.load(deps.storage)?;

    Ok(config)
}
