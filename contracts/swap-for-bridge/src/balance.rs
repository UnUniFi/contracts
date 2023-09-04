use cosmwasm_std::{Addr, BalanceResponse, BankQuery, Deps, QueryRequest, Uint128};

use crate::error::ContractError;

pub fn get_total_amounts(
    deps: Deps,
    addr: Addr,
    denoms: Vec<String>,
) -> Result<Uint128, ContractError> {
    let sum = Uint128::new(0);

    for denom in denoms {
        let request = QueryRequest::Bank(BankQuery::Balance {
            address: addr.to_string(),
            denom,
        });
        let response = deps.querier.query::<BalanceResponse>(&request)?;
        let coin = response.amount;

        sum.checked_add(coin.amount)?;
    }

    Ok(sum)
}
