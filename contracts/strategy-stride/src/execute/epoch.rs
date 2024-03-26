use crate::error::ContractError;
use crate::helpers::query_balance;
use crate::state::{PARAMS, STAKE_RATE_MULTIPLIER, STATE};
use cosmwasm_std::{DepsMut, Env, Response};
use ununifi_binding::v1::binding::UnunifiMsg;

pub fn execute_epoch(
    deps: DepsMut,
    env: Env,
    success: bool,
    ret: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    // TODO: write handler to request Icq
    Ok(Response::new())
}
