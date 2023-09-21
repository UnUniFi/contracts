use crate::error::ContractError;
use crate::msgs::DepositToVaultMsg;
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;
use ununifi_binding::v1::binding::UnunifiMsg;

#[cfg(not(feature = "library"))]
pub fn execute_deposit_to_vault(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: DepositToVaultMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut response = Response::new();
    let coin = one_coin(&info)?;

    // If we want to verify sender
    // https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/cosmwasm-integration/README.md
    // but it is not needed here

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // // todo: impl on chain
    // response = response.add_message(CosmosMsg::Custom(UnunifiMsg::DeputyDepositToVault {
    //     depositor: msg.depositor,
    //     vault_id: msg.vault_id,
    //     amount: coin,
    // }));

    response = response.add_message(CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
        to_address: msg.depositor,
        amount: vec![coin],
    }));

    Ok(response)
}
