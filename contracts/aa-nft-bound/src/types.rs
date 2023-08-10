use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum NftId {
    Sdk {
        class_id: String,
        token_id: String,
    },
    Cw721 {
        contract_address: String,
        token_id: String,
    },
}
