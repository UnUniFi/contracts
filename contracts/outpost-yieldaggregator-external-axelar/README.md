# Sample Hardhat Project

This project demonstrates a basic Hardhat use case. It comes with a sample contract, a test for that contract, and a script that deploys that contract.

Try running some of the following tasks:

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat run scripts/deploy.ts
```

## For Goerli

```shell
npx hardhat run scripts/deploy.ts --network goerli
npx hardhat verify YOUR_CONTRACT_ADDRESS AXL_GATEWAY_ADDRESS AXL_GAS_SERVICE_ADDRESS CHAIN_name --network goerli
```
