use crate::msgs::{ShareResp, TotalShareResp};
use crate::state::{SHARES, TOTAL_SHARE};
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_share(deps: Deps, address: String) -> StdResult<ShareResp> {
    let addr = deps.api.addr_validate(&address)?;
    let share = SHARES.load(deps.storage, addr)?;

    Ok(ShareResp { share })
}

#[cfg(not(feature = "library"))]
pub fn query_total_share(deps: Deps) -> StdResult<TotalShareResp> {
    let total_share = TOTAL_SHARE.load(deps.storage)?;

    Ok(TotalShareResp { total_share })
}