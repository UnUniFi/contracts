use crate::types::{Bonded, Params, Unbonding};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");

pub const TOTAL_DEPOSIT: Item<Uint128> = Item::new("total_deposit");
pub const TOTAL_SHARE: Item<Uint128> = Item::new("total_share");
pub const TOTAL_UNBONDING_SHARE: Item<Uint128> = Item::new("total_unbonding_share");

pub const BONDEDS: Map<Addr, Bonded> = Map::new("bondeds");
pub const UNBONDINGS: Map<Addr, Unbonding> = Map::new("unbondings");
