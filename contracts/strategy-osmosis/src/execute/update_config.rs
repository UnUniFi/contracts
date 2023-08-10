use crate::error::ContractError;
use crate::msgs::{ChannelInfo, UpdateConfigMsg};

use crate::state::{Config, CHANNEL_INFO, CONFIG};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use ununifi_binding::v0::binding::UnunifiMsg;

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateConfigMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = msg.owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unbond_period) = msg.unbond_period {
        config.unbond_period = unbond_period;
    }
    if let Some(pool_id) = msg.pool_id {
        config.host_config.pool_id = pool_id;
    }
    if let Some(ica_channel_id) = msg.ica_channel_id {
        let info: ChannelInfo = CHANNEL_INFO.load(deps.storage, ica_channel_id.as_str())?;
        config.ica_account = info.address.to_string();
        config.ica_channel_id = info.id;
        config.ica_connection_id = info.connection_id.to_string();
    }
    if let Some(phase) = msg.phase {
        config.phase = phase;
    }
    if let Some(phase_step) = msg.phase_step {
        config.phase_step = phase_step;
    }
    if let Some(transfer_timeout) = msg.transfer_timeout {
        config.transfer_timeout = transfer_timeout;
    }
    if let Some(lp_denom) = msg.lp_denom {
        config.host_config.lp_denom = lp_denom;
    }
    if let Some(transfer_channel_id) = msg.transfer_channel_id {
        config.host_config.transfer_channel_id = transfer_channel_id;
    }
    if let Some(quote_denom) = msg.quote_denom {
        config.host_config.quote_denom = quote_denom;
    }
    if let Some(base_denom) = msg.base_denom {
        config.host_config.base_denom = base_denom;
    }
    if let Some(chain_id) = msg.chain_id {
        config.host_config.chain_id = chain_id;
    }
    if let Some(deposit_denom) = msg.controller_deposit_denom {
        config.controller_config.deposit_denom = deposit_denom;
    }
    if let Some(transfer_channel_id) = msg.controller_transfer_channel_id {
        config.controller_config.transfer_channel_id = transfer_channel_id;
    }

    CONFIG.save(deps.storage, &config)?;
    let resp = Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("owner", config.owner.to_string())
        .add_attribute("unbond_period", config.unbond_period.to_string())
        .add_attribute(
            "lp_redemption_rate",
            config.host_config.lp_redemption_rate.to_string(),
        )
        .add_attribute(
            "deposit_denom",
            config.controller_config.deposit_denom.to_string(),
        );

    Ok(resp)
}
