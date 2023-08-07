use crate::error::ContractError;
use cosmwasm_std::{Binary, Coin, IbcTimeout};
use cosmwasm_std::{CosmosMsg, CustomMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// A number of Custom messages that can call into the Neutron bindings
pub enum UnunifiMsg {
    SubmitICQRequest {
        connection_id: String,
        chain_id: String,
        query_prefix: String,
        query_key: Binary,
    },
    IbcTransfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout: IbcTimeout,
    },
}

impl From<UnunifiMsg> for CosmosMsg<UnunifiMsg> {
    fn from(msg: UnunifiMsg) -> CosmosMsg<UnunifiMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for UnunifiMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    #[serde(rename = "kv_query_result")]
    KVQueryResult {
        connection_id: String,
        chain_id: String,
        query_prefix: String,
        query_key: Binary,
        data: Binary,
    },
    #[serde(rename = "transfer_callback")]
    TransferCallback {
        denom: String,
        amount: String,
        sender: String,
        receiver: String,
        memo: String,
        success: bool,
    },
}

/// Protobuf type url of standard Cosmos SDK bank transfer message
pub const COSMOS_SDK_TRANSFER_MSG_URL: &str = "/cosmos.bank.v1beta1.MsgSend";

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

/// Maximum length of address
pub const MAX_ADDR_LEN: usize = 255;

/// Name of the standard **bank** Cosmos-SDK module
pub const BANK_STORE_KEY: &str = "store/bank/key";

/// Name of osmosis **gamm** module
pub const GAMM_STORE_KEY: &str = "store/gamm/key";

/// Bytes representations of Bech32 address
pub type AddressBytes = Vec<u8>;

/// A **data structure** that can be reconstructed from slice of **Binary** structures.
/// Neutron provides `KVReconstruct` for many primitive and standard Cosmos-SDK types and query responses.
/// The complete list is [here][TODO_LINK]. All of these can be deserialized using Neutron out of the box.
///
/// Third-party projects may provide `KVReconstruct` implementations for types that they introduce.
/// For example if some query is not implemented in Neutron standard library, developers can create their own type/query and implement `KVReconstruct` for it.
///
///
/// Usually used together with `query_kv_result` function. For example, there is an Interchain Query with some `query_id` to query balance from remote chain.
/// And there is an implemented `KVReconstruct` for `Balance` structure.
/// So you can easily get reconstructed response for the query just in one line:
/// ```rust ignore
/// let balances: Balances = query_kv_result(deps, query_id)?;
/// ```
///
/// Anyone can implement `KVReconstruct` for any type and use `query_kv_result` without any problems.
pub trait KVReconstruct: Sized {
    /// Reconstructs this value from the slice of **Binary**'s.
    fn reconstruct(value: Binary) -> Result<Self, ContractError>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// A structure that can be reconstructed from **Binarys**'s for the **Balance Interchain Query**.
/// Contains coins that are held by some account on remote chain.
pub struct Balances {
    pub coins: Vec<Coin>,
}

// impl KVReconstruct for Balances {
//     fn reconstruct(value: Binary) -> Result<Balances, ContractError> {
//         let mut coins: Vec<Coin> = vec![];

//         let balance: ProtoCoin = ProtoCoin::decode(value.as_slice())?;
//         let amount = Uint128::from_str(balance.amount.as_str())?;
//         coins.push(Coin::new(amount.u128(), balance.denom));

//         Ok(Balances { coins })
//     }
// }
