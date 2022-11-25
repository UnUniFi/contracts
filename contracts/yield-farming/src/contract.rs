use crate::state::{
    increase_channel_balance, join_ibc_paths, Config, LockInfo, UnlockInfo, CHANNEL_INFO,
    CHANNEL_STATE, CONFIG, LOCKUP, REWARD_POOLS, TEMP_SENDER, TOTAL_DEPOSITS, USER_DEPOSITS,
    USER_LOCKS, USER_PENDING_REWARDS, USER_REWARD_DEBTS, USER_UNLOCKS,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, coin, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg,
    IbcQuery, MessageInfo, Order, PortIdResponse, Response, StdResult, Storage, Uint128, WasmMsg,
};

use cw_utils::{nonpayable, one_coin};
use yield_farming::{
    amount::Amount,
    error::ContractError,
    farming::{
        ChannelResponse, ClaimAllTokensMsg, ClaimTokensMsg, ConfigResponse, CreateLockupMsg,
        ExecuteMsg, ExitPoolMsg, InstantiateMsg, JoinPoolMsg, ListChannelsResponse, LockTokensMsg,
        LockupResponse, MigrateMsg, QueryMsg, SwapMsg, TransferMsg, UnlockTokensMsg,
    },
    ibc::{
        ClaimPacket, ExitPoolPacket, Ics20Packet, JoinPoolPacket, LockPacket, OsmoPacket,
        SwapAmountInRoute, SwapPacket, UnlockPacket,
    },
};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender,
        unlock_period: msg.unlock_period,
        is_freeze: false,
        default_timeout: msg.default_timeout,
        init_channel: false,
        default_remote_denom: None,
    };
    CONFIG.save(deps.storage, &config)?;
    TOTAL_DEPOSITS.save(deps.storage, &Uint128::zero())?;
    REWARD_POOLS.save(deps.storage, &vec![])?;

    Ok(Response::new())
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            unlock_period,
        } => execute_update_config(deps, env, info, owner, unlock_period),
        ExecuteMsg::UpdateFreezeFlag { freeze_flag } => {
            execute_update_freeze_flag(deps, env, info, freeze_flag)
        }
        ExecuteMsg::Swap(msg) => {
            let coin = one_coin(&info)?;
            execute_swap(deps, env, msg, Amount::Native(coin), info.sender)
        }
        ExecuteMsg::JoinPool(pool) => {
            let coin = one_coin(&info)?;
            execute_join_pool(deps, env, pool, Amount::Native(coin), info.sender)
        }
        ExecuteMsg::ExitPool(pool) => {
            let coin = one_coin(&info)?;
            execute_exit_pool(deps, env, pool, Amount::Native(coin), info.sender)
        }
        ExecuteMsg::CreateLockup(msg) => {
            nonpayable(&info)?;
            execute_create_lockup(deps, env, msg)
        }
        ExecuteMsg::LockTokens(msg) => {
            let coin = one_coin(&info)?;
            execute_lock_tokens(deps, env, msg, Amount::Native(coin), info.sender)
        }
        ExecuteMsg::ClaimReward(msg) => execute_claim_reward(deps, env, info, msg),
        ExecuteMsg::ClaimAllRewards(msg) => execute_claim_all_rewards(deps, env, info, msg),
        ExecuteMsg::StartUnlockTokens(msg) => execute_start_unlock_tokens(deps, env, info, msg),
        ExecuteMsg::ClaimUnlockedTokens {} => execute_claim_unlocked_tokens(deps, env, info),
    }
}

/// Only owner can execute it.
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    unlock_period: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unlock_period) = unlock_period {
        config.unlock_period = unlock_period;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

/// Only owner can execute it.
pub fn execute_update_freeze_flag(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    freeze_flag: bool,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.is_freeze = freeze_flag;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_freeze_flag"))
}

pub fn execute_swap(
    deps: DepsMut,
    env: Env,
    msg: SwapMsg,
    amount: Amount,
    sender: Addr,
) -> Result<Response, ContractError> {
    let swap_packet = SwapPacket {
        routes: vec![SwapAmountInRoute {
            pool_id: msg.pool,
            token_out_denom: msg.token_out,
        }],
        token_out_min_amount: msg.min_amount_out,
    };
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };

    execute_transfer_with_action(
        deps,
        env,
        transfer_msg,
        amount,
        sender,
        Some(OsmoPacket::Swap(swap_packet)),
        "swap",
    )
}

