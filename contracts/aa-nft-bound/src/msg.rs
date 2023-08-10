use crate::types::NftId;
use absacc::AccountSudoMsg;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ExecuteMsg {
    RegisterAccount(RegisterAccountMsg),
}

#[cw_serde]
pub struct RegisterAccountMsg {
    pub nft_id: NftId,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AccountResp)]
    Account { nft_id: NftId },
    #[returns(TokenResp)]
    Token { address: String },
}

#[cw_serde]
pub struct AccountResp {
    pub address: String,
}

#[cw_serde]
pub struct TokenResp {
    pub nft_id: NftId,
}

pub type SudoMsg = AccountSudoMsg;
