use crate::helpers::setup;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Coin,
};
use swap_for_bridge::{
    execute::deposit_liquidity::execute_deposit_liquidity, msgs::DepositLiquidityMsg,
};
mod helpers;

#[test]
fn deposit_liquidity() {
    let mut deps = setup();

    let sender = "anyone";
    // Change with other values for further tests
    execute_deposit_liquidity(
        deps.as_mut(),
        mock_env(),
        mock_info(sender, &[Coin::new(100, "denom1")]),
        DepositLiquidityMsg {},
    )
    .unwrap();
}
