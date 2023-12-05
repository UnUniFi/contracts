use crate::error::{ContractError, NoDeposit};
use crate::msgs::UpdateLegacyUnbondingRecipientsMsg;
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{
    DepositInfo, Unbonding, DEPOSITS, HOST_LP_RATE_MULTIPLIER, PARAMS, STAKE_RATE_MULTIPLIER,
    STATE, UNBONDINGS,
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use strategy::v1::msgs::UnstakeMsg;
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_unstake(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    msg: UnstakeMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let amount = msg.share_amount;
    let sender = info.sender;
    let recipient = msg.recipient;
    let mut state = STATE.load(deps.storage)?;
    let share_amount = amount * STAKE_RATE_MULTIPLIER / state.redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    share: unwrapped.share.checked_sub(share_amount)?,
                });
            }
            Err(NoDeposit {}.into())
        },
    )?;

    let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
    if unbondings.len() as u32 >= UNBONDING_ITEM_LIMIT {
        return Err(ContractError::UnbondingItemLimitReached {});
    }

    let mut recipient_addr = sender.to_owned();
    if let Some(recipient_str) = recipient {
        recipient_addr = deps.api.addr_validate(recipient_str.as_str())?;
    }

    let unbonding = &Unbonding {
        id: state.last_unbonding_id + 1,
        sender: recipient_addr.to_owned(),
        amount: amount * HOST_LP_RATE_MULTIPLIER / state.lp_redemption_rate,
        pending_start: false,
        start_time: 0u64,
        marked: false,
    };
    UNBONDINGS.save(deps.storage, unbonding.id, unbonding)?;

    // increase last unbonding id
    // NOTE: eventually, we should remove these params from params because it's simply double counting
    state.last_unbonding_id += 1;
    state.unbonding_lp_amount += unbonding.amount;
    if state.bonded_lp_amount < unbonding.amount {
        state.bonded_lp_amount = Uint128::from(0u128);
    } else {
        state.bonded_lp_amount = state
            .bonded_lp_amount
            .checked_sub(unbonding.amount)
            .unwrap_or(Uint128::from(0u128));
    }
    state.total_shares = state
        .total_shares
        .checked_sub(share_amount)
        .unwrap_or(Uint128::from(0u128));

    STATE.save(deps.storage, &state)?;

    let rsp = Response::new()
        .add_attribute("action", "unstake")
        .add_attribute("unbonding_id", state.last_unbonding_id.to_string())
        .add_attribute("sender", sender.to_string())
        .add_attribute("recipient", recipient_addr.to_string())
        .add_attribute("amount", amount)
        .add_attribute("share_amount", share_amount);
    Ok(rsp)
}

pub fn execute_update_unbonding_recipients(
    deps: DepsMut,
    _: Env,
    info: MessageInfo,
    msg: UpdateLegacyUnbondingRecipientsMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(deps.storage)?;
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }
    for update in msg.updates {
        let recipient = deps.api.addr_validate(update.recipient.as_str())?;
        UNBONDINGS.update(
            deps.storage,
            update.unbonding_id,
            |unbonding: Option<Unbonding>| -> StdResult<_> {
                if let Some(mut unwrapped) = unbonding {
                    unwrapped.sender = recipient;
                    return Ok(unwrapped);
                }
                Err(cosmwasm_std::StdError::NotFound {
                    kind: "not available unbonding".to_string(),
                })
            },
        )?;
    }

    let rsp = Response::new().add_attribute("action", "update_unbonding");
    Ok(rsp)
}
