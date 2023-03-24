use serde::{Deserialize, Serialize};

#[derive(
Clone,
Debug,
PartialEq,
Eq,
serde::Serialize,
serde::Deserialize,
schemars::JsonSchema,
)]
pub struct Fee {
    amount: String,
    recipient: String
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GeneralMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    #[serde(rename = "type")]
    pub type_: i64,
    pub fee: Option<Fee>
}
