use crate::error::ContractError;
use crate::icq::submit_icq_for_host;
use crate::msgs::{Phase, PhaseStep};
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{EpochCallSource, PARAMS, STATE, UNBONDINGS};
use cosmwasm_std::{DepsMut, Env, Response, StdResult, Storage, Uint128};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountInResponse;
use osmosis_std::types::osmosis::lockup::MsgLockTokensResponse;
use osmosis_std::types::osmosis::superfluid::MsgLockAndSuperfluidDelegateResponse;
use prost::Message;
use proto::cosmos::base::abci::v1beta1::TxMsgData;
use std::str::FromStr;
use ununifi_binding::v1::binding::UnunifiMsg;

use super::helpers::determine_ica_amounts;
use super::liquidity::execute_ica_join_swap_extern_amount_in;
use super::lockup::{
    execute_ica_begin_unbonding_lp_tokens, execute_ica_bond_liquidity,
    should_lock_and_superfluid_delegate,
};
use super::swap::{execute_ica_sell_extern_tokens, get_extern_token_sell_messages};
use super::token_transfer::execute_ibc_transfer_to_host;

pub fn calc_matured_unbondings(store: &dyn Storage, env: Env) -> StdResult<Uint128> {
    let params = PARAMS.load(store)?;
    let mut total_matured_unbondings = Uint128::new(0);
    let unbondings = query_unbondings(store, Some(UNBONDING_ITEM_LIMIT))?;
    for unbonding in unbondings {
        if unbonding.start_time + params.unbond_period < env.block.time.seconds() {
            total_matured_unbondings += unbonding.amount;
        }
    }
    Ok(total_matured_unbondings)
}