pub fn execute_join_pool(
    deps: DepsMut,
    env: Env,
    msg: JoinPoolMsg,
    amount: Amount,
    sender: Addr,
) -> Result<Response, ContractError> {
    let gamm_packet = JoinPoolPacket {
        pool_id: msg.pool,
        share_out_min_amount: msg.share_min_out,
    };
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };

    execute_transfer_with_action(
        deps,
        env,
        transfer_msg,
        amount,
        sender,
        Some(OsmoPacket::JoinPool(gamm_packet)),
        "join_pool",
    )
}

pub fn execute_exit_pool(
    deps: DepsMut,
    env: Env,
    msg: ExitPoolMsg,
    amount: Amount,
    sender: Addr,
) -> Result<Response, ContractError> {
    let gamm_packet = ExitPoolPacket {
        token_out_denom: msg.token_out,
        token_out_min_amount: msg.min_amount_out,
    };
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };

    execute_transfer_with_action(
        deps,
        env,
        transfer_msg,
        amount,
        sender,
        Some(OsmoPacket::ExitPool(gamm_packet)),
        "exit_pool",
    )
}

pub fn execute_create_lockup(
    deps: DepsMut,
    env: Env,
    msg: CreateLockupMsg,
) -> Result<Response, ContractError> {
    let lockup_key = (msg.channel.as_str(), env.contract.address.as_str());
    if LOCKUP.has(deps.storage, lockup_key) {
        return Err(ContractError::LockupAccountFound {});
    }

    let gamm_packet = OsmoPacket::LockupAccount {};
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };
    let port_id = get_ibc_port_id(deps.as_ref())?;

    execute_only_action(
        deps.storage,
        env.clone(),
        transfer_msg,
        env.contract.address,
        gamm_packet,
        "create_lockup",
        port_id,
    )
}

pub fn execute_lock_tokens(
    deps: DepsMut,
    env: Env,
    msg: LockTokensMsg,
    amount: Amount,
    sender: Addr,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.is_freeze {
        return Err(ContractError::ContractFreezed {});
    }

    assert_lockup_owner(deps.storage, msg.channel.as_str(), sender.as_str())?;

    let port_id = get_ibc_port_id(deps.as_ref())?;

    claim_tokens(
        deps.storage,
        env.clone(),
        ClaimTokensMsg {
            channel: msg.channel.clone(),
            timeout: msg.timeout,
            denom: amount.denom(),
        },
        env.contract.address.clone(),
        port_id,
    )?;
    withdraw_reward(deps.storage, sender.to_string())?;

    if let Some(mut user_deposits) = USER_DEPOSITS.may_load(deps.storage, sender.to_string())? {
        user_deposits = user_deposits.checked_add(amount.amount()).unwrap();
        USER_DEPOSITS.save(deps.storage, sender.to_string(), &user_deposits)?;
    } else {
        USER_DEPOSITS.save(deps.storage, sender.to_string(), &amount.amount())?;
    }
    let mut total_deposits = TOTAL_DEPOSITS.load(deps.storage)?;
    total_deposits = total_deposits.checked_add(amount.amount()).unwrap();
    TOTAL_DEPOSITS.save(deps.storage, &total_deposits)?;

    update_user_reward_debt(deps.storage, sender.to_string())?;

    let gamm_packet = OsmoPacket::Lock(LockPacket {
        duration: msg.duration,
    });
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };

    TEMP_SENDER.save(deps.storage, &sender.to_string())?;

    execute_transfer_with_action(
        deps,
        env.clone(),
        transfer_msg,
        amount,
        env.contract.address,
        Some(gamm_packet),
        "lock_tokens",
    )
}

pub fn claim_tokens(
    store: &mut dyn Storage,
    env: Env,
    msg: ClaimTokensMsg,
    sender: Addr,
    port_id: String,
) -> Result<Response, ContractError> {
    assert_lockup_owner(store, msg.channel.as_str(), sender.as_str())?;

    let gamm_packet = OsmoPacket::Claim(ClaimPacket { denom: msg.denom });
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };

    execute_only_action(
        store,
        env,
        transfer_msg,
        sender,
        gamm_packet,
        "claim_tokens",
        port_id,
    )
}

