use crate::msgs::ShareResp;
use crate::state::SHARES;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_share(deps: Deps, address: String) -> StdResult<ShareResp> {
    let addr = deps.api.addr_validate(&address)?;
    let share = SHARES.load(deps.storage, addr)?;

    Ok(ShareResp { share })
}
