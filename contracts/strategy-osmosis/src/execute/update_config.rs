use crate::error::ContractError;
use crate::msgs::{ChannelInfo, UpdateConfigMsg};

use crate::state::{CHANNEL_INFO, CONFIG, STATE};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use ununifi_binding::v0::binding::UnunifiMsg;

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    let mut resp = Response::new().add_attribute("action", "update_config");
    if let Some(owner) = msg.owner {
        config.owner = deps.api.addr_validate(&owner)?;
        resp = resp.add_attribute("owner", owner.to_string());
    }
    if let Some(deposit_token) = msg.deposit_token {
        config.deposit_token = deposit_token.to_owned();
        resp = resp.add_attribute("deposit_token", deposit_token.to_string());
    }
    if let Some(unbond_period) = msg.unbond_period {
        config.unbond_period = unbond_period;
        resp = resp.add_attribute("unbond_period", unbond_period.to_string());
    }
    if let Some(pool_id) = msg.pool_id {
        config.pool_id = pool_id;
        resp = resp.add_attribute("pool_id", pool_id.to_string());
    }
    if let Some(ica_channel_id) = msg.ica_channel_id {
        let info: ChannelInfo = CHANNEL_INFO.load(deps.storage, ica_channel_id.as_str())?;
        config.ica_account = info.address.to_string();
        config.ica_channel_id = info.id;
        config.ica_connection_id = info.connection_id.to_string();
        resp = resp
            .add_attribute("ica_account", info.address.to_string())
            .add_attribute("ica_channel_id", ica_channel_id.to_string())
            .add_attribute("ica_connection_id", info.connection_id.to_string());
    }
    if let Some(phase) = msg.phase {
        config.phase = phase.to_owned();
        resp = resp.add_attribute("phase", phase.to_string());
    }
    if let Some(phase_step) = msg.phase_step {
        config.phase_step = phase_step.to_owned();
        resp = resp.add_attribute("phase_step", phase_step.to_string());
    }
    if let Some(transfer_timeout) = msg.transfer_timeout {
        config.transfer_timeout = transfer_timeout.to_owned();
        resp = resp.add_attribute("transfer_timeout", transfer_timeout.to_string());
    }
    if let Some(lp_denom) = msg.lp_denom {
        config.lp_denom = lp_denom.to_owned();
        resp = resp.add_attribute("lp_denom", lp_denom.to_string());
    }
    if let Some(transfer_channel_id) = msg.transfer_channel_id {
        config.transfer_channel_id = transfer_channel_id.to_owned();
        resp = resp.add_attribute("transfer_channel_id", transfer_channel_id.to_string());
    }
    if let Some(quote_denom) = msg.quote_denom {
        config.quote_denom = quote_denom.to_owned();
        resp = resp.add_attribute("quote_denom", quote_denom.to_string());
    }
    if let Some(base_denom) = msg.base_denom {
        config.base_denom = base_denom.to_owned();
        resp = resp.add_attribute("base_denom", base_denom.to_string());
    }
    if let Some(chain_id) = msg.chain_id {
        config.chain_id = chain_id.to_owned();
        resp = resp.add_attribute("chain_id", chain_id.to_string());
    }
    if let Some(deposit_denom) = msg.controller_deposit_denom {
        config.controller_deposit_denom = deposit_denom.to_owned();
        resp = resp.add_attribute("deposit_denom", config.controller_deposit_denom.to_string());
    }
    if let Some(transfer_channel_id) = msg.controller_transfer_channel_id {
        config.controller_transfer_channel_id = transfer_channel_id.to_owned();
        resp = resp.add_attribute("transfer_channel_id", transfer_channel_id.to_string());
    }

    if let Some(superfluid_validator) = msg.superfluid_validator {
        if state.bonded_lp_amount.is_zero()
            || !config.automate_superfluid
            || config.superfluid_validator == ""
        {
            config.superfluid_validator = superfluid_validator.to_owned();
            resp = resp.add_attribute("superfluid_validator", superfluid_validator.to_string());
        }
    }

    if let Some(automate_superfluid) = msg.automate_superfluid {
        config.automate_superfluid = automate_superfluid.to_owned();
        resp = resp.add_attribute("automate_superfluid", "true");
    }

    if let Some(extern_tokens) = msg.extern_tokens {
        config.extern_tokens = extern_tokens.to_owned();
        resp = resp.add_attribute("extern_tokens", "true");

        state.extern_token_amounts = vec![Uint128::from(0u128); extern_tokens.len()];
    }

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;
    Ok(resp)
}
