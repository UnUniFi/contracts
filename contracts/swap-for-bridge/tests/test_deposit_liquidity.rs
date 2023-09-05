use cosmwasm_std::{Coin, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info};
use swap_for_bridge::error::ContractError;
use swap_for_bridge::execute::deposit_liquidity::execute_deposit_liquidity;
use swap_for_bridge::msgs::DepositLiquidityMsg;
use swap_for_bridge::query::share::query_share;

use crate::helpers::setup;

mod helpers;

#[test]
fn test_deposit_liquidity() {
    let mut deps = setup();
    let sender = "anyone";
    let msg = DepositLiquidityMsg {};

    // Failure due to invalid denom
    {   
        let info = mock_info(sender, &[Coin{denom: String::from("uguu"), amount: Uint128::one()}]);
        let res = execute_deposit_liquidity(deps.as_mut(), mock_env(), info, msg.clone());
        assert!(res.is_err());
        assert_eq!(ContractError::InvalidDenom, res.unwrap_err());
    }

    // Success
    {   
        let info = mock_info(sender, &[Coin{denom: String::from("uatom"), amount: Uint128::new(100)}]);
        let res = execute_deposit_liquidity(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());

        let share = query_share(deps.as_ref(), sender.to_string()).unwrap();
        assert_eq!(info.funds[0].amount, share.share);
    }

    // Success on additional deposit
    {   
        let info = mock_info(sender, &[Coin{denom: String::from("uatom"), amount: Uint128::new(100)}]);
        let res = execute_deposit_liquidity(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());

        let share = query_share(deps.as_ref(), sender.to_string()).unwrap();
        assert_eq!(info.funds[0].amount.checked_mul(Uint128::new(2)).unwrap(), share.share);
    }
}