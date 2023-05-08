# Strategy contract for yield aggregator v2

Yield aggregator is expecting following endpoints to be exposed by strategy contract.

## Message

```rs
    pub enum ExecuteMsg {
        Stake(StakeMsg),
        Unstake(UnstakeMsg),
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct StakeMsg {}

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct UnstakeMsg {
        pub amount: Uint128,
    }
```

### Stake

On `Stake`, staking amount is configured on `info.funds`

### Unstake

On `Unstake`, unstaking amount is put Uint128 variable on `UnstakeMsg`

## Query

````rs
    pub enum QueryMsg {
        Bonded { addr: String },
        Unbonding { addr: String },
    }
    ```
````

### Bonded

`Bonded` returns the value of `addr`'s bonded tokens.

### Unbonding

`Unbonding` returns the value of `addr`'s unbonding tokens.
