use crate::error::ContractError;
use crate::ica::{
    determine_ica_amounts, execute_ibc_transfer_to_controller, execute_ica_add_and_bond_liquidity,
    execute_ica_begin_unbonding_lp_tokens, execute_ica_remove_liquidity,
    execute_ica_swap_balance_to_two_tokens, execute_ica_swap_two_tokens_to_deposit_token,
};
use crate::icq::submit_icq_for_host;
use crate::query::{query_balance, query_unbondings, DEFAULT_LIMIT};
use crate::state::{Config, EpochCallSource, CONFIG, STAKE_RATE_MULTIPLIER, UNBONDINGS};
use cosmwasm_std::{
    coin, coins, BankMsg, CosmosMsg, DepsMut, Env, IbcTimeout, Response, StdResult, Storage,
    Uint128,
};
use osmosis_std::types::osmosis::lockup::MsgLockTokensResponse;
use prost::Message;
use proto::cosmos::base::abci::v1beta1::TxMsgData;
use strategy_osmosis_interface::strategy::Phase;
use ununifi_msg::v0::binding::UnunifiMsg;

pub fn calc_matured_unbondings(store: &dyn Storage, env: Env) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(store)?;
    let mut total_matured_unbondings = Uint128::new(0);
    let unbondings = query_unbondings(store, Some(DEFAULT_LIMIT))?;
    for unbonding in unbondings {
        if unbonding.start_time + config.unbond_period < env.block.time.seconds() {
            total_matured_unbondings += unbonding.amount;
        }
    }
    Ok(total_matured_unbondings)
}

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

    let mut rsp: Result<Response<UnunifiMsg>, ContractError> = Ok(Response::new());
    let mut next_phase = config.phase.to_owned();
    let mut next_phase_step = config.phase_step.to_owned();

    if config.phase == Phase::Withdraw {
        if config.phase_step == 1u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - Mark unbond ending queue items on contract
            // assumption: matured unbondings on the contract is same as matured unbondings on host chain
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            for mut unbonding in unbondings {
                if unbonding.start_time + config.unbond_period < env.block.time.seconds() {
                    unbonding.marked = true;
                    UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                }
            }
            // - execute remove liquidity operation
            rsp = execute_ica_remove_liquidity(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 2u64 {
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
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
                config.host_config.pending_lp_removal_amount = Uint128::from(0u128);
                CONFIG.save(deps.storage, &config)?;
            }
        } else if config.phase_step == 3u64 {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 4u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 5u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - swap full osmo to atom
            rsp = execute_ica_swap_two_tokens_to_deposit_token(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 6u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else if config.phase_step == 7u64 {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 8u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 9u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - ibc transfer full atom balance from ica to contract
            rsp = execute_ibc_transfer_to_controller(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 10u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else if config.phase_step == 11u64 {
            // - refresh balance of host chain after ibc transfer callback
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 12u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // 13u64
            // - calculate amount to return, contract balance - stacked atom balance for deposit
            let amount_to_return = config
                .controller_config
                .free_amount
                .checked_sub(config.controller_config.stacked_amount_to_deposit)
                .unwrap_or(Uint128::from(0u128));
            // - send amounts to marked unbond ending items proportionally
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            let mut total_marked_lp_amount = Uint128::from(0u128);
            for unbonding in unbondings.as_slice() {
                if unbonding.marked {
                    total_marked_lp_amount += unbonding.amount;
                }
            }
            if !total_marked_lp_amount.is_zero() {
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
            next_phase_step = 1u64;
        }
    } else {
        if config.phase_step == 1u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - ibc transfer to host for newly incoming atoms
            // - ibc transfer to host for stacked atoms during withdraw phases
            let ica_amounts = determine_ica_amounts(config.to_owned());
            let to_transfer_to_host = ica_amounts.to_transfer_to_host;
            if to_transfer_to_host.is_zero() {
                next_phase_step = config.phase_step + 2;
            } else {
                rsp = execute_ibc_transfer_to_host(deps.storage, env);
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 2u64 {
            // handle Transfer callback
            if called_from == EpochCallSource::TransferCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
                config.controller_config.pending_transfer_amount = Uint128::from(0u128);
                CONFIG.save(deps.storage, &config)?;
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 3u64 {
            // - icq balance of ica account when `Deposit` phase
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 4u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 5u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - swap half atom to osmo & half osmo to atom in a single ica tx
            let ica_amounts = determine_ica_amounts(config.to_owned());
            let to_swap_atom = ica_amounts.to_swap_base;
            let to_swap_osmo = ica_amounts.to_swap_quote;
            if to_swap_atom.is_zero() && to_swap_osmo.is_zero() {
                next_phase_step = config.phase_step + 2;
            } else {
                rsp = execute_ica_swap_balance_to_two_tokens(deps.storage, env);
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 6u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else if config.phase_step == 7u64 {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 8u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 9u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - add liquidity & bond in a single ica tx
            let share_out_amount = determine_ica_amounts(config.to_owned()).to_add_lp;
            if share_out_amount.is_zero() {
                next_phase_step = config.phase_step + 2;
            } else {
                rsp = execute_ica_add_and_bond_liquidity(deps.storage, env);
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 10u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
                let pending_bond_lp_amount = config.host_config.pending_bond_lp_amount;
                if success {
                    if let Some(ret_bytes) = ret {
                        let tx_msg_data_result = TxMsgData::decode(&ret_bytes[..]);
                        if let Ok(tx_msg_data) = tx_msg_data_result {
                            if tx_msg_data.data.len() > 1 {
                                let msg_ret_result =
                                    MsgLockTokensResponse::decode(&tx_msg_data.data[1].data[..]);
                                if let Ok(msg_ret) = msg_ret_result {
                                    config.host_config.lock_id = msg_ret.id;
                                }
                            }
                        }
                    }
                    config.host_config.bonded_lp_amount += pending_bond_lp_amount;
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
                config.host_config.pending_bond_lp_amount = Uint128::from(0u128);
                CONFIG.save(deps.storage, &config)?;
            }
        } else if config.phase_step == 11u64 {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 12u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 13u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // Unbonding epoch operation
            // - begin lp unbonding on host through ica tx per unbonding epoch - per day probably - (if to unbond lp is not enough, wait for icq to update bonded lp correctly)
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
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

            if !unbonding_lp_amount.is_zero() {
                rsp = execute_ica_begin_unbonding_lp_tokens(deps.storage, env, unbonding_lp_amount);
                next_phase_step = config.phase_step + 1;
            } else {
                next_phase_step = config.phase_step + 2;
            }
        } else if config.phase_step == 14u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.pending_start == true {
                            unbonding.start_time = env.block.time.seconds();
                            unbonding.pending_start = false;
                            UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                        }
                    }

                    next_phase_step = config.phase_step + 1;
                } else {
                    let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.start_time != 0 && unbonding.pending_start == true {
                            unbonding.pending_start = false;
                            UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                        }
                    }
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else {
            // 15u64
            // when free lp amount and matured unbondings exist, move to withdraw phase
            let matured_unbondings = calc_matured_unbondings(deps.storage, env)?;
            if !config.host_config.free_lp_amount.is_zero()
                && matured_unbondings > Uint128::from(0u128)
            {
                next_phase = Phase::Withdraw;
            }
            next_phase_step = 1u64;
        }
    }

    // update phase
    let mut config: Config = CONFIG.load(deps.storage)?;
    config.phase = next_phase;
    config.phase_step = next_phase_step;
    CONFIG.save(deps.storage, &config)?;
    return rsp;
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
