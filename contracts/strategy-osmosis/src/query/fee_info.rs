use cosmwasm_std::{Decimal, Deps, StdResult};
use strategy::v1::msgs::FeeResp;

pub fn query_fee_info(_: Deps) -> StdResult<FeeResp> {
    Ok(FeeResp {
        performance_fee_rate: Decimal::zero(),
        withdraw_fee_rate: Decimal::zero(),
        min_withdraw_fee: None,
        max_withdraw_fee: None,
    })
}
