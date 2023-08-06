use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    pub deposit_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        unbond_period: Option<u64>,
        deposit_denom: Option<String>,
    },
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    AddRewards(AddRewardsMsg),
}

#[cw_serde]
pub struct StakeMsg {}

#[cw_serde]
pub struct UnstakeMsg {
    pub amount: Uint128,
}

#[cw_serde]
pub struct AddRewardsMsg {}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Bonded { addr: String },
    Unbonding { addr: String },
    Fee {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
