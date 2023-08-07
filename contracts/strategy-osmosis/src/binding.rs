use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// A structure that can be reconstructed from **Binarys**'s for the **Balance Interchain Query**.
/// Contains coins that are held by some account on remote chain.
pub struct Balances {
    pub coins: Vec<Coin>,
}
