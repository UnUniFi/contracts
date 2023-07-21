use crate::binding::{
    SudoMsg, UnunifiMsg, BALANCES_PREFIX, BANK_STORE_KEY, GAMM_STORE_KEY, POOLS_PREFIX,
};
use crate::helpers::{decode_and_convert, length_prefix};
use crate::ica::{
    execute_ibc_transfer_to_controller, execute_ica_add_and_bond_liquidity,
    execute_ica_begin_unbonding_lp_tokens, execute_ica_remove_liquidity,
    execute_ica_swap_balance_to_two_tokens, execute_ica_swap_two_tokens_to_deposit_token,
};
use crate::msg::{
    ChannelResponse, ExecuteMsg, FeeInfo, InstantiateMsg, ListChannelsResponse, MigrateMsg,
    QueryMsg, UpdateConfigMsg,
};
use crate::state::{
    ChannelInfo, Config, DepositInfo, EpochCallSource, IcaAmounts, Phase, Unbonding, CHANNEL_INFO,
    CONFIG, DEPOSITS, HOST_LP_RATE_MULTIPLIER, STAKE_RATE_MULTIPLIER, UNBONDINGS,
};
use crate::state::{ControllerConfig, HostConfig};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, coins, to_binary, Addr, BalanceResponse, BankMsg, BankQuery, Binary, Coin, CosmosMsg,
    Decimal, Deps, DepsMut, Env, IbcTimeout, MessageInfo, Order, QuerierWrapper, QueryRequest,
    Response, StdResult, Storage, Uint128,
};
use cw_utils::one_coin;
use osmosis_std::types::osmosis::gamm::v1beta1::Pool as OsmosisBalancerPool;
use osmosis_std::types::osmosis::lockup::MsgLockTokensResponse;
use prost::Message;
use prost_types::Any;
use proto::cosmos::base::abci::v1beta1::TxMsgData;
use proto::cosmos::base::v1beta1::Coin as ProtoCoin;
use std::str::FromStr;
use strategy::error::{ContractError, NoDeposit};

//Initialize the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config = Config {
        owner: info.sender,
        unbond_period: msg.unbond_period,
        total_deposit: Uint128::from(0u128),
        last_unbonding_id: 1u64,
        redemption_rate: STAKE_RATE_MULTIPLIER,
        total_withdrawn: Uint128::from(0u128),
        transfer_timeout: 300, // 300s
        ica_connection_id: "".to_string(),
        ica_channel_id: "".to_string(),
        ica_account: "".to_string(),
        phase: Phase::Deposit,
        phase_step: 1u64,
        pending_icq: 0u64,
        host_config: HostConfig {
            chain_id: "test-1".to_string(),
            pool_id: 1,
            transfer_channel_id: "channel-1".to_string(),
            lp_redemption_rate: Uint128::from(200000u128),
            lock_id: 0u64,
            lp_denom: "gamm/pool/1".to_string(), // ATOM-OSMO
            bonded_lp_amount: Uint128::from(0u128),
            free_lp_amount: Uint128::from(0u128),
            pending_bond_lp_amount: Uint128::from(0u128),
            pending_lp_removal_amount: Uint128::from(0u128), // pending swap from lp to deposit token amount
            osmo_denom: "uosmo".to_string(),                 // OSMO
            free_osmo_amount: Uint128::from(0u128),
            pending_swap_to_atom_amount: Uint128::from(0u128), // Convert OSMO to ATOM
            atom_denom: "stake".to_string(),                   // ATOM
            free_atom_amount: Uint128::from(0u128),            // free ATOM balance
            pending_swap_to_osmo_amount: Uint128::from(0u128), // pending swap from ATOM -> OSMO to add liquidity
            pending_add_liquidity_amount: Uint128::from(0u128), // amount of ATOM used on liquidity addition
            pending_transfer_amount: Uint128::from(0u128), // pending transfer to controller - TODO: how to get hook for transfer finalization?
        },
        controller_config: ControllerConfig {
            transfer_channel_id: "channel-1".to_string(),
            deposit_denom: "stake".to_string(), // `ibc/xxxxuatom`
            free_amount: Uint128::from(0u128),
            pending_transfer_amount: Uint128::from(0u128), // TODO: where to get hook for transfer finalization?
            stacked_amount_to_deposit: Uint128::from(0u128), // TODO: to be set to 0 when deposit happens at `Deposit` phase
        },
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[entry_point]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response<UnunifiMsg>, ContractError> {
    match msg {
        SudoMsg::KVQueryResult {
            connection_id,
            chain_id,
            query_prefix,
            query_key,
            data,
        } => sudo_kv_query_result(
            deps,
            env,
            connection_id,
            chain_id,
            query_prefix,
            query_key,
            data,
        ),
        SudoMsg::TransferCallback {
            denom,
            amount,
            sender,
            receiver,
            memo,
            success,
        } => sudo_transfer_callback(deps, env, denom, amount, sender, receiver, memo, success),
    }
}

