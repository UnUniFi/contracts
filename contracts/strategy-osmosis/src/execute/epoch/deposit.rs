use crate::error::ContractError;
use crate::ica::{
    determine_ica_amounts, execute_ica_begin_unbonding_lp_tokens, execute_ica_bond_liquidity,
    execute_ica_join_swap_extern_amount_in,
};
use crate::icq::submit_icq_for_host;
use crate::msgs::{Phase, PhaseStep};
use crate::query::unbondings::{query_unbondings, UNBONDING_ITEM_LIMIT};
use crate::state::{Config, EpochCallSource, CONFIG, UNBONDINGS};
use cosmwasm_std::{coin, DepsMut, Env, IbcTimeout, Response, StdResult, Storage, Uint128};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountInResponse;
use osmosis_std::types::osmosis::lockup::MsgLockTokensResponse;
use prost::Message;
use proto::cosmos::base::abci::v1beta1::TxMsgData;
use std::str::FromStr;
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn calc_matured_unbondings(store: &dyn Storage, env: Env) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(store)?;
    let mut total_matured_unbondings = Uint128::new(0);
    let unbondings = query_unbondings(store, Some(UNBONDING_ITEM_LIMIT))?;
    for unbonding in unbondings {
        if unbonding.start_time + config.unbond_period < env.block.time.seconds() {
            total_matured_unbondings += unbonding.amount;
        }
    }
    Ok(total_matured_unbondings)
}

pub fn execute_ibc_transfer_to_host(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_transfer_to_host = ica_amounts.to_transfer_to_host;
    if to_transfer_to_host.is_zero() {
        return Ok(Response::new());
    }
    let timeout = env.block.time.plus_seconds(config.transfer_timeout);
    let ibc_msg = UnunifiMsg::IbcTransfer {
        channel_id: config.controller_config.transfer_channel_id,
        to_address: config.ica_account,
        amount: coin(
            to_transfer_to_host.u128(),
            config.controller_config.deposit_denom,
        ),
        timeout: IbcTimeout::from(timeout),
    };

    let mut config: Config = CONFIG.load(store)?;
    config.controller_config.stacked_amount_to_deposit = Uint128::from(0u128);
    config.controller_config.pending_transfer_amount += to_transfer_to_host;
    CONFIG.save(store, &config)?;

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", "ibc_transfer_to_host");
    Ok(res)
}

