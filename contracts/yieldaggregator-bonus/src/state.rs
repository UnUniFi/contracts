use crate::types::{BonusWindow, Params, VaultShareStaking, VotedVault};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");

pub const BONUS_WINDOWS: Map<u64, BonusWindow> = Map::new("bonus_windows");

pub const VOTED_VAULTS: Map<(u64, u64), VotedVault> = Map::new("voted_vaults");

pub const VAULT_SHARE_STAKINGS: Map<(u64, Addr), VaultShareStaking> =
    Map::new("vault_share_stakings");