pub fn sudo_transfer_callback(
    deps: DepsMut,
    env: Env,
    denom: String,
    amount: String,
    sender: String,
    receiver: String,
    memo: String,
    success: bool,
) -> Result<Response<UnunifiMsg>, ContractError> {
    deps.api
        .debug(format!("WASMDEBUG: sudo_transfer_callback received",).as_str());
    execute_epoch(deps, env, EpochCallSource::TransferCallback, success, None)?;
    let res = Response::new().add_attribute("action", "ibc_transfer_callback".to_string());
    return Ok(res);
}

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(
    deps: DepsMut,
    env: Env,
    connection_id: String,
    chain_id: String,
    query_prefix: String,
    query_key: Binary,
    data: Binary,
) -> Result<Response<UnunifiMsg>, ContractError> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_key,
        )
        .as_str(),
    );

    let mut config: Config = CONFIG.load(deps.storage)?;
    let converted_addr_bytes = decode_and_convert(&config.ica_account.as_str())?;
    let atom_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.atom_denom.to_string(),
    )?;
    let osmo_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.osmo_denom.to_string(),
    )?;
    let lp_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.lp_denom.to_string(),
    )?;

    if query_prefix == BANK_STORE_KEY {
        if query_key == atom_balance_key {}
        let mut amount = Uint128::from(0u128);
        if data.len() > 0 {
            // TODO: to update if Osmosis update Cosmos version to v0.47
            let balance: ProtoCoin = ProtoCoin::decode(data.as_slice())?;
            amount = Uint128::from_str(balance.amount.as_str())?;
        }
        if query_key == atom_balance_key {
            config.host_config.free_atom_amount = amount;
        } else if query_key == osmo_balance_key {
            config.host_config.free_osmo_amount = amount;
        } else if query_key == lp_balance_key {
            config.host_config.free_lp_amount = amount;
        }
    } else {
        // GAMM_STORE_KEY
        let any: Any = Any::decode(data.as_slice())?;
        let pool: OsmosisBalancerPool = OsmosisBalancerPool::decode(any.value.as_slice())?;
        let mut atom_amount = Uint128::from(0u128);
        let mut total_share = Uint128::from(0u128);
        for pool_asset in pool.pool_assets {
            if let Some(token) = pool_asset.token {
                if token.denom == config.host_config.atom_denom.to_string() {
                    atom_amount = Uint128::from_str(token.amount.as_str())?;
                    break;
                }
            }
        }
        if let Some(total_shares) = pool.total_shares {
            total_share = Uint128::from_str(total_shares.amount.as_str())?;
        }
        config.host_config.lp_redemption_rate =
            atom_amount * Uint128::from(2u128) * HOST_LP_RATE_MULTIPLIER / total_share;
    }

    config.pending_icq -= 1;
    CONFIG.save(deps.storage, &config)?;

    if config.pending_icq == 0 {
        execute_epoch(deps, env, EpochCallSource::IcqCallback, true, None)?;
    }

    Ok(Response::default())
}

