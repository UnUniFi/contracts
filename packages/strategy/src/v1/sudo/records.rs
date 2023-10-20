use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct TransferCallbackData {
    pub denom: String,
    pub amount: String,
    pub sender: String,
    pub receiver: String,
    pub memo: String,
    pub success: bool,
}

#[cw_serde]
pub struct DepositCallbackData {
    pub denom: String,
    pub amount: String,
    pub sender: String,
    pub receiver: String,
    pub success: bool,
}
