use crate::types::{Params, Provider, Verification};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub authority: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateParams(UpdateParamsMsg),
    RegisterProvider(RegisterProviderMsg),
    UpdateProvider(UpdateProviderMsg),
    CreateVerification(CreateVerificationMsg),
    RemoveVerification(RemoveVerificationMsg),
    RequestInformation(RequestInformationMsg),
    ApproveInformationRequest(ApproveInformationRequestMsg),
    RejectInformationRequest(RejectInformationRequestMsg),
    RemoveInformationRequest(RemoveInformationRequestMsg),
}

#[cw_serde]
pub struct UpdateParamsMsg {
    pub authority: Option<String>,
}

#[cw_serde]
pub struct RegisterProviderMsg {
    pub address: String,
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
}

#[cw_serde]
pub struct UpdateProviderMsg {
    pub id: u64,
    pub address: Option<String>,
    pub name: Option<String>,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    pub identity: Option<String>,
    /// website defines an optional website link.
    pub website: Option<String>,
    /// security_contact defines an optional email for security contact.
    pub security_contact: Option<String>,
    /// details define other optional details.
    pub details: Option<String>,
    pub information_fee: Option<Coin>,
}

#[cw_serde]
pub struct CreateVerificationMsg {
    pub provider_id: u64,
    pub customer: String,
}

#[cw_serde]
pub struct RemoveVerificationMsg {
    pub provider_id: u64,
    pub customer: String,
}

#[cw_serde]
pub struct RequestInformationMsg {
    pub provider_id: u64,
    pub customer: String,
    pub email: String,
}

#[cw_serde]
pub struct ApproveInformationRequestMsg {
    pub request_id: u64,
}

#[cw_serde]
pub struct RejectInformationRequestMsg {
    pub request_id: u64,
}

#[cw_serde]
pub struct RemoveInformationRequestMsg {
    pub request_id: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Params)]
    Params {},
    #[returns(Vec<Provider>)]
    Providers {},
    #[returns(Vec<Verification>)]
    Verifications { address: String },
}

/// We currently take no arguments for migrations
#[cw_serde]
pub struct MigrateMsg {}
