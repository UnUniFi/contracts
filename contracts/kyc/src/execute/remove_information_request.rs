use crate::error::ContractError;
use crate::msgs::RemoveInformationRequestMsg;
use crate::state::INFORMATION_REQUESTS;
use crate::state::PROVIDERS;
use cosmwasm_std::{BankMsg, CosmosMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

#[cfg(not(feature = "library"))]
pub fn execute_remove_information_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RemoveInformationRequestMsg,
) -> Result<Response, ContractError> {
    use cosmwasm_std::{coins, Decimal};

    let mut response = Response::new();

    let customer = deps.api.addr_validate(&msg.customer)?;

    let request = INFORMATION_REQUESTS.load(deps.storage, (customer.clone(), msg.request_id))?;

    match request.approved {
        Some(true) => {
            let provider = PROVIDERS.load(deps.storage, request.provider_id)?;
            if info.sender != provider.address {
                return Err(ContractError::Unauthorized {});
            }

            let customer_pay_back = Decimal::from_atomics(request.information_fee.amount, 0)?
                .checked_mul(provider.customer_fee_back_rate)?
                .to_uint_floor();
            let provider_amount = request
                .information_fee
                .amount
                .checked_sub(customer_pay_back)?;

            response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: request.customer.to_string(),
                amount: coins(
                    customer_pay_back.u128(),
                    request.information_fee.denom.clone(),
                ),
            }));
            response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: provider.address.to_string(),
                amount: coins(provider_amount.u128(), request.information_fee.denom),
            }));
        }
        Some(false) | None => {
            if info.sender != request.sender {
                return Err(ContractError::Unauthorized {});
            }
            response = response.add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: request.sender.to_string(),
                amount: vec![request.information_fee],
            }));
        }
    };

    INFORMATION_REQUESTS.remove(deps.storage, (customer, msg.request_id));

    response = response.add_attribute("action", "remove_information_request");

    Ok(response)
}
