use crate::error::{ContractError, NoDeposit};
use crate::state::{
    DepositInfo, Unbonding, CONFIG, DEPOSITS, HOST_LP_RATE_MULTIPLIER, STAKE_RATE_MULTIPLIER,
    UNBONDINGS,
};

use cosmwasm_std::{Addr, DepsMut, Response, StdResult, Uint128};
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let unstake_amount = amount * STAKE_RATE_MULTIPLIER / config.redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(unstake_amount)?,
                });
            }
            Err(NoDeposit {}.into())
        },
    )?;

    let unbonding = &Unbonding {
        id: config.last_unbonding_id + 1,
        sender: sender.to_owned(),
        amount: amount * HOST_LP_RATE_MULTIPLIER / config.host_config.lp_redemption_rate,
        pending_start: false,
        start_time: 0u64,
        marked: false,
    };
    UNBONDINGS.save(deps.storage, unbonding.id, unbonding)?;

    // increase last unbonding id
    // NOTE: eventually, we should remove these params from config because it's simply double counting
    config.last_unbonding_id += 1;
    config.host_config.unbonding_lp_amount += unbonding.amount;
    if config.host_config.bonded_lp_amount < unbonding.amount {
        config.host_config.bonded_lp_amount = Uint128::from(0u128);
    } else {
        config.host_config.bonded_lp_amount = config
            .host_config
            .bonded_lp_amount
            .checked_sub(unbonding.amount)
            .unwrap_or(Uint128::from(0u128));
    }
    config.total_shares = config
        .total_shares
        .checked_sub(unstake_amount)
        .unwrap_or(Uint128::from(0u128));

    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::new()
        .add_attribute("sender", sender.to_string())
        .add_attribute("amount", amount);
    Ok(rsp)
}
