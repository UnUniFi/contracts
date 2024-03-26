use crate::error::ContractError;
use crate::msgs::{phase_from_phase_step, ChannelInfo, UpdateParamsMsg, UpdateStateMsg};

use crate::state::{CHANNEL_INFO, PARAMS, STATE};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use ununifi_binding::v1::binding::UnunifiMsg;

/// Only authority can execute it.
pub fn execute_update_params(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateParamsMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut params = PARAMS.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let mut resp = Response::new().add_attribute("action", "update_params");
    if let Some(authority) = msg.authority {
        params.authority = deps.api.addr_validate(&authority)?;
        resp = resp.add_attribute("authority", authority.to_string());
    }
    if let Some(deposit_token) = msg.deposit_token {
        params.deposit_token = deposit_token.to_owned();
        resp = resp.add_attribute("deposit_token", deposit_token.to_string());
    }
    if let Some(unbond_period) = msg.unbond_period {
        params.unbond_period = unbond_period;
        resp = resp.add_attribute("unbond_period", unbond_period.to_string());
    }
    if let Some(pool_id) = msg.pool_id {
        params.pool_id = pool_id;
        resp = resp.add_attribute("pool_id", pool_id.to_string());
    }
    if let Some(ica_channel_id) = msg.ica_channel_id {
        let info: ChannelInfo = CHANNEL_INFO.load(deps.storage, ica_channel_id.as_str())?;
        params.ica_account = info.address.to_string();
        params.ica_channel_id = info.id;
        params.ica_connection_id = info.connection_id.to_string();
        resp = resp
            .add_attribute("ica_account", info.address.to_string())
            .add_attribute("ica_channel_id", ica_channel_id.to_string())
            .add_attribute("ica_connection_id", info.connection_id.to_string());
    }
    if let Some(phase) = msg.phase {
        params.phase = phase.to_owned();
        resp = resp.add_attribute("phase", phase.to_string());
    }
    if let Some(phase_step) = msg.phase_step {
        params.phase_step = phase_step.to_owned();
        resp = resp.add_attribute("phase_step", phase_step.to_string());
        if phase_from_phase_step(phase_step) != params.phase {
            return Err(ContractError::PhaseAndPhaseStepMismatch {});
        }
    }
    if let Some(transfer_timeout) = msg.transfer_timeout {
        params.transfer_timeout = transfer_timeout.to_owned();
        resp = resp.add_attribute("transfer_timeout", transfer_timeout.to_string());
    }
    if let Some(lp_denom) = msg.lp_denom {
        params.lp_denom = lp_denom.to_owned();
        resp = resp.add_attribute("lp_denom", lp_denom.to_string());
    }
    if let Some(transfer_channel_id) = msg.transfer_channel_id {
        params.transfer_channel_id = transfer_channel_id.to_owned();
        resp = resp.add_attribute("transfer_channel_id", transfer_channel_id.to_string());
    }
    if let Some(quote_denom) = msg.quote_denom {
        params.quote_denom = quote_denom.to_owned();
        resp = resp.add_attribute("quote_denom", quote_denom.to_string());
    }
    if let Some(base_denom) = msg.base_denom {
        params.base_denom = base_denom.to_owned();
        resp = resp.add_attribute("base_denom", base_denom.to_string());
    }
    if let Some(chain_id) = msg.chain_id {
        params.chain_id = chain_id.to_owned();
        resp = resp.add_attribute("chain_id", chain_id.to_string());
    }
    if let Some(deposit_denom) = msg.controller_deposit_denom {
        params.controller_deposit_denom = deposit_denom.to_owned();
        resp = resp.add_attribute("deposit_denom", params.controller_deposit_denom.to_string());
    }
    if let Some(transfer_channel_id) = msg.controller_transfer_channel_id {
        params.controller_transfer_channel_id = transfer_channel_id.to_owned();
        resp = resp.add_attribute("transfer_channel_id", transfer_channel_id.to_string());
    }

    if let Some(superfluid_validator) = msg.superfluid_validator {
        if state.bonded_lp_amount.is_zero()
            || !params.automate_superfluid
            || params.superfluid_validator == ""
        {
            params.superfluid_validator = superfluid_validator.to_owned();
            resp = resp.add_attribute("superfluid_validator", superfluid_validator.to_string());
        }
    }

    if let Some(automate_superfluid) = msg.automate_superfluid {
        params.automate_superfluid = automate_superfluid.to_owned();
        resp = resp.add_attribute("automate_superfluid", "true");
    }

    if let Some(extern_tokens) = msg.extern_tokens {
        params.extern_tokens = extern_tokens.to_owned();
        resp = resp.add_attribute("extern_tokens", "true");

        state.extern_token_amounts = vec![Uint128::from(0u128); extern_tokens.len()];
    }

    PARAMS.save(deps.storage, &params)?;
    STATE.save(deps.storage, &state)?;
    Ok(resp)
}

