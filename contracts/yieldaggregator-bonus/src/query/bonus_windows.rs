use crate::state::BONUS_WINDOWS;
use crate::types::BonusWindow;
use cosmwasm_std::Order;
use cosmwasm_std::{Deps, StdResult};

#[cfg(not(feature = "library"))]
pub fn query_bonus_windows(deps: Deps) -> StdResult<Vec<BonusWindow>> {
    let bonus_windows = BONUS_WINDOWS
        .range(deps.storage, None, None, Order::Descending)
        .map(|item| -> StdResult<_> { Ok(item?.1) })
        .collect::<StdResult<Vec<_>>>()?;
    Ok(bonus_windows)
}
