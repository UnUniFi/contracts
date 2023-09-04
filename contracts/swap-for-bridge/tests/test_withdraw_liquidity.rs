use crate::helpers::setup;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Uint128,
};
use swap_for_bridge::{
    execute::withdraw_liquidity::execute_withdraw_liquidity, msgs::WithdrawLiquidityMsg,
};
mod helpers;

#[test]
fn withdraw_liquidity() {
    let mut deps = setup();

    let sender = "anyone";
    // Change with other values for further tests
    execute_withdraw_liquidity(
        deps.as_mut(),
        mock_env(),
        mock_info(sender, &[]),
        WithdrawLiquidityMsg {
            output_denom: "denom2".to_string(),
            share_amount: Uint128::from(100u128),
        },
    )
    .unwrap();
}
