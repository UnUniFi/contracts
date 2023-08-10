use crate::state::{State, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};

pub fn query_state(deps: Deps) -> StdResult<State> {
    let state = STATE.load(deps.storage)?;
    Ok(state)
}
