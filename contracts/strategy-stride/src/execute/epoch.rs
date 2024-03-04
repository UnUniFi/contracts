use crate::error::ContractError;
use cosmwasm_std::{DepsMut, Env, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_epoch(
    _deps: DepsMut,
    _env: Env,
    _success: bool,
    _ret: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    // TODO: write handler to request Icq
    Ok(Response::new())
}
