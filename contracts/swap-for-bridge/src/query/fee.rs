use crate::error::ContractError;
use crate::msgs::EstimateFeeResp;
use crate::state::PARAMS;
use cosmwasm_std::Decimal;
use cosmwasm_std::Deps;
use cosmwasm_std::Uint128;

#[cfg(not(feature = "library"))]
pub fn query_estimate_fee(deps: Deps, amount: Uint128) -> Result<EstimateFeeResp, ContractError> {
    let params = PARAMS.load(deps.storage)?;

    let mut total_fee = Decimal::from_atomics(amount, 0)?
        .checked_mul(params.fee_rate)?
        .to_uint_floor();
    if let Some(max_fee) = params.max_fee {
        if max_fee < total_fee {
            total_fee = max_fee;
        }
    }
    if let Some(min_fee) = params.min_fee {
        if min_fee > total_fee {
            total_fee = min_fee;
        }
    }

    let lp_fee = Decimal::from_atomics(total_fee, 0)?
        .checked_mul(params.lp_fee_weight)?
        .to_uint_floor();
    let fee = total_fee.checked_sub(lp_fee)?;

    Ok(EstimateFeeResp {
        fee,
        lp_fee,
        total_fee,
    })
}
