use crate::tx::Tx;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Result {
    pub txs: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UnconfirmedTxsResponse {
    pub result: Result,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DecodeTxResponse {
    pub result: Tx,
}
