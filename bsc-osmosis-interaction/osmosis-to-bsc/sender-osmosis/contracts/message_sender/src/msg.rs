use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    /// IBC channel id connects to Axelar
    /// For example: `channel-1946` is the IBC channel id from Osmosis to Axelar
    /// ref: https://docs.axelar.dev/resources/testnet#ibc-channels
    pub channel: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Send a cross chain message without any tokens
    SendMessage {
        destination_chain: String,
        destination_address: String,
        message: String,
    },
    /// Send a cross chain message with tokens
    SendMessageWithTokens {
        destination_chain: String,
        destination_address: String,
        message: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
