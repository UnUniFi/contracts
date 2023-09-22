use crate::error::ContractError;
use crate::helpers::superfluid_delegate_msg_to_any;
use crate::state::{CONFIG, STATE};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::types::osmosis::superfluid::MsgSuperfluidDelegate;
use ununifi_binding::v0::binding::UnunifiMsg;

pub fn execute_superfluid_delegate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    let msg = MsgSuperfluidDelegate {
        sender: config.ica_account.to_string(),
        lock_id: state.lock_id,
        val_addr: config.superfluid_validator,
    };
    if let Ok(msg_any) = superfluid_delegate_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "superfluid_delegate".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "failure in conversion from proto to any: MsgSuperfluidDelegate".to_string(),
    }))
}