pub fn unlock_tokens(
    store: &mut dyn Storage,
    env: Env,
    msg: UnlockTokensMsg,
    sender: Addr,
    port_id: String,
) -> Result<Response, ContractError> {
    assert_lockup_owner(store, msg.channel.as_str(), sender.as_str())?;

    if let Some(mut user_unlocks) = USER_UNLOCKS.may_load(store, sender.to_string())? {
        user_unlocks.push(UnlockInfo {
            lock_id: msg.lock_id.u64(),
            start_time: env.block.time.seconds(),
        });
        USER_UNLOCKS.save(store, sender.to_string(), &user_unlocks)?;
    } else {
        USER_UNLOCKS.save(
            store,
            sender.to_string(),
            &vec![UnlockInfo {
                lock_id: msg.lock_id.u64(),
                start_time: env.block.time.seconds(),
            }],
        )?;
    }

    let gamm_packet = OsmoPacket::Unlock(UnlockPacket { id: msg.lock_id });
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };

    execute_only_action(
        store,
        env,
        transfer_msg,
        sender,
        gamm_packet,
        "begin_unlock_tokens",
        port_id,
    )
}

pub fn assert_lockup_owner(
    store: &mut dyn Storage,
    channel: &str,
    owner: &str,
) -> Result<(), ContractError> {
    let lockup_key = (channel, owner);
    if !LOCKUP.has(store, lockup_key) {
        return Err(ContractError::NoLockupAccount {});
    }

    Ok(())
}

pub fn get_ibc_port_id(deps: Deps) -> StdResult<String> {
    let query = IbcQuery::PortId {}.into();
    let PortIdResponse { port_id } = deps.querier.query(&query)?;

    Ok(port_id)
}

pub fn get_ibc_full_denom(port_id: String, channel: &str, denom: &str) -> StdResult<String> {
    let ibc_prefix = join_ibc_paths(port_id.as_str(), channel);

    Ok(join_ibc_paths(ibc_prefix.as_str(), denom))
}

pub fn execute_only_action(
    store: &mut dyn Storage,
    env: Env,
    msg: TransferMsg,
    sender: Addr,
    action: OsmoPacket,
    action_label: &str,
    port_id: String,
) -> Result<Response, ContractError> {
    // ensure the requested channel is registered
    if !CHANNEL_INFO.has(store, &msg.channel) {
        return Err(ContractError::NoSuchChannel { id: msg.channel });
    }

    let config = CONFIG.load(store)?;
    if config.default_remote_denom.is_none() {
        return Err(ContractError::NoSuchChannel { id: msg.channel });
    }

    // delta from user is in seconds
    let timeout_delta = match msg.timeout {
        Some(t) => t,
        None => config.default_timeout,
    };
    // timeout is in nanoseconds
    let timeout = env.block.time.plus_seconds(timeout_delta);

    let denom = get_ibc_full_denom(
        port_id,
        msg.channel.as_str(),
        config.default_remote_denom.unwrap().as_str(),
    )?;

    // build ics20 packet
    let packet = Ics20Packet::new(
        0u8.into(),
        denom,
        sender.as_ref(),
        &msg.remote_address,
        Some(action),
    );

    // prepare ibc message
    let msg: CosmosMsg = IbcMsg::SendPacket {
        channel_id: msg.channel,
        data: to_binary(&packet)?,
        timeout: timeout.into(),
    }
    .into();

    // send response
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", action_label)
        .add_attribute("sender", &packet.sender);

    Ok(res)
}

