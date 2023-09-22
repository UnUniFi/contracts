use crate::helpers::setup;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Coin, Uint128};
use swap_for_bridge::error::ContractError;
use swap_for_bridge::execute::swap::execute_swap;
use swap_for_bridge::msgs::SwapMsg;

mod helpers;

#[test]
fn test_swap() {
    let mut deps = setup();
    let sender = "anyone";

    // Failure due to wrong sending token
    {
        let swap_msg = SwapMsg {
            output_denom: String::from("uatom"),
            recipient: None,
        };
        let info = mock_info(
            sender,
            &[Coin {
                denom: String::from("uguu"),
                amount: Uint128::one(),
            }],
        );
        let res = execute_swap(deps.as_mut(), mock_env(), info, swap_msg);
        assert!(res.is_err());
        assert_eq!(ContractError::InvalidDenom, res.unwrap_err());
    }

    // Failure due to wrong output denom
    {
        let swap_msg = SwapMsg {
            output_denom: String::from("uguu"),
            recipient: None,
        };
        let info = mock_info(
            sender,
            &[Coin {
                denom: String::from("uatom"),
                amount: Uint128::one(),
            }],
        );
        let res = execute_swap(deps.as_mut(), mock_env(), info, swap_msg);
        assert!(res.is_err());
        assert_eq!(ContractError::InvalidDenom, res.unwrap_err());
    }

    // Success
    {
        let swap_msg = SwapMsg {
            output_denom: String::from("uatom"),
            recipient: None,
        };
        let info = mock_info(
            sender,
            &[Coin {
                denom: String::from("uatom"),
                amount: Uint128::new(100),
            }],
        );
        let res = execute_swap(deps.as_mut(), mock_env(), info.clone(), swap_msg);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().messages.len(), 2);
    }
}