pub fn execute_deposit_phase_epoch(
    deps: DepsMut,
    env: Env,
    called_from: EpochCallSource,
    success: bool,
    ret: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;
    let mut rsp: Result<Response<UnunifiMsg>, ContractError> = Ok(Response::new());
    let mut next_phase = params.phase.to_owned();
    let mut next_phase_step = params.phase_step.to_owned();

    match params.phase_step {
        PhaseStep::IbcTransferToHost => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - ibc transfer to host for newly staked tokens
            let ica_amounts = determine_ica_amounts(params.to_owned(), state.to_owned());
            let to_transfer_to_host = ica_amounts.to_transfer_to_host;
            if to_transfer_to_host.is_zero() {
                next_phase_step = PhaseStep::RequestIcqAfterIbcTransferToHost;
            } else {
                rsp = execute_ibc_transfer_to_host(deps.storage, env);
                next_phase_step = PhaseStep::IbcTransferToHostCallback;
            }
        }
        PhaseStep::IbcTransferToHostCallback => {
            // handle Transfer callback
            if called_from == EpochCallSource::TransferCallback {
                let mut state = STATE.load(deps.storage)?;
                if success {
                    STATE.save(deps.storage, &state)?;
                    next_phase_step = PhaseStep::RequestIcqAfterIbcTransferToHost;
                } else {
                    // revert the state
                    state.controller_stacked_amount_to_deposit =
                        state.controller_pending_transfer_amount;
                    next_phase_step = PhaseStep::IbcTransferToHost;
                }
                state.controller_pending_transfer_amount = Uint128::from(0u128);
                STATE.save(deps.storage, &state)?;
            }
        }
        PhaseStep::RequestIcqAfterIbcTransferToHost => {
            // - icq balance of ica account when `Deposit` phase
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseIcqAfterIbcTransferToHost;
        }
        PhaseStep::ResponseIcqAfterIbcTransferToHost => {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                let msgs = get_extern_token_sell_messages(deps.storage)?;
                if msgs.len() > 0 {
                    next_phase_step = PhaseStep::SellExternTokens
                } else {
                    next_phase_step = PhaseStep::AddLiquidity;
                }
            }
        }
        PhaseStep::SellExternTokens => {
            rsp = execute_ica_sell_extern_tokens(deps.storage, env);
            next_phase_step = PhaseStep::SellExternTokensCallback;
        }
        PhaseStep::SellExternTokensCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = PhaseStep::RequestIcqAfterSellExternTokens;
                } else {
                    next_phase_step = PhaseStep::AddLiquidity;
                }
            }
        }
        PhaseStep::RequestIcqAfterSellExternTokens => {
            // - icq balance of ica account when `Deposit` phase
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseIcqAfterSellExternTokens;
        }
        PhaseStep::ResponseIcqAfterSellExternTokens => {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = PhaseStep::AddLiquidity;
            }
        }
        PhaseStep::AddLiquidity => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - add osmo and atom as liquidity in a single ica tx
            if state.free_base_amount.is_zero() && state.free_quote_amount.is_zero() {
                next_phase_step = PhaseStep::BeginUnbondingForPendingRequests;
            } else {
                rsp = execute_ica_join_swap_extern_amount_in(deps.storage, env);
                next_phase_step = PhaseStep::AddLiquidityCallback;
            }
        }
        PhaseStep::AddLiquidityCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut state = STATE.load(deps.storage)?;
                if success {
                    if let Some(ret_bytes) = ret {
                        let tx_msg_data_result = TxMsgData::decode(&ret_bytes[..]);
                        if let Ok(tx_msg_data) = tx_msg_data_result {
                            for data in tx_msg_data.data {
                                let msg_ret_result =
                                    MsgJoinSwapExternAmountInResponse::decode(&data.data[..]);
                                if let Ok(msg_ret) = msg_ret_result {
                                    let share_out_amount =
                                        Uint128::from_str(msg_ret.share_out_amount.as_str())?;
                                    state.pending_bond_lp_amount += share_out_amount;
                                }
                            }
                        }
                    }
                    next_phase_step = PhaseStep::BondLiquidity;
                } else {
                    next_phase_step = PhaseStep::AddLiquidity;
                }
                STATE.save(deps.storage, &state)?;
            }
        }
        PhaseStep::BondLiquidity => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - add liquidity ica tx
            let share_out_amount = state.pending_bond_lp_amount;
            if share_out_amount.is_zero() {
                next_phase_step = PhaseStep::BeginUnbondingForPendingRequests;
            } else {
                rsp = execute_ica_bond_liquidity(deps.storage, env);
                next_phase_step = PhaseStep::BondLiquidityCallback;
            }
        }
        PhaseStep::BondLiquidityCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut state = STATE.load(deps.storage)?;
                let pending_bond_lp_amount = state.pending_bond_lp_amount;
                if success {
                    if let Some(ret_bytes) = ret {
                        let tx_msg_data_result = TxMsgData::decode(&ret_bytes[..]);
                        if let Ok(tx_msg_data) = tx_msg_data_result {
                            if tx_msg_data.data.len() > 0 {
                                if should_lock_and_superfluid_delegate(
                                    params.superfluid_validator,
                                    state.bonded_lp_amount,
                                    params.automate_superfluid,
                                ) {
                                    // handle the case for MsgLockAndSuperfluidDelegate message
                                    let msg_ret_result =
                                        MsgLockAndSuperfluidDelegateResponse::decode(
                                            &tx_msg_data.data[0].data[..],
                                        );
                                    if let Ok(msg_ret) = msg_ret_result {
                                        state.lock_id = msg_ret.id;
                                    }
                                } else {
                                    // handle the case for MsgLockTokensResponse message
                                    let msg_ret_result = MsgLockTokensResponse::decode(
                                        &tx_msg_data.data[0].data[..],
                                    );
                                    if let Ok(msg_ret) = msg_ret_result {
                                        state.lock_id = msg_ret.id;
                                    }
                                }
                            }
                        }
                    }
                    state.bonded_lp_amount += pending_bond_lp_amount;
                    state.pending_bond_lp_amount = Uint128::from(0u128);
                    next_phase_step = PhaseStep::RequestIcqAfterBondLiquidity;
                } else {
                    next_phase_step = PhaseStep::BondLiquidity;
                }
                STATE.save(deps.storage, &state)?;
            }
        }
        PhaseStep::RequestIcqAfterBondLiquidity => {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseIcqAfterBondLiquidity;
        }
        PhaseStep::ResponseIcqAfterBondLiquidity => {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = PhaseStep::BeginUnbondingForPendingRequests;
            }
        }
        PhaseStep::BeginUnbondingForPendingRequests => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // Unbonding epoch operation
            // - begin lp unbonding on host through ica tx per unbonding epoch - per day probably - (if to unbond lp is not enough, wait for icq to update bonded lp correctly)
            let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
            let mut unbond_init_lp_amount = Uint128::from(0u128);
            for mut unbonding in unbondings {
                // restart unbonding for callback not received ones & new unbondings
                if unbonding.start_time != 0 && unbonding.pending_start == false {
                    continue;
                }
                unbonding.start_time = env.block.time.seconds();
                unbonding.pending_start = true;
                UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                unbond_init_lp_amount += unbonding.amount;
            }

            if !unbond_init_lp_amount.is_zero()
                && unbond_init_lp_amount <= state.bonded_lp_amount
                && unbond_init_lp_amount <= state.unbond_request_lp_amount
            {
                rsp =
                    execute_ica_begin_unbonding_lp_tokens(deps.storage, env, unbond_init_lp_amount);
                next_phase_step = PhaseStep::BeginUnbondingForPendingRequestsCallback;
            } else {
                next_phase_step = PhaseStep::CheckMaturedUnbondings;
            }
        }
        PhaseStep::BeginUnbondingForPendingRequestsCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    let mut unbond_init_lp_amount = Uint128::from(0u128);
                    let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.pending_start == true {
                            unbonding.start_time = env.block.time.seconds();
                            unbonding.pending_start = false;
                            UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                            unbond_init_lp_amount += unbonding.amount;
                        }
                    }
                    let mut state = STATE.load(deps.storage)?;
                    state.unbond_request_lp_amount -= unbond_init_lp_amount;
                    state.unbonding_lp_amount += unbond_init_lp_amount;
                    state.bonded_lp_amount = state
                        .bonded_lp_amount
                        .checked_sub(unbond_init_lp_amount)
                        .unwrap_or(Uint128::from(0u128));
                    STATE.save(deps.storage, &state)?;

                    next_phase_step = PhaseStep::CheckMaturedUnbondings;
                } else {
                    let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.start_time != 0 && unbonding.pending_start == true {
                            unbonding.start_time = 0;
                            unbonding.pending_start = false;
                            UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                        }
                    }
                    next_phase_step = PhaseStep::BeginUnbondingForPendingRequests;
                }
            }
        }
        _ => {
            // PhaseStep::CheckMaturedUnbondings
            // when free lp amount and matured unbondings exist, move to withdraw phase
            let matured_unbondings = calc_matured_unbondings(deps.storage, env)?;
            if !state.free_lp_amount.is_zero() && matured_unbondings > Uint128::from(0u128) {
                next_phase = Phase::Withdraw;
                next_phase_step = PhaseStep::RemoveLiquidity;
            } else {
                next_phase_step = PhaseStep::IbcTransferToHost;
            }
        }
    }

    // update phase
    let mut params = PARAMS.load(deps.storage)?;
    params.phase = next_phase;
    params.phase_step = next_phase_step;
    PARAMS.save(deps.storage, &params)?;
    return rsp;
}
