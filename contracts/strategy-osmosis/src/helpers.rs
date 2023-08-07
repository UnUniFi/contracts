use crate::error::ContractError;
use crate::msgs::{ChannelResponse, FeeInfo, ListChannelsResponse};
use crate::state::{
    Config, Unbonding, CHANNEL_INFO, CONFIG, DEPOSITS, HOST_LP_RATE_MULTIPLIER,
    STAKE_RATE_MULTIPLIER, UNBONDINGS,
};
use cosmwasm_std::{
    Addr, BalanceResponse, BankQuery, QuerierWrapper, QueryRequest, StdResult, Uint128,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgExitPool, MsgJoinPool, MsgSwapExactAmountIn};
use osmosis_std::types::osmosis::lockup::{MsgBeginUnlocking, MsgLockTokens};
use prost::EncodeError;
use prost_types::Any;

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

/// Storage prefix for account balances store
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/bank/types/key.go#L27
pub const BALANCES_PREFIX: u8 = 0x02;

/// Storage prefix for pools
/// https://github.com/osmosis-labs/osmosis/blob/main/x/gamm/types/key.go#L27
pub const POOLS_PREFIX: u8 = 0x02;

/// Key for delegations in the **staking** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L39
pub const DELEGATION_KEY: u8 = 0x31;

/// Key for validators in the **staking** module's storage
/// https://github.com/cosmos/cosmos-sdk/blob/35ae2c4c72d4aeb33447d5a7af23ca47f786606e/x/staking/types/keys.go#L35
pub const VALIDATORS_KEY: u8 = 0x21;

/// Name of the standard **bank** Cosmos-SDK module
pub const BANK_STORE_KEY: &str = "store/bank/key";

/// Name of osmosis **gamm** module
pub const GAMM_STORE_KEY: &str = "store/gamm/key";

/// Bytes representations of Bech32 address
pub type AddressBytes = Vec<u8>;

/// Decodes a bech32 encoded string and converts to base64 encoded bytes
/// https://github.com/cosmos/cosmos-sdk/blob/ad9e5620fb3445c716e9de45cfcdb56e8f1745bf/types/bech32/bech32.go#L20
pub fn decode_and_convert(encoded: &str) -> Result<AddressBytes, ContractError> {
    let (_hrp, bytes, _variant) = bech32::decode(encoded)?;

    Ok(bech32::convert_bits(&bytes, 5, 8, false)?)
}

/// Prefixes the address bytes with its length
pub fn length_prefix<AddrBytes: AsRef<[u8]>>(addr: AddrBytes) -> Result<Vec<u8>, ContractError> {
    let bz_length = addr.as_ref().len();

    if bz_length == 0 {
        return Ok(vec![]);
    }

    if bz_length > MAX_ADDR_LEN {
        return Err(ContractError::MaxAddrLength {});
    }

    let mut p: Vec<u8> = vec![bz_length as u8];
    p.extend_from_slice(addr.as_ref());

    Ok(p)
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

pub fn join_pool_to_any(msg: MsgJoinPool) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.gamm.v1beta1.MsgJoinPool".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn exit_pool_to_any(msg: MsgExitPool) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.gamm.v1beta1.MsgExitPool".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn swap_msg_to_any(msg: MsgSwapExactAmountIn) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn lock_tokens_msg_to_any(msg: MsgLockTokens) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.lockup.MsgLockTokens".to_owned(),
        value: msg.to_proto_bytes(),
    });
}

pub fn begin_unlocking_msg_to_any(msg: MsgBeginUnlocking) -> Result<Any, EncodeError> {
    return Ok(Any {
        type_url: "/osmosis.lockup.MsgBeginUnlocking".to_owned(),
        value: msg.to_proto_bytes(),
    });
}