pub fn execute_deposit_phase_epoch(
    deps: DepsMut,
    env: Env,
    called_from: EpochCallSource,
    success: bool,
    ret: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    let mut rsp: Result<Response<UnunifiMsg>, ContractError> = Ok(Response::new());
    let mut next_phase = config.phase.to_owned();
    let mut next_phase_step = config.phase_step.to_owned();

    match config.phase_step {
        PhaseStep::IbcTransferToHost => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - ibc transfer to host for newly incoming atoms
            // - ibc transfer to host for stacked atoms during withdraw phases
            let ica_amounts = determine_ica_amounts(config.to_owned());
            let to_transfer_to_host = ica_amounts.to_transfer_to_host;
            if to_transfer_to_host.is_zero() {
                next_phase_step = PhaseStep::RequestICQAfterIbcTransferToHost;
            } else {
                rsp = execute_ibc_transfer_to_host(deps.storage, env);
                next_phase_step = PhaseStep::IbcTransferToHostCallback;
            }
        }
        PhaseStep::IbcTransferToHostCallback => {
            // handle Transfer callback
            if called_from == EpochCallSource::TransferCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
                config.controller_config.pending_transfer_amount = Uint128::from(0u128);
                CONFIG.save(deps.storage, &config)?;
                next_phase_step = PhaseStep::RequestICQAfterIbcTransferToHost;
            }
        }
        PhaseStep::RequestICQAfterIbcTransferToHost => {
            // - icq balance of ica account when `Deposit` phase
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseICQAfterIbcTransferToHost;
        }
        PhaseStep::ResponseICQAfterIbcTransferToHost => {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = PhaseStep::AddLiquidity;
            }
        }
        PhaseStep::AddLiquidity => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - swap half atom to osmo & half osmo to atom in a single ica tx
            let ica_amounts = determine_ica_amounts(config.to_owned());
            let to_swap_atom = ica_amounts.to_swap_base;
            let to_swap_osmo = ica_amounts.to_swap_quote;
            if to_swap_atom.is_zero() && to_swap_osmo.is_zero() {
                next_phase_step = PhaseStep::BondLiquidity;
            } else {
                rsp = execute_ica_join_swap_extern_amount_in(deps.storage, env);
                next_phase_step = PhaseStep::AddLiquidityCallback;
            }
        }
        PhaseStep::AddLiquidityCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
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
                                    config.host_config.pending_bond_lp_amount += share_out_amount;
                                }
                            }
                        }
                    }
                    next_phase_step = PhaseStep::BondLiquidity;
                } else {
                    next_phase_step = PhaseStep::AddLiquidity;
                }
                CONFIG.save(deps.storage, &config)?;
            }
        }
        PhaseStep::BondLiquidity => {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - add liquidity ica tx
            let share_out_amount = config.host_config.pending_bond_lp_amount;
            if share_out_amount.is_zero() {
                next_phase_step = PhaseStep::RequestICQAfterBondLiquidity;
            } else {
                rsp = execute_ica_bond_liquidity(deps.storage, env);
                next_phase_step = PhaseStep::BondLiquidityCallback;
            }
        }
        PhaseStep::BondLiquidityCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
                let pending_bond_lp_amount = config.host_config.pending_bond_lp_amount;
                if success {
                    if let Some(ret_bytes) = ret {
                        let tx_msg_data_result = TxMsgData::decode(&ret_bytes[..]);
                        if let Ok(tx_msg_data) = tx_msg_data_result {
                            if tx_msg_data.data.len() > 0 {
                                let msg_ret_result =
                                    MsgLockTokensResponse::decode(&tx_msg_data.data[0].data[..]);
                                if let Ok(msg_ret) = msg_ret_result {
                                    config.host_config.lock_id = msg_ret.id;
                                }
                            }
                        }
                    }
                    config.host_config.bonded_lp_amount += pending_bond_lp_amount;
                    config.host_config.pending_bond_lp_amount = Uint128::from(0u128);
                    next_phase_step = PhaseStep::RequestICQAfterBondLiquidity;
                } else {
                    next_phase_step = PhaseStep::BondLiquidity;
                }
                CONFIG.save(deps.storage, &config)?;
            }
        }
        PhaseStep::RequestICQAfterBondLiquidity => {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = PhaseStep::ResponseICQAfterBondLiquidity;
        }
        PhaseStep::ResponseICQAfterBondLiquidity => {
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
            let mut unbonding_lp_amount = Uint128::from(0u128);
            for mut unbonding in unbondings {
                if unbonding.start_time != 0 || unbonding.pending_start == true {
                    continue;
                }
                unbonding.start_time = env.block.time.seconds();
                unbonding.pending_start = true;
                UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                unbonding_lp_amount += unbonding.amount;
            }

            if !unbonding_lp_amount.is_zero()
                && unbonding_lp_amount <= config.host_config.bonded_lp_amount
            {
                rsp = execute_ica_begin_unbonding_lp_tokens(deps.storage, env, unbonding_lp_amount);
                next_phase_step = PhaseStep::BeginUnbondingForPendingRequestsCallback;
            } else {
                next_phase_step = PhaseStep::CheckMaturedUnbondings;
            }
        }
        PhaseStep::BeginUnbondingForPendingRequestsCallback => {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.pending_start == true {
                            unbonding.start_time = env.block.time.seconds();
                            unbonding.pending_start = false;
                            UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                        }
                    }

                    next_phase_step = PhaseStep::CheckMaturedUnbondings;
                } else {
                    let unbondings = query_unbondings(deps.storage, Some(UNBONDING_ITEM_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.start_time != 0 && unbonding.pending_start == true {
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
            if !config.host_config.free_lp_amount.is_zero()
                && matured_unbondings > Uint128::from(0u128)
            {
                next_phase = Phase::Withdraw;
            }
            next_phase_step = PhaseStep::RemoveLiquidity;
        }
    }

    // update phase
    let mut config: Config = CONFIG.load(deps.storage)?;
    config.phase = next_phase;
    config.phase_step = next_phase_step;
    CONFIG.save(deps.storage, &config)?;
    return rsp;
}