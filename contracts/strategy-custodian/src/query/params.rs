use crate::state::PARAMS;
use crate::types::Params;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_params(deps: Deps) -> StdResult<Params> {
    let params: Params = PARAMS.load(deps.storage)?;
    Ok(params)
}
