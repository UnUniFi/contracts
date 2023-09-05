use cosmwasm_std::{Coin, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info};
use swap_for_bridge::error::ContractError;
use swap_for_bridge::execute::deposit_liquidity::execute_deposit_liquidity;
use swap_for_bridge::execute::withdraw_liquidity::execute_withdraw_liquidity;
use swap_for_bridge::msgs::{DepositLiquidityMsg, WithdrawLiquidityMsg};
use swap_for_bridge::query::share::{query_share, query_total_share};

use crate::helpers::setup;

mod helpers;

#[test]
fn test_withdraw_liquidity() {
    let mut deps = setup();
    let sender = "anyone";
    let info = mock_info(sender, &[]);
    let deposit_liquidity_msg = DepositLiquidityMsg {};

    // Failure due to invalid denom
    {   
        let withdraw_msg = WithdrawLiquidityMsg{
            share_amount: Uint128::new(100),
            output_denom: String::from("uguu"),
        };
        let res = execute_withdraw_liquidity(deps.as_mut(), mock_env(), info.clone(), withdraw_msg.clone());
        assert!(res.is_err());
        assert_eq!(ContractError::InvalidDenom, res.unwrap_err());
    }

    // Failure due to insufficient funds
    {   
        let withdraw_msg = WithdrawLiquidityMsg{
            share_amount: Uint128::new(100),
            output_denom: String::from("uatom"),
        };
        let res = execute_withdraw_liquidity(deps.as_mut(), mock_env(), info.clone(), withdraw_msg.clone());
        assert!(res.is_err());
        assert_eq!(ContractError::InsufficientFunds, res.unwrap_err());
    }

    // Success
    {   
        let deposit_info = mock_info(sender, &[Coin{denom: String::from("uatom"), amount: Uint128::new(100)}]);
        execute_deposit_liquidity(deps.as_mut(), mock_env(), deposit_info, deposit_liquidity_msg.clone()).unwrap();

        let withdraw_msg = WithdrawLiquidityMsg{
            share_amount: Uint128::new(100),
            output_denom: String::from("uatom"),
        };
        let res = execute_withdraw_liquidity(deps.as_mut(), mock_env(), info.clone(), withdraw_msg.clone());
        assert!(res.is_ok());
        assert_eq!(res.unwrap().messages.len(), 2);

        let share = query_share(deps.as_ref(), sender.to_string()).unwrap();
        assert_eq!(Uint128::new(0), share.share);   
        let total_share = query_total_share(deps.as_ref()).unwrap();
        assert_eq!(Uint128::new(0), total_share.total_share);
    }
}