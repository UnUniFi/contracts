use crate::types::{BonusWindow, Params, VaultShareStaking};
use cosmwasm_std::{Addr, Uint128, Decimal};
use cw_storage_plus::{Item, Map};

pub const PARAMS: Item<Params> = Item::new("params");

pub const BONUS_WINDOWS: Map<u64, BonusWindow> = Map::new("bonus_windows");

pub const VOTED_VAULTS: Map<(BonusWindowId, VaultId), Uint128> = Map::new("voted_vaults");

pub const VAULT_SHARE_STAKINGS: Map<(BonusWindowId, VaultId, Addr), VaultShareStaking> =
    Map::new("vault_share_stakings");

pub const TOTAL_STAKING_INFO: Map<(BonusWindowId, VaultId), (TotalStakedAmount, TotalStakingPowerIndex)> =
    Map::new("total_staking_power_index");

pub type BonusWindowId = u64;
pub type VaultId = u64;
pub type TotalStakedAmount = Uint128;
pub type TotalStakingPowerIndex = Decimal;
