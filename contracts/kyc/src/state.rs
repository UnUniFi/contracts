use crate::types::{Params, Provider, Verification};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");

pub const PROVIDER_ID: Item<u64> = Item::new("provider_id");
pub const PROVIDERS: Map<u64, Provider> = Map::new("providers");

pub const VERIFICATIONS: Map<(Addr, u64), Verification> = Map::new("verifications");