//Execute the handle messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig(msg) => execute_update_config(deps, env, info, msg),
        ExecuteMsg::Stake(_) => {
            let coin: Coin = one_coin(&info)?;
            execute_stake(deps, env, coin, info.sender)
        }
        ExecuteMsg::Unstake(msg) => execute_unstake(deps, msg.amount, info.sender),
        ExecuteMsg::ExecuteEpoch(_) => {
            execute_epoch(deps, env, EpochCallSource::NormalEpoch, true, None)
        }
    }
}

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
    if let Some(lp_redemption_rate) = msg.lp_redemption_rate {
        config.host_config.lp_redemption_rate = lp_redemption_rate;
    }
    if let Some(transfer_channel_id) = msg.transfer_channel_id {
        config.host_config.transfer_channel_id = transfer_channel_id;
    }
    if let Some(osmo_denom) = msg.osmo_denom {
        config.host_config.osmo_denom = osmo_denom;
    }
    if let Some(atom_denom) = msg.atom_denom {
        config.host_config.atom_denom = atom_denom;
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

// Regular epoch operation (once per day)
// - icq balance of ica account when `Deposit` phase
// Unbonding epoch operation
// - begin lp unbonding on host through ica tx per unbonding epoch - per day probably - (if to unbond lp is not enough, wait for icq to update bonded lp correctly)
// `Deposit` phase operations
// - This phase starts when `WithdrawToUser` phase ends
// - ibc transfer to host for newly incoming atoms
// - ibc transfer to host for stacked atoms during withdraw phases
// - swap half atom to osmo & half osmo to atom in a single ica tx
// - initiate and wait for icq to update latest balances
// - add liquidity & bond in a single ica tx
// - repeat the flow
// `DepositEnding` phase operations
// - This phase starts from `Deposit` phase, when ica free lp balance is positive
// - ibc transfers are disabled
// - swap half atom to osmo & half osmo to atom in a single ica tx
// - wait for icq to update latest balances
// - add liquidity & bond in a single ica tx
// - initiate and wait for icq to update latest balances
// - update to phase to `LqWithdraw`
// `Withdraw` phase operations
// - This phase starts when `DepositEnding` phase ends
// - Mark unbond ending queue items on contract
// - execute remove liquidity operation
// - initiate and wait or icq to update latest balances
// - swap full osmo to atom
// - initiate and wait or icq to update latest balances
// - ibc transfer full atom balance from ica to contract
// - wait for ica callback for ibc transfer finalization
// - calculate amount to return, contract balance - stacked atom balance for deposit
// - send amounts to marked unbond ending items proportionally
// - switch to `Deposit` phase
pub fn determine_ica_amounts(config: Config) -> IcaAmounts {
    if config.phase == Phase::Withdraw {
        let mut amount_to_return = Uint128::from(0u128);
        if config.controller_config.free_amount > config.controller_config.stacked_amount_to_deposit
        {
            amount_to_return = config.controller_config.free_amount
                - config.controller_config.stacked_amount_to_deposit;
        }
        return IcaAmounts {
            to_swap_atom: Uint128::from(0u128),
            to_swap_osmo: config.host_config.free_atom_amount,
            to_remove_lp: config.host_config.free_lp_amount,
            to_add_lp: Uint128::from(0u128),
            to_transfer_to_controller: config.host_config.free_atom_amount,
            to_transfer_to_host: Uint128::from(0u128),
            to_return_amount: amount_to_return,
        };
    } else {
        let to_add_lp = config.host_config.free_atom_amount
            * HOST_LP_RATE_MULTIPLIER
            * Uint128::from(2u128)
            * Uint128::from(9u128)
            / Uint128::from(10u128)
            / config.host_config.lp_redemption_rate;

        return IcaAmounts {
            to_swap_atom: config.host_config.free_atom_amount / Uint128::from(2u128),
            to_swap_osmo: config.host_config.free_osmo_amount / Uint128::from(2u128),
            to_add_lp: to_add_lp,
            to_remove_lp: Uint128::from(0u128),
            to_transfer_to_controller: Uint128::from(0u128),
            to_transfer_to_host: config.controller_config.free_amount,
            to_return_amount: Uint128::from(0u128),
        };
    }
}

pub fn execute_stake(
    deps: DepsMut,
    env: Env,
    coin: Coin,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.controller_config.deposit_denom != coin.denom {
        return Err(ContractError::NoAllowedToken {});
    }
    let amount = coin.amount;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                let stake_amount = amount * STAKE_RATE_MULTIPLIER / config.redemption_rate;
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_add(stake_amount)?,
                });
            }
            Ok(DepositInfo {
                sender: sender.clone(),
                amount: amount,
            })
        },
    )?;
    config.total_deposit += amount;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::default()
        .add_attribute("action", "stake")
        .add_attribute("sender", sender)
        .add_attribute("amount", amount);
    Ok(rsp)
}

