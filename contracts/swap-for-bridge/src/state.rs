use crate::types::Params;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");
pub const TOTAL_SHARE: Item<Uint128> = Item::new("total_share");
pub const SHARES: Map<Addr, Uint128> = Map::new("shares");
