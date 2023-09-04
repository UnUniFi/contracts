use cosmwasm_std::{Deps, StdResult};
use strategy::v1::msgs::VersionResp;

#[cfg(not(feature = "library"))]
pub fn query_version(_deps: Deps) -> StdResult<VersionResp> {
    Ok(VersionResp { version: 1 })
}