/// Creates balances Cosmos-SDK storage prefix for account with **addr**
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/x/bank/types/key.go#L55
pub fn create_account_balances_prefix<AddrBytes: AsRef<[u8]>>(
    addr: AddrBytes,
) -> Result<Vec<u8>, ContractError> {
    let mut prefix: Vec<u8> = vec![BALANCES_PREFIX];
    prefix.extend_from_slice(length_prefix(addr)?.as_slice());

    Ok(prefix)
}

/// Creates **denom** balance Cosmos-SDK storage key for account with **addr**
pub fn create_account_denom_balance_key<AddrBytes: AsRef<[u8]>, S: AsRef<str>>(
    addr: AddrBytes,
    denom: S,
) -> Result<Vec<u8>, ContractError> {
    let mut account_balance_key = create_account_balances_prefix(addr)?;
    account_balance_key.extend_from_slice(denom.as_ref().as_bytes());

    Ok(account_balance_key)
}

pub fn create_pool_key(pool_id: u64) -> Result<Vec<u8>, ContractError> {
    let mut pool_key: Vec<u8> = vec![POOLS_PREFIX];
    pool_key.extend_from_slice(pool_id.to_be_bytes().as_slice());

    Ok(pool_key)
}

// Submit the ICQ for the withdrawal account balance
pub fn submit_icq_for_host(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(store)?;
    config.pending_icq = 4u64;
    CONFIG.save(store, &config)?;

    let converted_addr_bytes = decode_and_convert(&config.ica_account.as_str())?;

    let atom_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.atom_denom,
    )?;
    let osmo_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.osmo_denom,
    )?;
    let lp_balance_key = create_account_denom_balance_key(
        converted_addr_bytes.clone(),
        config.host_config.lp_denom,
    )?;
    let gamm_pool_key = create_pool_key(config.host_config.pool_id)?;

    let msgs = vec![
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(atom_balance_key),
        },
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(osmo_balance_key),
        },
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: BANK_STORE_KEY.to_string(),
            query_key: Binary(lp_balance_key),
        },
        UnunifiMsg::SubmitICQRequest {
            chain_id: config.host_config.chain_id.to_string(),
            connection_id: config.ica_connection_id.to_string(),
            query_prefix: GAMM_STORE_KEY.to_string(),
            query_key: Binary(gamm_pool_key),
        },
    ];

    // Note: bonded lp and unbonding lp token balance could be managed without icq on contract side
    let resp = Response::new().add_messages(msgs);
    return Ok(resp);
}

pub fn query_balance(
    querier: &QuerierWrapper,
    account_addr: Addr,
    denom: String,
) -> StdResult<Uint128> {
    let balance: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount)
}

pub fn calc_matured_unbondings(store: &dyn Storage, env: Env) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(store)?;
    let mut total_matured_unbondings = Uint128::new(0);
    let unbondings = query_unbondings(store, Some(DEFAULT_LIMIT))?;
    for unbonding in unbondings {
        if unbonding.start_time + config.unbond_period < env.block.time.seconds() {
            total_matured_unbondings += unbonding.amount;
        }
    }
    Ok(total_matured_unbondings)
}

