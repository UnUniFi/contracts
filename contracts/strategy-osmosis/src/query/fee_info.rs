use crate::msgs::FeeInfo;

use cosmwasm_std::{Decimal, Deps, StdResult};

pub fn query_fee_info(_: Deps) -> StdResult<FeeInfo> {
    Ok(FeeInfo {
        deposit_fee_rate: Decimal::zero(),
        withdraw_fee_rate: Decimal::zero(),
        interest_fee_rate: Decimal::zero(),
    })
}
