use crate::errors;
use crate::responses;
use crate::tx::Tx;

use base64::decode;
use maplit::hashmap;
use reqwest::Client;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Terra {
    rpc_url: String,
    lcd_url: String,
    http_client: Client,
}

impl Terra {
    pub fn new(rpc_url: String, lcd_url: String, http_client: Client) -> Terra {
        Terra {
            rpc_url,
            lcd_url,
            http_client,
        }
    }

    pub async fn decode_tx(&self, tx_string: &str) -> Result<Tx, errors::RequestError> {
        let res = self
            .http_client
            .post(format!("{}/txs/decode", self.lcd_url))
            .json(&hashmap! { "tx" => tx_string })
            .send()
            .await?
            .json::<responses::DecodeTxResponse>()
            .await?;
        log::debug!("tx with {} msgs decoded", res.result.msg.len());
        Ok(res.result)
    }

    pub async fn get_tx_hash(&self, tx_string: &str) -> Result<String, errors::TxHashDecodeError> {
        // create a Sha256 object
        let mut hasher = Sha256::new();
        hasher.update(decode(tx_string)?);
        let tx_bytes = hasher.finalize();
        let hash = format!("{:X}", tx_bytes);
        log::debug!("got tx hash {} from tx_string", hash);
        Ok(hash)
    }

    pub async fn get_unconfirmed_txs(&self) -> Result<Vec<String>, errors::RequestError> {
        let res = self
            .http_client
            .get(format!(
                "{}/unconfirmed_txs?limit=1000000000000",
                self.rpc_url
            ))
            .send()
            .await?
            .json::<responses::UnconfirmedTxsResponse>()
            .await?;
        log::debug!("fetched {} unconfirmed txs", res.result.txs.len());
        Ok(res.result.txs)
    }
}
