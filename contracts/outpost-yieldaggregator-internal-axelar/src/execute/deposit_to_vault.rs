use crate::error::ContractError;
use crate::msgs::DepositToVaultMsg;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::one_coin;
use ununifi_binding::v1::binding::UnunifiMsg;

#[cfg(not(feature = "library"))]
pub fn execute_deposit_to_vault(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: DepositToVaultMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    use cosmwasm_std::{to_binary, Coin, WasmMsg};
    use swap_for_bridge::msgs::SwapMsg;

    use crate::state::PARAMS;

    let mut response = Response::new();
    let mut coin = one_coin(&info)?;

    // If we want to verify sender
    // https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/cosmwasm-integration/README.md
    // but it is not needed here

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    let params = PARAMS.load(deps.storage)?;

    if let Some(swap_output_denom) = msg.swap_output_denom {
        let contract = params
            .symbol_swap_contract_map
            .get(&swap_output_denom)
            .ok_or(ContractError::Unauthorized {})?;

        let fee_subtracted_amount = coin.amount.checked_sub(0u128.into())?;

        response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract.to_string(),
            msg: to_binary(&SwapMsg {
                output_denom: swap_output_denom.clone(),
                recipient: None,
            })?,
            funds: vec![coin],
        }));

        coin = Coin {
            denom: swap_output_denom,
            amount: fee_subtracted_amount,
        };
    }

    response = response.add_message(CosmosMsg::Custom(UnunifiMsg::DeputyDepositToVault {
        depositor: msg.depositor,
        vault_id: msg.vault_id,
        amount: coin,
    }));

    Ok(response)
}
