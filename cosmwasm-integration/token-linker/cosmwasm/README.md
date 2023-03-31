# Crosschain Token

Corsschain token inherits from cw20-base, and adds ExecuteFromRemote and TrasnferToRemote functions to allow crosschain token transfer.

### Initialization
To initialize a crosschain token, you need to provide the following parameters :
    - original_chain: the chain name of the original token deployed on the EVM chain
    - linker_address: the address of the token linker contract deployed on the EVM chain
    - axelar_gmp_account: the axelar gmp account address representation on the Cosmos chain