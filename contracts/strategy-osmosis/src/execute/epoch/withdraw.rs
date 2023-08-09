use crate::error::ContractError;
use crate::ica::{
    execute_ibc_transfer_to_controller, execute_ica_remove_liquidity,
    execute_ica_swap_two_tokens_to_deposit_token,
};
use crate::icq::submit_icq_for_host;
use crate::msgs::{Phase, PhaseStep};
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{Config, EpochCallSource, CONFIG, UNBONDINGS};
use cosmwasm_std::{coins, BankMsg, CosmosMsg, DepsMut, Env, Response, Uint128};
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn execute_withdraw_phase_epoch(
    deps: DepsMut,
    env: Env,
    called_from: EpochCallSource,
    success: bool,
    _: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let mut rsp: Result<Response<UnunifiMsg>, ContractError> = Ok(Response::new());
    let mut next_phase = config.phase.to_owned();
    let mut next_phase_step = config.phase_step.to_owned();

    match config.phase_step {
        PhaseStep::RemoveLiquidity => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - Mark unbond ending queue items on contract
            // assumption: matured unbondings on the contract is same as matured unbondings on host chain
            let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
            for mut unbonding in unbondings {
                if unbonding.start_time + config.unbond_period < env.block.time.seconds() {
                    unbonding.marked = true;
                    UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                }
            }
            // - execute remove liquidity operation
            rsp = execute_ica_remove_liquidity(deps.storage, env);
            next_phase_step = PhaseStep::RemoveLiquidityCallback;
        }
        PhaseStep::RemoveLiquidityCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
                let pending_lp_removal_amount = config.host_config.pending_lp_removal_amount;
                if success {
                    config.host_config.unbonding_lp_amount = config
                        .host_config
                        .unbonding_lp_amount
                        .checked_sub(pending_lp_removal_amount)
                        .unwrap_or(Uint128::from(0u128));
                    next_phase_step = PhaseStep::RequestICQAfterRemoveLiquidity;
                } else {
                    next_phase_step = PhaseStep::RemoveLiquidity;
                }
                config.host_config.pending_lp_removal_amount = Uint128::from(0u128);
                CONFIG.save(deps.storage, &config)?;
            }
        }
        PhaseStep::RequestICQAfterRemoveLiquidity => {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseICQAfterRemoveLiquidity;
        }
        PhaseStep::ResponseICQAfterRemoveLiquidity => {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = PhaseStep::SwapTwoTokensToDepositToken;
            }
        }
        PhaseStep::SwapTwoTokensToDepositToken => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - swap full osmo to atom
            rsp = execute_ica_swap_two_tokens_to_deposit_token(deps.storage, env);
            next_phase_step = PhaseStep::SwapTwoTokensToDepositTokenCallback;
        }
        PhaseStep::SwapTwoTokensToDepositTokenCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = PhaseStep::RequestICQAfterSwapTwoTokensToDepositToken;
                } else {
                    next_phase_step = PhaseStep::SwapTwoTokensToDepositToken;
                }
            }
        }
        PhaseStep::RequestICQAfterSwapTwoTokensToDepositToken => {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseICQAfterSwapTwoTokensToDepositToken;
        }
        PhaseStep::ResponseICQAfterSwapTwoTokensToDepositToken => {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = PhaseStep::IbcTransferToController;
            }
        }
        PhaseStep::IbcTransferToController => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - ibc transfer full atom balance from ica to contract
            rsp = execute_ibc_transfer_to_controller(deps.storage, env);
            next_phase_step = PhaseStep::IbcTransferToControllerCallback;
        }
        PhaseStep::IbcTransferToControllerCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = PhaseStep::RequestICQAfterIbcTransferToController;
                } else {
                    next_phase_step = PhaseStep::IbcTransferToController;
                }
            }
        }
        PhaseStep::RequestICQAfterIbcTransferToController => {
            // - refresh balance of host chain after ibc transfer callback
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseICQAfterIbcTransferToController;
        }
        PhaseStep::ResponseICQAfterIbcTransferToController => {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = PhaseStep::DistributeToUnbondedUsers;
            }
        }
        _ => {
            // PhaseStep::DistributeToUnbondedUsers
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - calculate amount to return, contract balance - stacked atom balance for deposit
            let amount_to_return = config
                .controller_config
                .free_amount
                .checked_sub(config.controller_config.stacked_amount_to_deposit)
                .unwrap_or(Uint128::from(0u128));
            // - send amounts to marked unbond ending items proportionally
            let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
            let mut total_marked_lp_amount = Uint128::from(0u128);
            for unbonding in unbondings.as_slice() {
                if unbonding.marked {
                    total_marked_lp_amount += unbonding.amount;
                }
            }
            if !total_marked_lp_amount.is_zero() {
                let mut config: Config = CONFIG.load(deps.storage)?;
                let mut resp: Response<UnunifiMsg> = Response::new();
                for unbonding in unbondings {
                    if unbonding.marked {
                        let returning_amount =
                            amount_to_return * unbonding.amount / total_marked_lp_amount;
                        let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
                            to_address: unbonding.sender.to_string(),
                            amount: coins(
                                returning_amount.into(),
                                &config.controller_config.deposit_denom,
                            ),
                        });
                        resp = resp.add_message(bank_send_msg);
                        UNBONDINGS.remove(deps.storage, unbonding.id);
                        // update the total_withdrawn amount in config just for the record
                        // memo: this param can be deleted in the future
                        config.total_withdrawn += returning_amount;
                    }
                }
                CONFIG.save(deps.storage, &config)?;
                rsp = Ok(resp);
            }
            // - switch to `Deposit` phase
            next_phase = Phase::Deposit;
            next_phase_step = PhaseStep::IbcTransferToHost;
        }
    }

    // update phase
    let mut config: Config = CONFIG.load(deps.storage)?;
    config.phase = next_phase;
    config.phase_step = next_phase_step;
    CONFIG.save(deps.storage, &config)?;
    return rsp;
}
