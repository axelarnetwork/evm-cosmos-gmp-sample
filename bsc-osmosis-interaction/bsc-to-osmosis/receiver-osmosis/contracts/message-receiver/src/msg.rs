use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    /// Send a cross chain message without any tokens
    ReceiveMessage {
        destination_chain: String,
        destination_address: String,
        message: String,
    },
    /// Send a cross chain message with tokens
    ReceiveMessageWithToken {
        destination_chain: String,
        destination_address: String,
        message: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
}