pub fn execute_update_state(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateStateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let mut resp = Response::new().add_attribute("action", "update_state");

    if let Some(bonded_lp_amount) = msg.bonded_lp_amount {
        state.bonded_lp_amount = bonded_lp_amount;
        resp = resp.add_attribute("bonded_lp_amount", bonded_lp_amount.to_string());
    }

    if let Some(unbonding_lp_amount) = msg.unbonding_lp_amount {
        state.unbonding_lp_amount = unbonding_lp_amount;
        resp = resp.add_attribute("unbonding_lp_amount", unbonding_lp_amount.to_string());
    }

    if let Some(total_shares) = msg.total_shares {
        state.total_shares = total_shares;
        resp = resp.add_attribute("total_shares", total_shares.to_string());
    }

    if let Some(total_deposit) = msg.total_deposit {
        state.total_deposit = total_deposit;
        resp = resp.add_attribute("total_deposit", total_deposit.to_string());
    }

    if let Some(total_withdrawn) = msg.total_withdrawn {
        state.total_withdrawn = total_withdrawn;
        resp = resp.add_attribute("total_withdrawn", total_withdrawn.to_string());
    }

    if let Some(pending_icq) = msg.pending_icq {
        state.pending_icq = pending_icq;
        resp = resp.add_attribute("pending_icq", pending_icq.to_string());
    }

    if let Some(unbond_request_lp_amount) = msg.unbond_request_lp_amount {
        state.unbond_request_lp_amount = unbond_request_lp_amount;
        resp = resp.add_attribute(
            "unbond_request_lp_amount",
            unbond_request_lp_amount.to_string(),
        );
    }

    if let Some(free_lp_amount) = msg.free_lp_amount {
        state.free_lp_amount = free_lp_amount;
        resp = resp.add_attribute("free_lp_amount", free_lp_amount.to_string());
    }

    if let Some(pending_bond_lp_amount) = msg.pending_bond_lp_amount {
        state.pending_bond_lp_amount = pending_bond_lp_amount;
        resp = resp.add_attribute("pending_bond_lp_amount", pending_bond_lp_amount.to_string());
    }

    if let Some(pending_lp_removal_amount) = msg.pending_lp_removal_amount {
        state.pending_lp_removal_amount = pending_lp_removal_amount;
        resp = resp.add_attribute(
            "pending_lp_removal_amount",
            pending_lp_removal_amount.to_string(),
        );
    }

    if let Some(free_quote_amount) = msg.free_quote_amount {
        state.free_quote_amount = free_quote_amount;
        resp = resp.add_attribute("free_quote_amount", free_quote_amount.to_string());
    }

    if let Some(free_base_amount) = msg.free_base_amount {
        state.free_base_amount = free_base_amount;
        resp = resp.add_attribute("free_base_amount", free_base_amount.to_string());
    }

    if let Some(controller_free_amount) = msg.controller_free_amount {
        state.controller_free_amount = controller_free_amount;
        resp = resp.add_attribute("controller_free_amount", controller_free_amount.to_string());
    }

    if let Some(controller_pending_transfer_amount) = msg.controller_pending_transfer_amount {
        state.controller_pending_transfer_amount = controller_pending_transfer_amount;
        resp = resp.add_attribute(
            "controller_pending_transfer_amount",
            controller_pending_transfer_amount.to_string(),
        );
    }

    if let Some(controller_stacked_amount_to_deposit) = msg.controller_stacked_amount_to_deposit {
        state.controller_stacked_amount_to_deposit = controller_stacked_amount_to_deposit;
        resp = resp.add_attribute(
            "controller_stacked_amount_to_deposit",
            controller_stacked_amount_to_deposit.to_string(),
        );
    }

    STATE.save(deps.storage, &state)?;
    Ok(resp)
}
