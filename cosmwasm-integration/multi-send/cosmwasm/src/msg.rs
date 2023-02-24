use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    /// IBC channel id connects to Axelar
    pub channel: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Distribute equal amount among recipients
    MultiSend {
        recipients: Vec<String>
    },
    
    /// Send a cross chain message, distribute equal amount among recipients
    MultiSendToEvm {
        destination_chain: String,
        destination_address: String,
        recipients: Vec<String>
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
