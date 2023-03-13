module.exports = async ({
    getNamedAccounts,
    deployments,
}) => {
    const { deploy } = deployments;
    const { deployer } = await getNamedAccounts();

    const AXELAR_GATEWAY = "0x4D147dCb984e6affEEC47e44293DA442580A3Ec0";
    const AXELAR_GAS_RECEIVER = "0xbE406F0189A0B4cf3A05C286473D23791Dd44Cc6";

    await deploy('MessageSender', {
        from: deployer,
        gasLimit: 2000000,
        args: [AXELAR_GATEWAY, AXELAR_GAS_RECEIVER],
        log: true
    });
};

module.exports.tags = ['contract'];