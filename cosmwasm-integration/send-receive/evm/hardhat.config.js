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
    fuji: {
      url: `https://avalanche-fuji.infura.io/v3/${process.env.FUJI_API_KEY}`,
      accounts: [process.env.DEPLOYER_PRIV_KEY],
    }
  }
};
