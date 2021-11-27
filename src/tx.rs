use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Msg {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Fee {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Signature {}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Tx {
    pub msg: Vec<Msg>,
    pub fee: Fee,
    pub signatures: Vec<Signature>,
    pub memo: String,
    pub timeout_height: String,
}
