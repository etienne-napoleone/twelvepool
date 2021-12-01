use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Tx {
    pub msg: Vec<Msg>,
    pub fee: Fee,
    pub signatures: Vec<Signature>,
    pub memo: String,
    pub timeout_height: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Msg {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: Value,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Fee {
    pub amount: Vec<Coin>,
    pub gas: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Signature {
    pub pub_key: PubKey,
    pub signature: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PubKey {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}
