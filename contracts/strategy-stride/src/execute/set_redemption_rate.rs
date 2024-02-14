use crate::msgs::SetRedemptionRateMsg;
use crate::state::STATE;
use crate::{error::ContractError, state::PARAMS};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
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
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}
