use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Map};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[cw_serde]
pub struct Config {
    pub crosschain_swap_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const CHANNEL_MAP: Map<&str,Vec<String>> = Map::new("channel_map");