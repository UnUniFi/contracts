use crate::types::{ChannelInfo, Config, DepositInfo};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

pub const DEPOSITS: Map<String, DepositInfo> = Map::new("deposits");

pub const CHANNEL_INFO: Map<&str, ChannelInfo> = Map::new("channel_info");
