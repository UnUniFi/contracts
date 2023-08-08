use crate::types::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub unbond_period: u64,
    pub deposit_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
    },
    ListNft {
        sender: String,
        source_chain: String,
        erc721_contract: String,
        erc721_token_id: String,
    },
    Borrow {
        sender: String,
        source_chain: String,
        erc721_contract: String,
        erc721_token_id: String,
        amount: Uint128,
    },
    Repay {
        source_chain: String,
        erc721_contract: String,
        erc721_token_id: String,
        amount: Uint128,
    },
    EndListing {
        sender: String,
        source_chain: String,
        erc721_contract: String,
        erc721_token_id: String,
    },
    WithdrawNft {
        sender: String,
        source_chain: String,
        erc721_contract: String,
        erc721_token_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
