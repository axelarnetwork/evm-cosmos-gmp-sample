require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.18",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    onyx: {
      url: process.env.RPC,
      accounts: [process.env.KEY || "0566b0ae7e92e8c645a80855c541b18be2d91858a8e4f7de82f7306666165a17"],  // test private key
    }
  }
};
