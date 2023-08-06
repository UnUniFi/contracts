use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct TransferCallbackData {
    denom: String,
    amount: String,
    sender: String,
    receiver: String,
    memo: String,
    success: bool,
}
