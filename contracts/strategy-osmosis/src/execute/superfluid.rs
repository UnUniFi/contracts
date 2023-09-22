use crate::error::ContractError;
use crate::helpers::superfluid_delegate_msg_to_any;
use crate::state::{PARAMS, STATE};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::types::osmosis::superfluid::MsgSuperfluidDelegate;
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_superfluid_delegate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let params = PARAMS.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    // Permission check
    if info.sender != params.authority {
        return Err(ContractError::Unauthorized {});
    }

    let msg = MsgSuperfluidDelegate {
        sender: params.ica_account.to_string(),
        lock_id: state.lock_id,
        val_addr: params.superfluid_validator,
    };
    if let Ok(msg_any) = superfluid_delegate_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            params.ica_channel_id,
            params.transfer_timeout,
            "superfluid_delegate".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "failure in conversion from proto to any: MsgSuperfluidDelegate".to_string(),
    }))
}
