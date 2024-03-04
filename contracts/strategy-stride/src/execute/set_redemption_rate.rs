use crate::msgs::SetRedemptionRateMsg;
use crate::state::STATE;
use crate::{error::ContractError, state::PARAMS};
use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, Response};
use std::ops::Mul;
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_set_redemption_rate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SetRedemptionRateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(deps.storage)?;

    // Permission check
    if info.sender != params.ls_rate_feeder {
        return Err(ContractError::Unauthorized {});
    }

    let mut state = STATE.load(deps.storage)?;
    state.ls_redemption_rate = msg.ls_rate;
    let ls_yield = (msg.ls_rate - msg.last_ls_rate) / msg.last_ls_rate;
    let apy = ls_yield.mul(Decimal::from_atomics(1460u128, 0).unwrap()); // 365 * 24 / 6
    state.ls_denom_apy = apy;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}
