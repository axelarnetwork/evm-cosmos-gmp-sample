use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

/// Supply is dynamic and tracks the current supply of staked and ERC20 tokens.
#[cw_serde]
#[derive(Default)]
pub struct Supply {
    /// issued is how many derivative tokens this contract has issued
    pub issued: Uint128,
    /// bonded is how many native tokens exist bonded to the validator
    pub bonded: Uint128,
    /// claims is how many tokens need to be reserved paying back those who unbonded
    pub claims: Uint128,
}

#[cw_serde]
pub struct Config {
    pub channel: String,
    pub source_chain: String,
    pub linker_address: String,
    pub axelar_gmp_account: String,
}

pub const TOTAL_SUPPLY: Item<Supply> = Item::new("total_supply");
pub const CONFIG: Item<Config> = Item::new("cross_chain_token_config");
