import { ethers } from "hardhat";

async function main() {
  const [deployer] = await ethers.getSigners();

  console.log("Deploying contracts with the account:", deployer.address);

  const outpost = await ethers.deployContract("YieldAggregatorOutpost", [
    "0xe432150cce91c13a887f7D836923d5597adD8E31", // axelar gateway
    "0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6", // axelar gas service
    "ethereum-2", // fuji chain name
  ]);

  console.log("YieldAggregatorOutpost address:", await outpost.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
