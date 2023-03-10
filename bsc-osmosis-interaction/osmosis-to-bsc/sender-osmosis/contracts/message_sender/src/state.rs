use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

pub const CONFIG: Item<Config> = Item::new("multi_send_config");

#[cw_serde]
pub struct Config {
    pub channel: String,
}
