use crate::error::ContractError;
use crate::msgs::SendBackMsg;
use crate::state::PARAMS;
use crate::state::UNBONDINGS;
use crate::state::{TOTAL_DEPOSIT, TOTAL_SHARE};
use cosmwasm_std::{coins, BankMsg, CosmosMsg, Uint128};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cosmwasm_std::{Order, StdResult};
use cw_utils::one_coin;

#[cfg(not(feature = "library"))]
pub fn execute_send_back(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: SendBackMsg,
) -> Result<Response, ContractError> {
    use crate::state::TOTAL_UNBONDING_SHARE;

    let mut response = Response::new();
    let coin = one_coin(&info)?;

    let params = PARAMS.load(deps.storage)?;
    // Permission check
    if info.sender != params.admin {
        return Err(ContractError::Unauthorized {});
    }
    if params.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let total_deposit = TOTAL_DEPOSIT.load(deps.storage)?;
    let total_share = TOTAL_SHARE.load(deps.storage)?;
    let total_unbonding_share = TOTAL_UNBONDING_SHARE.load(deps.storage)?;

    let unstake_requests = UNBONDINGS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    let mut total_unstaked_deposit = Uint128::new(0);
    let mut total_unstaked_share = Uint128::new(0);

    for (addr, unstake_request) in unstake_requests {
        let amount = unstake_request
            .share
            .checked_mul(total_deposit)?
            .checked_div(total_share)
            .unwrap();

        if !amount.is_zero() {
            let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
                to_address: addr.to_string(),
                amount: coins(amount.u128(), &params.deposit_denom),
            });
            response = response.add_message(bank_send_msg);
        }

        UNBONDINGS.remove(deps.storage, addr);

        total_unstaked_deposit = total_unstaked_deposit.checked_add(amount)?;
        total_unstaked_share = total_unstaked_share.checked_add(unstake_request.share)?;
    }

    TOTAL_DEPOSIT.save(
        deps.storage,
        &(total_deposit.checked_sub(total_unstaked_deposit)?),
    )?;
    TOTAL_SHARE.save(
        deps.storage,
        &(total_share.checked_sub(total_unstaked_share)?),
    )?;
    TOTAL_UNBONDING_SHARE.save(
        deps.storage,
        &(total_unbonding_share.checked_sub(total_unstaked_share)?),
    )?;

    let remaining_amount = coin.amount.checked_sub(total_unstaked_deposit)?;
    if !remaining_amount.is_zero() {
        let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(remaining_amount.u128(), &params.deposit_denom),
        });
        response = response.add_message(bank_send_msg);
    }

    response = response
        .add_attribute("action", "send_back")
        .add_attribute("amount", coin.amount);
    Ok(response)
}
