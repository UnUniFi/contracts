use crate::error::ContractError;
use crate::state::{CONFIG, STATE};
use cosmwasm_std::StdError;
use cosmwasm_std::{coin, Env, IbcTimeout, Response, Storage, Uint128};
use ica_tx::helpers::send_ica_tx;
use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
use proto::ibc::applications::transfer::v1::MsgTransfer;
use proto::traits::MessageExt;
use ununifi_binding::v0::binding::UnunifiMsg;

use super::helpers::determine_ica_amounts;

pub fn execute_ibc_transfer_to_host(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(store)?;
    let mut state = STATE.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned(), state.to_owned());
    let to_transfer_to_host = ica_amounts.to_transfer_to_host;
    if to_transfer_to_host.is_zero() {
        return Ok(Response::new());
    }
    let timeout = env.block.time.plus_seconds(config.transfer_timeout);
    let ibc_msg = UnunifiMsg::IbcTransfer {
        channel_id: config.controller_transfer_channel_id,
        to_address: config.ica_account,
        amount: coin(to_transfer_to_host.u128(), config.controller_deposit_denom),
        timeout: IbcTimeout::from(timeout),
    };

    state.controller_stacked_amount_to_deposit = Uint128::from(0u128);
    state.controller_pending_transfer_amount += to_transfer_to_host;
    STATE.save(store, &state)?;

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", "ibc_transfer_to_host");
    Ok(res)
}

pub fn execute_ibc_transfer_to_controller(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(store)?;
    let state = STATE.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned(), state.to_owned());
    let to_transfer_to_controller = ica_amounts.to_transfer_to_controller;
    if to_transfer_to_controller.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: config.transfer_channel_id,
        token: Some(ProtoCoin {
            denom: config.base_denom,
            amount: to_transfer_to_controller.to_string(),
        }),
        sender: config.ica_account,
        receiver: env.contract.address.to_string(),
        timeout_height: None,
        timeout_timestamp: env.block.time.nanos() + config.transfer_timeout * 1000_000_000,
    };
    if let Ok(msg_any) = msg.to_any() {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "transfer_to_controller".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}
