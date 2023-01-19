use std::convert::TryInto;
use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, coin, has_coins, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    IbcMsg, IbcQuery, MessageInfo, Order, PortIdResponse, Reply, Response, StdResult, Storage,
    SubMsg, SubMsgResponse, SubMsgResult, Uint128, Uint64, WasmMsg,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgExitSwapShareAmountInResponse, MsgSwapExactAmountInResponse,
};

// use crate::contract::SWAP_REPLY_ID;
use crate::contract::{EXIT_SWAP_REPLY_ID, JOIN_SWAP_REPLY_ID};
use crate::error::ContractError;
use crate::helpers::{generate_exit_swap_share_amount_in, generate_join_swap_extern_msg};
use crate::state::{
    ExitSwapMsgReplyState, SwapJoinMsgReplyState, DEPOSITOR_SHARE, EXIT_SWAP_REPLY_STATES,
    SWAP_JOIN_REPLY_STATES,
};

pub fn execute_join_swap_extern(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pool_id: u64,
    token_in: Coin,
    share_out_min_amount: String,
    // slipage: Slipage,
) -> Result<Response, ContractError> {
    // check if user send enough fund in a tx to swap and join in a pool
    if !has_coins(&info.funds, &token_in) {
        return Err(ContractError::InsufficientFunds { coins: info.funds });
    }

    // generate the join_swap_extern_amount_in_msg
    let join_swap_extern_amount_in_msg = generate_join_swap_extern_msg(
        env.contract.address,
        pool_id,
        token_in,
        share_out_min_amount,
    )?;

    // record original sender in the state for the information of the later recording
    // of share amount for sender
    SWAP_JOIN_REPLY_STATES.save(
        deps.storage,
        JOIN_SWAP_REPLY_ID,
        &SwapJoinMsgReplyState {
            original_sender: info.sender,
        },
    )?;

    // TODO: Should we handle the error here?
    Ok(Response::new()
        .add_attribute("action", "trade and join in a pool")
        .add_submessage(SubMsg::reply_on_success(
            join_swap_extern_amount_in_msg,
            JOIN_SWAP_REPLY_ID,
        )))
}

pub fn execute_exit_swap_share(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pool_id: u64,
    token_out_denom: String,
    share_in_amount: String,
    token_out_min_amount: String,
) -> Result<Response, ContractError> {
    let exit_swap_share_amount_in_msg = generate_exit_swap_share_amount_in(
        env.contract.address,
        pool_id,
        token_out_denom.clone(),
        share_in_amount.clone(),
        token_out_min_amount,
    )?;

    // record original sender in the state for the information of the later recording
    // of share amount for sender
    EXIT_SWAP_REPLY_STATES.save(
        deps.storage,
        EXIT_SWAP_REPLY_ID,
        &ExitSwapMsgReplyState {
            original_sender: info.sender.clone(),
            token_out_denom: token_out_denom,
        },
    )?;

    DEPOSITOR_SHARE.update(deps.storage, &info.sender, |share| -> StdResult<_> {
        let share = share.unwrap_or_default();

        Ok(share - Uint128::from_str(&share_in_amount).unwrap())
    })?;

    Ok(Response::new()
        .add_attribute("action", "exit_swap_share_amount_in")
        .add_submessage(SubMsg::reply_on_success(
            exit_swap_share_amount_in_msg,
            EXIT_SWAP_REPLY_ID,
        )))
}

pub fn handle_join_swap_reply(
    deps: DepsMut,
    msg: Reply,
    swap_join_msg_reply_state: SwapJoinMsgReplyState,
) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        let res: MsgSwapExactAmountInResponse = b.try_into().map_err(ContractError::Std)?;

        // TODO: better form
        // record share out amount with the original sender for the proper retrieve
        let added_share: Uint128 = res.token_out_amount.parse().unwrap();
        DEPOSITOR_SHARE.update(
            deps.storage,
            &swap_join_msg_reply_state.original_sender,
            |share| -> StdResult<_> {
                let share = share.unwrap_or_default();

                Ok(share + added_share)
            },
        )?;

        return Ok(Response::new()
            .add_attribute(
                "original_sender",
                &swap_join_msg_reply_state.original_sender,
            )
            .add_attribute("share_out_amount", res.token_out_amount));
    }

    Err(ContractError::FailedSwapJoin {
        reason: msg.result.unwrap_err(),
    })
}

pub fn handle_exit_swap_reply(
    _deps: DepsMut,
    msg: Reply,
    exit_swap_msg_reply_state: ExitSwapMsgReplyState,
) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        let res: MsgExitSwapShareAmountInResponse = b.try_into().map_err(ContractError::Std)?;

        let token_out = vec![Coin {
            denom: exit_swap_msg_reply_state.token_out_denom,
            amount: Uint128::from_str(&res.token_out_amount).unwrap(),
        }];

        let return_deposit_msg: CosmosMsg = BankMsg::Send {
            to_address: exit_swap_msg_reply_state.original_sender.to_string(),
            amount: token_out.clone(),
        }
        .into();

        return Ok(Response::new()
            .add_attribute(
                "original_sender",
                &exit_swap_msg_reply_state.original_sender,
            )
            .add_attribute("token_out_amount", res.token_out_amount)
            .add_message(return_deposit_msg));
    }

    Err(ContractError::FailedExitSwap {
        reason: msg.result.unwrap_err(),
    })
}
