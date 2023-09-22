use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};

#[cw_serde]
pub struct Params {
    pub authority: Addr,
}

#[cw_serde]
pub struct Provider {
    pub id: u64,
    pub address: Addr,
    pub name: String,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    pub identity: String,
    /// website defines an optional website link.
    pub website: String,
    /// security_contact defines an optional email for security contact.
    pub security_contact: String,
    /// details define other optional details.
    pub details: String,
    pub information_fee: Coin,
    pub customer_fee_back_rate: Decimal,
}

#[cw_serde]
pub struct Verification {
    pub address: Addr,
    pub provider_id: u64,
}

#[cw_serde]
pub struct InformationRequest {
    pub customer: Addr,
    pub id: u64,
    pub sender: Addr,
    pub provider_id: u64,
    pub information_fee: Coin,
    pub email: String,
    pub approved: Option<bool>,
}