pub fn execute_transfer_with_action(
    deps: DepsMut,
    env: Env,
    msg: TransferMsg,
    amount: Amount,
    sender: Addr,
    action: Option<OsmoPacket>,
    action_label: &str,
) -> Result<Response, ContractError> {
    if amount.is_empty() {
        return Err(ContractError::NoFunds {});
    }

    // ensure the requested channel is registered
    if !CHANNEL_INFO.has(deps.storage, &msg.channel) {
        return Err(ContractError::NoSuchChannel { id: msg.channel });
    }

    let denom = amount.denom();
    let our_chain = true;

    // delta from user is in seconds
    let timeout_delta = match msg.timeout {
        Some(t) => t,
        None => CONFIG.load(deps.storage)?.default_timeout,
    };
    // timeout is in nanoseconds
    let timeout = env.block.time.plus_seconds(timeout_delta);

    // build ics20 packet
    let packet = Ics20Packet::new(
        amount.amount(),
        denom,
        sender.as_ref(),
        &msg.remote_address,
        action,
    );

    if our_chain {
        increase_channel_balance(deps.storage, &msg.channel, &amount.denom(), amount.amount())?;
    }

    // prepare ibc message
    let msg = IbcMsg::SendPacket {
        channel_id: msg.channel,
        data: to_binary(&packet)?,
        timeout: timeout.into(),
    }
    .into();
    let msgs: Vec<CosmosMsg> = vec![msg];
    let mut attributes = vec![
        attr("action", action_label),
        attr("sender", &packet.sender),
        attr("denom", &packet.denom),
        attr("amount", &packet.amount.to_string()),
    ];
    if !packet.receiver.is_empty() {
        attributes.push(attr("receiver", &packet.receiver));
    }

    // send response
    let res = Response::new()
        .add_messages(msgs)
        .add_attributes(attributes);

    Ok(res)
}

pub fn execute_claim_reward(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ClaimTokensMsg,
) -> Result<Response, ContractError> {
    let port_id = get_ibc_port_id(deps.as_ref())?;
    claim_tokens(
        deps.storage,
        env.clone(),
        ClaimTokensMsg {
            channel: msg.channel,
            timeout: msg.timeout,
            denom: msg.denom,
        },
        env.contract.address,
        port_id,
    )?;
    withdraw_reward(deps.storage, info.sender.to_string())?;

    let mut total_coins: Vec<Coin> = vec![];
    let reward_pools = REWARD_POOLS.load(deps.storage)?;
    for reward_pool in reward_pools {
        let key = reward_pool.reward_token.to_string() + &info.sender.to_string();
        if let Some(claim_amount) = USER_PENDING_REWARDS.may_load(deps.storage, key.clone())? {
            if !claim_amount.is_zero() {
                total_coins.push(Coin {
                    denom: reward_pool.reward_token,
                    amount: claim_amount,
                });
                USER_PENDING_REWARDS.save(deps.storage, key, &Uint128::zero())?;
            }
        }
    }

    update_user_reward_debt(deps.storage, info.sender.to_string())?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: total_coins,
        }))
        .add_attribute("action", "execute_claim_reward"))
}

pub fn execute_claim_all_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ClaimAllTokensMsg,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let mut denoms: Vec<String> = vec![];
    if let Some(user_locks) = USER_LOCKS.may_load(deps.storage, info.sender.to_string())? {
        for user_lock in user_locks {
            if !denoms.contains(&user_lock.denom) {
                denoms.push(user_lock.denom);
            }
        }
    }

    for denom in denoms {
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::ClaimReward(ClaimTokensMsg {
                channel: msg.channel.clone(),
                timeout: msg.timeout,
                denom,
            }))?,
            funds: vec![],
        }));
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "execute_claim_all_rewards"))
}

pub fn execute_start_unlock_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: UnlockTokensMsg,
) -> Result<Response, ContractError> {
    let port_id = get_ibc_port_id(deps.as_ref())?;
    claim_tokens(
        deps.storage,
        env.clone(),
        ClaimTokensMsg {
            channel: msg.channel.clone(),
            timeout: msg.timeout.clone(),
            denom: msg.denom.clone(),
        },
        env.contract.address.clone(),
        port_id.clone(),
    )?;
    withdraw_reward(deps.storage, info.sender.to_string())?;

    unlock_tokens(
        deps.storage,
        env.clone(),
        msg,
        env.contract.address,
        port_id,
    )?;

    update_user_reward_debt(deps.storage, info.sender.to_string())?;
    Ok(Response::new().add_attribute("action", "execute_start_unlock_tokens"))
}

