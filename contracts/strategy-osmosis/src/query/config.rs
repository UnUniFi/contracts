use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}
