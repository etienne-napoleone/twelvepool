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

    pub async fn decode_tx(&self, tx_string: &String) -> Result<Tx, errors::RequestError> {
        let res = self
            .http_client
            .post(format!("{}/txs/decode", self.lcd_url))
            .json(&hashmap! { "tx" => tx_string })
            .send()
            .await?
            .json::<responses::DecodeTxResponse>()
            .await?;
        Ok(res.result)
    }

    pub async fn get_tx_hash(
        &self,
        tx_string: &String,
    ) -> Result<String, errors::TxHashDecodeError> {
        // create a Sha256 object
        let mut hasher = Sha256::new();
        hasher.update(decode(tx_string)?);
        let tx_bytes = hasher.finalize();
        Ok(format!("{:X}", tx_bytes))
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
        Ok(res.result.txs)
    }
}

#[cfg(test)]
mod tests {
    use super::Terra;
    use tokio_test::block_on;

    const TX_STR: &str = "Cp0BCo4BChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEm4KLHRlcnJhMXBqMmpwZ3l1Mmo4YW1nNXBkZnZqYzhkdjNnMHpkazcyaHpxZXdyEix0ZXJyYTE0bDQ2anJkZ2RoYXc0Y2VqdWt4NTBuZHAwaHNzOTVla3Qya2ZtdxoQCgV1bHVuYRIHMjEwOTk5NRIKMTg5NTQyMjk1MRJoClEKRgofL2Nvc21vcy5jcnlwdG8uc2VjcDI1NmsxLlB1YktleRIjCiEDmsiXXHtxyCbFAeeOfN7j4Ur4Z45hLi9zuNQ53j7mZ0ASBAoCCAEYiAgSEwoNCgV1bHVuYRIEMTA0ORDl0gUaQPT/oKGfhP5eHLYttAspV675WAshoWqPWe91x7VEy7/pWTmwmcsvxw4zYRrTO5Zh3mvaoO/MNOs/uJHhQLn2im0=";
    const TX_HASH: &str = "B884E37E0FF71E9FC81ACE90A54E85A7F5644FCB9BF50BBB5BA52835222DA9D1";

    #[test]
    fn tx_hash_is_correctly_computed() {
        let w = Terra::new(
            "test".to_string(),
            "test".to_string(),
            reqwest::Client::new(),
        );
        block_on(async {
            assert_eq!(w.get_tx_hash(&TX_STR.to_string()).await.unwrap(), TX_HASH);
        });
    }

    #[test]
    fn tx_str_is_correctly_decoded() {
        let w = Terra::new(
            "http://localhost:26657".to_string(),
            "https://lcd.terra.dev".to_string(),
            reqwest::Client::new(),
        );
        block_on(async { w.decode_tx(&TX_STR.to_string()).await.unwrap() });
    }
}