pub fn execute_claim_unlocked_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut messages: Vec<CosmosMsg> = vec![];

    if let Some(user_unlocks) = USER_UNLOCKS.may_load(deps.storage, info.sender.to_string())? {
        let user_locks = USER_LOCKS
            .may_load(deps.storage, info.sender.to_string())?
            .unwrap();
        let mut new_user_unlocks = vec![];
        for item in user_unlocks {
            if item.start_time + config.unlock_period < env.block.time.seconds() {
                let mut new_user_locks = user_locks.clone();
                let lock_info_index = user_locks
                    .iter()
                    .position(|user_lock| user_lock.lock_id == item.lock_id)
                    .unwrap();
                let lock_info = user_locks.get(lock_info_index).unwrap();
                new_user_locks.remove(lock_info_index);
                USER_LOCKS.save(deps.storage, info.sender.to_string(), &new_user_locks)?;

                let mut user_deposit = USER_DEPOSITS
                    .may_load(deps.storage, info.sender.to_string())?
                    .unwrap();
                user_deposit = user_deposit.checked_sub(lock_info.amount).unwrap();
                USER_DEPOSITS.save(deps.storage, info.sender.to_string(), &user_deposit)?;

                messages.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: vec![coin(
                        lock_info.clone().amount.u128(),
                        lock_info.clone().denom,
                    )],
                }));
            } else {
                new_user_unlocks.push(item);
            }
        }
        USER_UNLOCKS.save(deps.storage, info.sender.to_string(), &new_user_unlocks)?;
    } else {
        return Err(ContractError::NoUnlockedTokens {});
    }
    Ok(Response::new().add_attribute("action", "execute_claim_unlocked_tokens"))
}

pub fn withdraw_reward(store: &mut dyn Storage, wallet: String) -> StdResult<()> {
    let reward_pools = REWARD_POOLS.load(store)?;

    for reward_pool in reward_pools {
        if let Some(user_deposit) = USER_DEPOSITS.may_load(store, wallet.clone())? {
            let key = reward_pool.reward_token.to_string() + &wallet;
            if let Some(user_reward_debt) = USER_REWARD_DEBTS.may_load(store, key.clone())? {
                let pending = user_deposit
                    .checked_mul(reward_pool.acc_reward_per_share)?
                    .checked_div(Uint128::from(1_000_000_000_000u128))?
                    .checked_sub(user_reward_debt)?;
                if !pending.is_zero() {
                    USER_PENDING_REWARDS.save(store, key, &pending)?;
                }
            }
        }
    }

    Ok(())
}

pub fn update_user_reward_debt(store: &mut dyn Storage, wallet: String) -> StdResult<()> {
    let reward_pools = REWARD_POOLS.load(store)?;

    for reward_pool in reward_pools {
        if let Some(user_deposit) = USER_DEPOSITS.may_load(store, wallet.clone())? {
            let key = reward_pool.reward_token.to_string() + &wallet;
            let user_reward_debt = user_deposit
                .checked_mul(reward_pool.acc_reward_per_share)?
                .checked_div(Uint128::from(1_000_000_000_000u128))?;
            USER_REWARD_DEBTS.save(store, key, &user_reward_debt)?;
        }
    }

    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::ListChannels {} => to_binary(&query_list(deps)?),
        QueryMsg::Channel { id } => to_binary(&query_channel(deps, id)?),
        QueryMsg::Lockup { channel, owner } => to_binary(&query_lockup(deps, channel, owner)?),
        QueryMsg::UserLocks { owner } => to_binary(&query_user_locks(deps, owner)?),
        QueryMsg::UserUnlocks { owner } => to_binary(&query_user_unlocks(deps, owner)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner.to_string(),
        unlock_period: config.unlock_period,
        is_freeze: config.is_freeze,
        default_timeout: config.default_timeout,
    })
}

fn query_list(deps: Deps) -> StdResult<ListChannelsResponse> {
    let channels = CHANNEL_INFO
        .range_raw(deps.storage, None, None, Order::Ascending)
        .map(|r| r.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;
    Ok(ListChannelsResponse { channels })
}

// make public for ibc tests
pub fn query_channel(deps: Deps, id: String) -> StdResult<ChannelResponse> {
    let info = CHANNEL_INFO.load(deps.storage, &id)?;
    // this returns Vec<(outstanding, total)>
    let state = CHANNEL_STATE
        .prefix(&id)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|r| {
            r.map(|(denom, v)| {
                let outstanding = Amount::from_parts(denom.clone(), v.outstanding);
                let total = Amount::from_parts(denom, v.total_sent);
                (outstanding, total)
            })
        })
        .collect::<StdResult<Vec<_>>>()?;
    // we want (Vec<outstanding>, Vec<total>)
    let (balances, total_sent) = state.into_iter().unzip();

    Ok(ChannelResponse {
        info,
        balances,
        total_sent,
    })
}

