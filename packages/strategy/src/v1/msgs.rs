use super::{ibc_hooks::IBCLifecycleComplete, transfer_callback::TransferCallbackData};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    Stake(StakeMsg),
    Unstake(UnstakeMsg),
    ExecuteEpoch(ExecuteEpochMsg),
}

#[cw_serde]
pub struct StakeMsg {}

#[cw_serde]
pub struct UnstakeMsg {
    pub amount: Uint128,
}

#[cw_serde]
pub struct ExecuteEpochMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(VersionResp)]
    Version {},
    #[returns(DepositDenomResp)]
    DepositDenom {},
    #[returns(AmountsResp)]
    Amounts { addr: String },
    #[returns(FeeResp)]
    Fee {},
}

#[cw_serde]
pub struct VersionResp {
    pub version: u8,
}

#[cw_serde]
pub struct DepositDenomResp {
    pub denom: String,
}

#[cw_serde]
pub struct AmountsResp {
    pub total_deposited: Uint128,
    pub not_bonded: Uint128,
    pub bonded: Uint128,
    pub unbonding: Uint128,
}

#[cw_serde]
pub struct FeeResp {
    pub deposit_fee_rate: Decimal,
    pub withdraw_fee_rate: Decimal,
    pub interest_fee_rate: Decimal,
}

#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "transfer_callback")]
    TransferCallback(TransferCallbackData),
    #[serde(rename = "icq_result")]
    ICQResult {
        // TBD
    },
    /// From ibc-hooks
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}
