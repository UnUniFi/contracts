use crate::error::ContractError;
use crate::msgs::ReportProfitMsg;
use crate::state::TOTAL_DEPOSIT;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::nonpayable;

#[cfg(not(feature = "library"))]
pub fn execute_report_profit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ReportProfitMsg,
) -> Result<Response, ContractError> {
    let mut response = Response::new();
    nonpayable(&info)?;

    let total_deposit = TOTAL_DEPOSIT.load(deps.storage)?;

    let total_deposit = if msg.sign {
        total_deposit.checked_add(msg.profit)?
    } else {
        total_deposit.checked_sub(msg.profit)?
    };
    TOTAL_DEPOSIT.save(deps.storage, &total_deposit)?;

    response = response.add_attribute("action", "report_profit");
    Ok(response)
}
