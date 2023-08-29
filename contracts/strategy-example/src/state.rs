use crate::types::{DepositInfo, Params};
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");

pub const DEPOSITS: Map<String, DepositInfo> = Map::new("deposits");
