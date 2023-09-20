use crate::error::ContractError;
use crate::helpers::swap_msg_to_any;
use crate::state::{DepositToken, CONFIG, STATE};
use cosmwasm_std::{Env, Response, Storage};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountIn;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use prost_types::Any;
use ununifi_binding::v0::binding::UnunifiMsg;

use super::helpers::determine_ica_amounts;

pub fn get_extern_token_sell_messages(store: &mut dyn Storage) -> Result<Vec<Any>, ContractError> {
    let config = CONFIG.load(store)?;
    let state = STATE.load(store)?;
    let mut msgs: Vec<Any> = vec![];
    for (i, extern_token) in config.extern_tokens.iter().enumerate() {
        if state.extern_token_amounts[i].is_zero() {
            continue;
        }
        let msg = MsgSwapExactAmountIn {
            sender: config.ica_account.to_string(),
            token_in: Some(OsmosisCoin {
                denom: extern_token.extern_token.to_owned(),
                amount: state.extern_token_amounts[i].to_string(),
            }),
            token_out_min_amount: "1".to_string(),
            routes: extern_token.swap_in_route.to_owned(),
        };
        if let Ok(msg_any) = swap_msg_to_any(msg) {
            msgs.push(msg_any);
        }
    }
    return Ok(msgs);
}

pub fn get_swap_to_deposit_token_messages(
    store: &mut dyn Storage,
) -> Result<Vec<Any>, ContractError> {
    let config = CONFIG.load(store)?;
    let state = STATE.load(store)?;
    let ica_amounts: crate::state::IcaAmounts =
        determine_ica_amounts(config.to_owned(), state.to_owned());
    let mut in_denom = config.quote_denom.to_string();
    let mut out_denom = config.base_denom.to_string();
    if config.deposit_token == DepositToken::Quote {
        in_denom = config.base_denom.to_string();
        out_denom = config.quote_denom.to_string();
    }
    let to_swap_amount = ica_amounts.to_swap_amount;
    let mut msgs: Vec<Any> = vec![];
    if !to_swap_amount.is_zero() {
        let msg = MsgSwapExactAmountIn {
            sender: config.ica_account.to_string(),
            token_in: Some(OsmosisCoin {
                denom: in_denom,
                amount: to_swap_amount.to_string(),
            }),
            token_out_min_amount: "1".to_string(),
            routes: vec![SwapAmountInRoute {
                pool_id: config.pool_id,
                token_out_denom: out_denom,
            }],
        };
        if let Ok(msg_any) = swap_msg_to_any(msg) {
            msgs.push(msg_any);
        }
    }

    return Ok(msgs);
}

// withdraw phase function
pub fn execute_swap_to_deposit_token(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(store)?;
    let msgs = get_swap_to_deposit_token_messages(store)?;
    if msgs.len() > 0 {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "swap_to_deposit_token".to_string(),
            msgs,
        )?);
    }
    Ok(Response::new())
}

// deposit phase function
pub fn execute_ica_sell_extern_tokens(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = CONFIG.load(store)?;
    let msgs = get_extern_token_sell_messages(store)?;
    if msgs.len() > 0 {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "sell_extern_tokens".to_string(),
            msgs,
        )?);
    }
    Ok(Response::new())
}
