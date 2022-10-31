use crate::state::{
    increase_channel_balance, join_ibc_paths, Config, RewardPool, CHANNEL_INFO, CHANNEL_STATE,
    CONFIG, LOCKUP, REWARD_POOLS, TOTAL_DEPOSITS, USER_DEPOSITS, USER_PENDING_REWARDS,
    USER_REWARD_DEBTS,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcQuery, MessageInfo,
    Order, PortIdResponse, Response, StdResult, Storage, Uint128, Uint64,
};

use cw_utils::one_coin;
use yield_farming::{
    amount::Amount,
    error::ContractError,
    farming::{
        ChannelResponse, ClaimTokensMsg, ConfigResponse, CreateLockupMsg, ExecuteMsg, ExitPoolMsg,
        InstantiateMsg, JoinPoolMsg, ListChannelsResponse, LockTokensMsg, LockupResponse,
        MigrateMsg, QueryMsg, SwapMsg, TransferMsg, UnlockTokensMsg,
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
        unbond_period: msg.unbond_period,
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
            unbond_period,
        } => execute_update_config(deps, env, info, owner, unbond_period),
        ExecuteMsg::UpdateFreezeFlag { freeze_flag } => {
            execute_update_freeze_flag(deps, env, info, freeze_flag)
        }
        ExecuteMsg::DepositNativeToken {
            channel,
            timeout,
            duration,
        } => execute_deposit_native_token(deps, env, info, channel, timeout, duration),
        ExecuteMsg::ClaimReward { asset } => execute_claim_reward(deps, env, info, asset),
        ExecuteMsg::ClaimAllRewards {} => execute_claim_all_rewards(deps, env, info),
        ExecuteMsg::StartUnbond {} => execute_start_unbond(deps, env, info),
        ExecuteMsg::ClaimUnbond {} => execute_claim_unbond(deps, env, info),
        ExecuteMsg::UpdatePool {} => execute_update_pool(deps, env),
        ExecuteMsg::SwapReward {
            source_token,
            dest_token,
        } => execute_swap_reward(deps, env, info, source_token, dest_token),
        ExecuteMsg::AutoCompoundRewards {} => execute_auto_compound_rewards(deps, env, info),
    }
}

/// Only owner can execute it. To update the owner address
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    unbond_period: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    // Permission check
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
    }
    if let Some(unbond_period) = unbond_period {
        config.unbond_period = unbond_period;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

/// Only owner can execute it. To update the owner address
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

pub fn execute_deposit_native_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    channel: String,
    timeout: Option<u64>,
    duration: Uint64,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.is_freeze {
        return Err(ContractError::ContractFreezed {});
    }

    let coin = one_coin(&info)?;
    let port_id = get_ibc_port_id(deps.as_ref())?;

    // users need to add liquidity before call this function
    lock_tokens(
        deps.storage,
        env.clone(),
        LockTokensMsg {
            channel: channel.clone(),
            timeout,
            duration,
        },
        Amount::Native(coin.clone()),
        info.sender.clone(),
    )?;
    update_pool(
        deps.storage,
        env,
        channel,
        timeout,
        info.sender.clone(),
        coin.denom,
        port_id,
    )?;
    withdraw_reward(deps.storage, info.sender.to_string())?;

    if let Some(mut user_deposits) =
        USER_DEPOSITS.may_load(deps.storage, info.sender.to_string())?
    {
        user_deposits = user_deposits.checked_add(coin.amount).unwrap();
        USER_DEPOSITS.save(deps.storage, info.sender.to_string(), &user_deposits)?;
    } else {
        USER_DEPOSITS.save(deps.storage, info.sender.to_string(), &coin.amount)?;
    }
    let mut total_deposits = TOTAL_DEPOSITS.load(deps.storage)?;
    total_deposits = total_deposits.checked_add(coin.amount).unwrap();
    TOTAL_DEPOSITS.save(deps.storage, &total_deposits)?;

    update_user_reward_debt(deps.storage, info.sender.to_string())?;

    Ok(Response::new().add_attribute("action", "deposit_native_token"))
}

pub fn swap(
    store: &mut dyn Storage,
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

    transfer_with_action(
        store,
        env,
        transfer_msg,
        amount,
        sender,
        Some(OsmoPacket::Swap(swap_packet)),
        "swap",
    )
}

pub fn join_pool(
    store: &mut dyn Storage,
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

    transfer_with_action(
        store,
        env,
        transfer_msg,
        amount,
        sender,
        Some(OsmoPacket::JoinPool(gamm_packet)),
        "join_pool",
    )
}

pub fn exit_pool(
    store: &mut dyn Storage,
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

    transfer_with_action(
        store,
        env,
        transfer_msg,
        amount,
        sender,
        Some(OsmoPacket::ExitPool(gamm_packet)),
        "exit_pool",
    )
}