pub fn execute_epoch(
    deps: DepsMut,
    env: Env,
    called_from: EpochCallSource,
    success: bool,
    ret: Option<Vec<u8>>,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if let Ok(balance) = query_balance(
        &deps.querier,
        env.contract.address.to_owned(),
        config.controller_config.deposit_denom.to_string(),
    ) {
        config.controller_config.free_amount = balance;
        CONFIG.save(deps.storage, &config)?;
    }

    let mut rsp: Result<Response<UnunifiMsg>, ContractError> = Ok(Response::new());
    let mut next_phase = config.phase.to_owned();
    let mut next_phase_step = config.phase_step.to_owned();

    if config.phase == Phase::Withdraw {
        if config.phase_step == 1u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - Mark unbond ending queue items on contract
            // assumption: matured unbondings on the contract is same as matured unbondings on controller chain
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            for mut unbonding in unbondings {
                if unbonding.start_time + config.unbond_period < env.block.time.seconds() {
                    unbonding.marked = true;
                    UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                }
            }
            // - execute remove liquidity operation
            rsp = execute_ica_remove_liquidity(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 2u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
                let pending_lp_removal_amount = config.host_config.pending_lp_removal_amount;
                if success {
                    if config.host_config.bonded_lp_amount < pending_lp_removal_amount {
                        config.host_config.bonded_lp_amount = Uint128::from(0u128);
                    } else {
                        config.host_config.bonded_lp_amount -= pending_lp_removal_amount;
                    }
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
                config.host_config.pending_lp_removal_amount = Uint128::from(0u128);
                CONFIG.save(deps.storage, &config)?;
            }
        } else if config.phase_step == 3u64 {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 4u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 5u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - swap full osmo to atom
            rsp = execute_ica_swap_two_tokens_to_deposit_token(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 6u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else if config.phase_step == 7u64 {
            // - initiate and wait or icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 8u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 9u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - ibc transfer full atom balance from ica to contract
            rsp = execute_ibc_transfer_to_controller(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 10u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else if config.phase_step == 11u64 {
            // - refresh balance of host chain after ibc transfer callback
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 12u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // 13u64
            // - calculate amount to return, contract balance - stacked atom balance for deposit
            let amount_to_return = config.controller_config.free_amount
                - config.controller_config.stacked_amount_to_deposit;
            // - send amounts to marked unbond ending items proportionally
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            let mut total_marked_lp_amount = Uint128::from(0u128);
            for unbonding in unbondings.as_slice() {
                if unbonding.marked {
                    total_marked_lp_amount += unbonding.amount;
                }
            }
            if !total_marked_lp_amount.is_zero() {
                let mut resp: Response<UnunifiMsg> = Response::new();
                for unbonding in unbondings {
                    if unbonding.marked {
                        let returning_amount =
                            amount_to_return * unbonding.amount / total_marked_lp_amount;
                        let bank_send_msg = CosmosMsg::Bank(BankMsg::Send {
                            to_address: unbonding.sender.to_string(),
                            amount: coins(
                                returning_amount.into(),
                                &config.controller_config.deposit_denom,
                            ),
                        });
                        resp = resp.add_message(bank_send_msg);
                        UNBONDINGS.remove(deps.storage, unbonding.id);
                        // update the total_withdrawn amount in config just for the record
                        // memo: this param can be deleted in the future
                        config.total_withdrawn += returning_amount;
                    }
                }
                CONFIG.save(deps.storage, &config)?;
                rsp = Ok(resp);
            }
            // - switch to `Deposit` phase
            next_phase = Phase::Deposit;
            next_phase_step = 1u64;
        }
    } else {
        if config.phase_step == 1u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - ibc transfer to host for newly incoming atoms
            // - ibc transfer to host for stacked atoms during withdraw phases
            let ica_amounts = determine_ica_amounts(config.to_owned());
            let to_transfer_to_host = ica_amounts.to_transfer_to_host;
            if to_transfer_to_host.is_zero() {
                next_phase_step = config.phase_step + 2;
            } else {
                rsp = execute_ibc_transfer_to_host(deps.storage, env);
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 2u64 {
            // handle Transfer callback
            if called_from == EpochCallSource::TransferCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 3u64 {
            // - icq balance of ica account when `Deposit` phase
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 4u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 5u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - swap half atom to osmo & half osmo to atom in a single ica tx
            let ica_amounts = determine_ica_amounts(config.to_owned());
            let to_swap_atom = ica_amounts.to_swap_atom;
            let to_swap_osmo = ica_amounts.to_swap_osmo;
            if to_swap_atom.is_zero() && to_swap_osmo.is_zero() {
                next_phase_step = config.phase_step + 2;
            } else {
                rsp = execute_ica_swap_balance_to_two_tokens(deps.storage, env);
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 6u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else if config.phase_step == 7u64 {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 8u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 9u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // - add liquidity & bond in a single ica tx
            let share_out_amount = determine_ica_amounts(config.to_owned()).to_add_lp;
            if share_out_amount.is_zero() {
                next_phase_step = config.phase_step + 2;
            } else {
                rsp = execute_ica_add_and_bond_liquidity(deps.storage, env);
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 10u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                let mut config: Config = CONFIG.load(deps.storage)?;
                let pending_bond_lp_amount = config.host_config.pending_bond_lp_amount;
                if success {
                    if let Some(ret_bytes) = ret {
                        let tx_msg_data_result = TxMsgData::decode(&ret_bytes[..]);
                        if let Ok(tx_msg_data) = tx_msg_data_result {
                            if tx_msg_data.data.len() > 1 {
                                let msg_ret_result =
                                    MsgLockTokensResponse::decode(&tx_msg_data.data[1].data[..]);
                                if let Ok(msg_ret) = msg_ret_result {
                                    config.host_config.lock_id = msg_ret.id;
                                }
                            }
                        }
                    }
                    config.host_config.bonded_lp_amount += pending_bond_lp_amount;
                    next_phase_step = config.phase_step + 1;
                } else {
                    next_phase_step = config.phase_step - 1;
                }
                config.host_config.pending_bond_lp_amount = Uint128::from(0u128);
                CONFIG.save(deps.storage, &config)?;
            }
        } else if config.phase_step == 11u64 {
            // - initiate and wait for icq to update latest balances
            rsp = submit_icq_for_host(deps.storage, env);
            next_phase_step = config.phase_step + 1;
        } else if config.phase_step == 12u64 {
            // handle ICQ callback
            if called_from == EpochCallSource::IcqCallback {
                next_phase_step = config.phase_step + 1;
            }
        } else if config.phase_step == 13u64 {
            if called_from != EpochCallSource::NormalEpoch {
                return rsp;
            }
            // Unbonding epoch operation
            // - begin lp unbonding on host through ica tx per unbonding epoch - per day probably - (if to unbond lp is not enough, wait for icq to update bonded lp correctly)
            let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
            let mut unbonding_lp_amount = Uint128::from(0u128);
            for mut unbonding in unbondings {
                if unbonding.start_time != 0 || unbonding.pending_start == true {
                    continue;
                }
                unbonding.start_time = env.block.time.seconds();
                unbonding.pending_start = true;
                UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                unbonding_lp_amount += unbonding.amount;
            }

            if !unbonding_lp_amount.is_zero() {
                rsp = execute_ica_begin_unbonding_lp_tokens(deps.storage, env, unbonding_lp_amount);
                next_phase_step = config.phase_step + 1;
            } else {
                next_phase_step = config.phase_step + 2;
            }
        } else if config.phase_step == 14u64 {
            // handle ICA callback
            if called_from == EpochCallSource::IcaCallback {
                if success {
                    let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.pending_start == true {
                            unbonding.start_time = env.block.time.seconds();
                            unbonding.pending_start = false;
                            UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                        }
                    }

                    next_phase_step = config.phase_step + 1;
                } else {
                    let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
                    for mut unbonding in unbondings {
                        if unbonding.start_time != 0 && unbonding.pending_start == true {
                            unbonding.pending_start = false;
                            UNBONDINGS.save(deps.storage, unbonding.id, &unbonding)?;
                        }
                    }
                    next_phase_step = config.phase_step - 1;
                }
            }
        } else {
            // 15u64
            // when free lp amount and matured unbondings exist, move to withdraw phase
            let matured_unbondings = calc_matured_unbondings(deps.storage, env)?;
            if !config.host_config.free_lp_amount.is_zero()
                && matured_unbondings > Uint128::from(0u128)
            {
                next_phase = Phase::Withdraw;
            }
            next_phase_step = 1u64;
        }
    }

    // update phase
    let mut config: Config = CONFIG.load(deps.storage)?;
    config.phase = next_phase;
    config.phase_step = next_phase_step;
    CONFIG.save(deps.storage, &config)?;
    return rsp;
}

pub fn execute_unstake(
    deps: DepsMut,
    amount: Uint128,
    sender: Addr,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let unstake_amount = amount * STAKE_RATE_MULTIPLIER / config.redemption_rate;
    DEPOSITS.update(
        deps.storage,
        sender.to_string(),
        |deposit: Option<DepositInfo>| -> StdResult<_> {
            if let Some(unwrapped) = deposit {
                return Ok(DepositInfo {
                    sender: sender.clone(),
                    amount: unwrapped.amount.checked_sub(unstake_amount)?,
                });
            }
            Err(NoDeposit {}.into())
        },
    )?;

    let unbonding = &Unbonding {
        id: config.last_unbonding_id + 1,
        sender: sender.to_owned(),
        amount: amount * HOST_LP_RATE_MULTIPLIER / config.host_config.lp_redemption_rate,
        pending_start: false,
        start_time: 0u64,
        marked: false,
    };
    UNBONDINGS.save(deps.storage, unbonding.id, unbonding)?;

    // increase last unbonding id
    // NOTE: eventually, we should remove these params from config because it's simply double counting
    config.last_unbonding_id += 1;
    CONFIG.save(deps.storage, &config)?;

    let rsp = Response::new()
        .add_attribute("sender", sender.to_string())
        .add_attribute("amount", amount);
    Ok(rsp)
}

pub fn execute_ibc_transfer_to_host(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let to_transfer_to_host = ica_amounts.to_transfer_to_host;
    if to_transfer_to_host.is_zero() {
        return Ok(Response::new());
    }
    let timeout = env.block.time.plus_seconds(config.transfer_timeout);
    let ibc_msg = UnunifiMsg::IbcTransfer {
        channel_id: config.controller_config.transfer_channel_id,
        to_address: config.ica_account,
        amount: coin(
            to_transfer_to_host.u128(),
            config.controller_config.deposit_denom,
        ),
        timeout: IbcTimeout::from(timeout),
    };

    let mut config: Config = CONFIG.load(store)?;
    config.controller_config.stacked_amount_to_deposit = Uint128::from(0u128);
    config.controller_config.pending_transfer_amount += to_transfer_to_host;
    CONFIG.save(store, &config)?;

    let res = Response::new()
        .add_message(ibc_msg)
        .add_attribute("action", "ibc_transfer_to_host");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Unbonding { addr } => to_binary(&query_unbonding(deps, addr)?),
        QueryMsg::Bonded { addr } => to_binary(&query_bonded(deps, addr)?),
        QueryMsg::Fee {} => to_binary(&query_fee_info(deps)?),
        QueryMsg::ListChannels {} => to_binary(&query_list_channels(deps)?),
        QueryMsg::Channel { id } => to_binary(&query_channel(deps, id)?),
        QueryMsg::Unbondings {} => to_binary(&query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(config)
}

fn query_list_channels(deps: Deps) -> StdResult<ListChannelsResponse> {
    let channels = CHANNEL_INFO
        .range_raw(deps.storage, None, None, Order::Ascending)
        .map(|r| r.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;
    Ok(ListChannelsResponse { channels: channels })
}

// make public for ibc tests
pub fn query_channel(deps: Deps, id: String) -> StdResult<ChannelResponse> {
    let info = CHANNEL_INFO.load(deps.storage, &id)?;
    Ok(ChannelResponse { info })
}

pub fn query_fee_info(_: Deps) -> StdResult<FeeInfo> {
    Ok(FeeInfo {
        deposit_fee_rate: Decimal::zero(),
        withdraw_fee_rate: Decimal::zero(),
        interest_fee_rate: Decimal::zero(),
    })
}

pub fn query_unbonding(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let mut pending_unbonding_lp = Uint128::new(0u128);
    let unbondings = query_unbondings(deps.storage, Some(DEFAULT_LIMIT))?;
    for unbonding in unbondings {
        if unbonding.sender == addr {
            pending_unbonding_lp += unbonding.amount;
        }
    }
    Ok(pending_unbonding_lp * config.host_config.lp_redemption_rate / HOST_LP_RATE_MULTIPLIER)
}

pub fn query_bonded(deps: Deps, addr: String) -> StdResult<Uint128> {
    let config: Config = CONFIG.load(deps.storage)?;
    let deposit = DEPOSITS.load(deps.storage, addr)?;
    Ok(deposit.amount * config.redemption_rate / STAKE_RATE_MULTIPLIER)
}

const DEFAULT_LIMIT: u32 = 50;
pub fn query_unbondings(storage: &dyn Storage, limit: Option<u32>) -> StdResult<Vec<Unbonding>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT) as usize;

    UNBONDINGS
        .range(storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, v) = item?;
            Ok(v)
        })
        .collect::<StdResult<Vec<Unbonding>>>()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    Ok(Response::default())
}
