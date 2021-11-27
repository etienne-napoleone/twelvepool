use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Msg {
    #[serde(rename = "type")]
    type_: String,
    value: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Fee {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Signature {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Tx {
    msg: Vec<Msg>,
    fee: Fee,
    signatures: Vec<Signature>,
    memo: String,
    timeout_height: String,
}
