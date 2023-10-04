import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import * as dotenv from "dotenv";

dotenv.config();

// Go to https://infura.io, sign up, create a new API key
// in its dashboard, and replace "KEY" with it
const INFURA_API_KEY = process.env.INFURA_API_KEY;

// Replace this private key with your Goerli account private key
// To export your private key from Coinbase Wallet, go to
// Settings > Developer Settings > Show private key
// To export your private key from Metamask, open Metamask and
// go to Account Details > Export Private Key
// Beware: NEVER put real Ether into testing accounts
const PRIV_KEY = process.env.PRIV_KEY || "";

const config: HardhatUserConfig = {
  solidity: "0.8.19",
  networks: {
    goerli: {
      url: `https://goerli.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIV_KEY],
    },
    sepolia: {
      url: `https://sepolia.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIV_KEY],
    },
    mumbai: {
      url: `https://polygon-mumbai.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIV_KEY],
    },
    fuji: {
      url: `https://avalanche-fuji.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIV_KEY],
    },
    arbitrum: {
      url: `https://arbitrum-goerli.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIV_KEY],
    },
  },
  etherscan: {
    // Your API key for Etherscan
    // Obtain one at https://etherscan.io/
    apiKey: process.env.ETHERSCAN_API_KEY,
  },
};

export default config;
