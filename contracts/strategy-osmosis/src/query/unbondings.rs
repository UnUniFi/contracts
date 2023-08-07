use crate::state::{Unbonding, UNBONDINGS};
use cosmwasm_std::{Order, StdResult, Storage};

pub const DEFAULT_LIMIT: u32 = 50;
pub fn query_unbondings(storage: &dyn Storage, limit: Option<u32>) -> StdResult<Vec<Unbonding>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT) as usize;

    UNBONDINGS
        .range(storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, v) = item?;
            Ok(v)
        })
        .collect::<StdResult<Vec<Unbonding>>>()
}