pub fn create_lockup(
    store: &mut dyn Storage,
    env: Env,
    msg: CreateLockupMsg,
    sender: Addr,
    port_id: String,
) -> Result<Response, ContractError> {
    let lockup_key = (msg.channel.as_str(), sender.as_str());
    if LOCKUP.has(store, lockup_key) {
        return Err(ContractError::LockupAccountFound {});
    }

    let gamm_packet = OsmoPacket::LockupAccount {};
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
        "create_lockup",
        port_id,
    )
}

pub fn lock_tokens(
    store: &mut dyn Storage,
    env: Env,
    msg: LockTokensMsg,
    amount: Amount,
    sender: Addr,
) -> Result<Response, ContractError> {
    assert_lockup_owner(store, msg.channel.as_str(), sender.as_str())?;

    let gamm_packet = OsmoPacket::Lock(LockPacket {
        duration: msg.duration,
    });
    let transfer_msg = TransferMsg {
        channel: msg.channel,
        remote_address: String::new(),
        timeout: msg.timeout,
    };

    transfer_with_action(
        store,
        env,
        transfer_msg,
        amount,
        sender,
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

pub fn execute_unlock_tokens(
    store: &mut dyn Storage,
    env: Env,
    msg: UnlockTokensMsg,
    sender: Addr,
    port_id: String,
) -> Result<Response, ContractError> {
    assert_lockup_owner(store, msg.channel.as_str(), sender.as_str())?;

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

pub fn transfer_with_action(
    store: &mut dyn Storage,
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
    if !CHANNEL_INFO.has(store, &msg.channel) {
        return Err(ContractError::NoSuchChannel { id: msg.channel });
    }

    let denom = amount.denom();
    let our_chain = true;

    // delta from user is in seconds
    let timeout_delta = match msg.timeout {
        Some(t) => t,
        None => CONFIG.load(store)?.default_timeout,
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
        increase_channel_balance(store, &msg.channel, &amount.denom(), amount.amount())?;
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _amount: Amount,
) -> Result<Response, ContractError> {
    // claim_tokens(deps.storage, env, msg, info.sender, port_id)
    Ok(Response::new().add_attribute("action", "execute_claim_reward"))
}

pub fn execute_claim_all_rewards(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "execute_claim_all_rewards"))
}

pub fn execute_start_unbond(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "execute_start_unbond"))
}

pub fn execute_claim_unbond(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "execute_claim_unbond"))
}

pub fn execute_swap_reward(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _source_token: String,
    _dest_token: String,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "execute_swap_reward"))
}

pub fn execute_auto_compound_rewards(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "execute_auto_compound_rewards"))
}

pub fn update_pool(
    store: &mut dyn Storage,
    env: Env,
    channel: String,
    timeout: Option<u64>,
    sender: Addr,
    denom: String,
    port_id: String,
) -> StdResult<()> {
    // let res = claim_tokens(
    //     store,
    //     env.clone(),
    //     ClaimTokensMsg {
    //         channel,
    //         timeout,
    //         denom,
    //     },
    //     sender,
    //     port_id,
    // )
    // .unwrap();
    // Ok(Response::new().add_messages(res.messages))
    //     let reward_pools = REWARD_POOLS.load(deps.storage)?;
    //     let mut new_reward_pools: Vec<RewardPool> = vec![];
    //     let total_deposits = TOTAL_DEPOSITS.load(deps.storage)?;

    //     for mut reward_pool in reward_pools {
    //         if total_deposits.is_zero() {
    //             reward_pool.acc_reward_per_share = Uint128::from(env.block.time.seconds());
    //         } else {
    //             let new_rewards = Uint128::zero(); // claimed reward of selected pool
    //             reward_pool.acc_reward_per_share += new_rewards
    //                 .checked_mul(Uint128::from(1_000_000_000_000u128))?
    //                 .checked_div(total_deposits)?;
    //         }
    //         new_reward_pools.push(reward_pool);
    //     }

    //     REWARD_POOLS.save(deps.storage, &new_reward_pools)?;
    Ok(())
}

pub fn execute_update_pool(_deps: DepsMut, _env: Env) -> Result<Response, ContractError> {
    //     let reward_pools = REWARD_POOLS.load(deps.storage)?;
    //     let mut new_reward_pools: Vec<RewardPool> = vec![];
    //     let total_deposits = TOTAL_DEPOSITS.load(deps.storage)?;

    //     for mut reward_pool in reward_pools {
    //         if total_deposits.is_zero() {
    //             reward_pool.acc_reward_per_share = Uint128::from(env.block.time.seconds());
    //         } else {
    //             let new_rewards = Uint128::zero(); // claimed reward of selected pool
    //             reward_pool.acc_reward_per_share += new_rewards
    //                 .checked_mul(Uint128::from(1_000_000_000_000u128))?
    //                 .checked_div(total_deposits)?;
    //         }
    //         new_reward_pools.push(reward_pool);
    //     }

    //     REWARD_POOLS.save(deps.storage, &new_reward_pools)?;
    Ok(Response::new())
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
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner.to_string(),
        unbond_period: config.unbond_period,
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
