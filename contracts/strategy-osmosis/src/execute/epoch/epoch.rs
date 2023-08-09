use crate::error::ContractError;
use crate::helpers::query_balance;
use crate::msgs::Phase;
use crate::state::{Config, EpochCallSource, CONFIG, STAKE_RATE_MULTIPLIER};
use cosmwasm_std::{DepsMut, Env, Response};
use ununifi_binding::v0::binding::UnunifiMsg;

use super::deposit::execute_deposit_phase_epoch;
use super::withdraw::execute_withdraw_phase_epoch;

pub fn execute_epoch(
    deps: DepsMut,
    env: Env,
    called_from: EpochCallSource,
    success: bool,
    ret: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if let Ok(balance) = query_balance(
        &deps.querier,
        env.contract.address.to_owned(),
        config.controller_config.deposit_denom.to_string(),
    ) {
        config.controller_config.free_amount = balance;
        CONFIG.save(deps.storage, &config)?;
    }

    // recalculate redemption rate on every icq callback
    if called_from == EpochCallSource::IcqCallback {
        if config.total_shares.is_zero() {
            config.redemption_rate = STAKE_RATE_MULTIPLIER;
        } else {
            // active tvl is not unbonding tvl that is allocated to shares
            let mut active_tvl =
                config.host_config.bonded_lp_amount * config.host_config.lp_redemption_rate;
            active_tvl += config.controller_config.stacked_amount_to_deposit
                + config.controller_config.pending_transfer_amount;
            if config.phase == Phase::Deposit {
                active_tvl += config.host_config.free_base_amount;
            }
            config.redemption_rate = active_tvl * STAKE_RATE_MULTIPLIER / config.total_shares;
        }
        CONFIG.save(deps.storage, &config)?;
    }

    if config.phase == Phase::Withdraw {
        return execute_withdraw_phase_epoch(deps, env, called_from, success, ret);
    }
    return execute_deposit_phase_epoch(deps, env, called_from, success, ret);
}
