module.exports = async ({
    getNamedAccounts,
    deployments,
}) => {
    const { deploy } = deployments;
    const { deployer } = await getNamedAccounts();

    const AXELAR_GATEWAY = "0x4D147dCb984e6affEEC47e44293DA442580A3Ec0";

    await deploy('MessageReceiver', {
        from: deployer,
        gasLimit: 2000000,
        args: [AXELAR_GATEWAY],
        log: true
    });
};

module.exports.tags = ['contract'];