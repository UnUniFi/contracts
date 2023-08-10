use crate::error::ContractError;
use crate::helpers::swap_msg_to_any;
use crate::state::{Config, DepositToken, CONFIG};
use cosmwasm_std::{Env, Response, StdError, Storage};
use ica_tx::helpers::send_ica_tx;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountIn;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use ununifi_binding::v0::binding::UnunifiMsg;

use super::helpers::determine_ica_amounts;

pub fn execute_ica_swap_two_tokens_to_deposit_token(
    store: &mut dyn Storage,
    env: Env,
) -> Result<Response<UnunifiMsg>, ContractError> {
    let config: Config = CONFIG.load(store)?;
    let ica_amounts = determine_ica_amounts(config.to_owned());
    let mut in_denom = config.host_config.quote_denom.to_string();
    let mut out_denom = config.host_config.base_denom.to_string();
    if config.deposit_token == DepositToken::Quote {
        in_denom = config.host_config.base_denom.to_string();
        out_denom = config.host_config.quote_denom.to_string();
    }
    let to_swap_amount = ica_amounts.to_swap_amount;
    if to_swap_amount.is_zero() {
        return Ok(Response::new());
    }
    let msg = MsgSwapExactAmountIn {
        sender: config.ica_account.to_string(),
        token_in: Some(OsmosisCoin {
            denom: in_denom,
            amount: to_swap_amount.to_string(),
        }),
        token_out_min_amount: "1".to_string(),
        routes: vec![SwapAmountInRoute {
            pool_id: config.host_config.pool_id,
            token_out_denom: out_denom,
        }],
    };
    if let Ok(msg_any) = swap_msg_to_any(msg) {
        return Ok(send_ica_tx(
            env,
            config.ica_channel_id,
            config.transfer_timeout,
            "swap_to_deposit_token".to_string(),
            vec![msg_any],
        )?);
    }
    Err(ContractError::Std(StdError::SerializeErr {
        source_type: "proto_any_conversion".to_string(),
        msg: "".to_string(),
    }))
}
