use crate::error::ContractError;
use crate::msgs::DepositToVaultMsg;
use crate::state::PARAMS;
use cosmwasm_std::{to_binary, Coin, Decimal, Empty, WasmMsg, WasmQuery};
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, Response};
use cw_utils::one_coin;
use swap_for_bridge::{
    msgs::{QueryMsg, SwapMsg},
    types::Params,
};
use ununifi_binding::v1::binding::UnunifiMsg;

#[cfg(not(feature = "library"))]
pub fn execute_deposit_to_vault(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: DepositToVaultMsg,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let mut response = Response::new();
    let mut coin = one_coin(&info)?;

    // If we want to verify sender
    // https://github.com/axelarnetwork/evm-cosmos-gmp-sample/blob/main/cosmwasm-integration/README.md
    // but it is not needed here

    // https://docs.axelar.dev/dev/general-message-passing/cosmos-gmp

    // todo : impl swap
    let params = PARAMS.load(deps.storage)?;

    if let Some(swap_output_denom) = msg.swap_output_denom {
        if coin.denom != swap_output_denom {
            let contract = params
                .denom_swap_contract_map
                .get(&swap_output_denom)
                .ok_or(ContractError::Unauthorized {})?;

            let request = QueryRequest::<Empty>::Wasm(WasmQuery::Smart {
                contract_addr: contract.to_string(),
                msg: to_binary(&QueryMsg::Params {})?,
            });
            let params = deps.querier.query::<Params>(&request)?;

            let fee = Decimal::from_atomics(coin.amount, 0)?
                .checked_mul(params.fee_rate)?
                .to_uint_floor();
            let lp_fee = Decimal::from_atomics(fee, 0)?
                .checked_mul(params.lp_fee_weight)?
                .to_uint_floor();
            let fee_subtracted = coin.amount.checked_sub(fee)?.checked_sub(lp_fee)?;

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
                amount: fee_subtracted,
            };
        }
    }

    // todo: impl on chain
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
