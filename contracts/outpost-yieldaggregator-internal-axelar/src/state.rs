use crate::types::Params;
use cw_storage_plus::Item;

pub const PARAMS: Item<Params> = Item::new("params");
