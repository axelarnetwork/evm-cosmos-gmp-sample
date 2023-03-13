use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

pub const CONFIG: Item<Config> = Item::new("messenger_receiver_config");

#[cw_serde]
pub struct Config {
    pub last_received_message: String,
}