fn query_lockup(deps: Deps, channel_id: String, owner: String) -> StdResult<LockupResponse> {
    let lockup_key = (channel_id.as_str(), owner.as_str());
    let lockup_address = LOCKUP.load(deps.storage, lockup_key).unwrap_or_default();
    let res = LockupResponse {
        owner,
        address: lockup_address,
    };
    Ok(res)
}

pub fn query_user_locks(deps: Deps, owner: String) -> StdResult<Vec<LockInfo>> {
    if let Some(user_locks) = USER_LOCKS.may_load(deps.storage, owner)? {
        return Ok(user_locks);
    }
    Ok(vec![])
}

pub fn query_user_unlocks(deps: Deps, owner: String) -> StdResult<Vec<UnlockInfo>> {
    if let Some(user_unlocks) = USER_UNLOCKS.may_load(deps.storage, owner)? {
        return Ok(user_unlocks);
    }
    Ok(vec![])
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::*;

    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn setup_and_query() {
        let deps = setup(&["channel-3"]);

        let raw_list = query(deps.as_ref(), mock_env(), QueryMsg::ListChannels {}).unwrap();
        let list_res: ListChannelsResponse = from_binary(&raw_list).unwrap();
        assert_eq!(1, list_res.channels.len());
        assert_eq!(mock_channel_info("channel-3"), list_res.channels[0]);
        // assert_eq!(mock_channel_info("channel-7"), list_res.channels[1]);

        let raw_channel = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Channel {
                id: "channel-3".to_string(),
            },
        )
        .unwrap();
        let chan_res: ChannelResponse = from_binary(&raw_channel).unwrap();
        assert_eq!(chan_res.info, mock_channel_info("channel-3"));
        assert_eq!(0, chan_res.total_sent.len());
        assert_eq!(0, chan_res.balances.len());

        let err = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Channel {
                id: "channel-10".to_string(),
            },
        )
        .unwrap_err();
        assert_eq!(
            err,
            StdError::not_found("yield_farming::farming::ChannelInfo")
        );
    }

    #[test]
    fn proper_checks_on_execute_native() {
        let send_channel = "channel-5";
        let mut deps = setup(&[send_channel]);

        let mut msg = TransferMsg {
            channel: send_channel.to_string(),
            remote_address: "foreign-address".to_string(),
            timeout: None,
        };

        // works with proper funds
        let info = mock_info("foobar", &coins(1234567, "ucosm"));
        let res = execute_transfer_with_action(
            deps.as_mut(),
            mock_env(),
            msg.clone(),
            Amount::Native(one_coin(&info).unwrap()),
            info.sender,
            None,
            "transfer",
        )
        .unwrap();
        assert_eq!(1, res.messages.len());
        if let CosmosMsg::Ibc(IbcMsg::SendPacket {
            channel_id,
            data,
            timeout,
        }) = &res.messages[0].msg
        {
            let expected_timeout = mock_env().block.time.plus_seconds(DEFAULT_TIMEOUT);
            assert_eq!(timeout, &expected_timeout.into());
            assert_eq!(channel_id.as_str(), send_channel);
            let msg: Ics20Packet = from_binary(data).unwrap();

            assert_eq!(msg.amount, Uint128::new(1234567));
            assert_eq!(msg.denom.as_str(), "ucosm");
            assert_eq!(msg.sender.as_str(), "foobar");
            assert_eq!(msg.receiver.as_str(), "foreign-address");
        } else {
            panic!("Unexpected return message: {:?}", res.messages[0]);
        }

        // reject with bad channel id
        msg.channel = "channel-45".to_string();
        let info = mock_info("foobar", &coins(1234567, "ucosm"));
        let err = execute_transfer_with_action(
            deps.as_mut(),
            mock_env(),
            msg,
            Amount::Native(one_coin(&info).unwrap()),
            info.sender,
            None,
            "transfer",
        )
        .unwrap_err();
        assert_eq!(
            err,
            ContractError::NoSuchChannel {
                id: "channel-45".to_string()
            }
        );
    }
}
