use std::collections::HashMap;
use std::vec;

pub use crate::tx::Tx;

use base64::decode;
use maplit::hashmap;
use reqwest::Client;
use sha2::{Digest, Sha256};
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

mod errors;
mod responses;
mod tx;

const CHANNEL_CAPACITY: usize = 20;
const INTERVAL: Duration = Duration::from_secs(2);

#[derive(Debug)]
pub struct Watcher {
    rpc_url: String,
    lcd_url: String,
    http_client: Client,
    new_txs: Vec<Tx>,
    cached_txs: HashMap<String, Tx>,
    handles: Vec<JoinHandle<()>>,
}

impl Watcher {
    pub fn new(rpc_url: String, lcd_url: String, http_client: Option<Client>) -> Watcher {
        Watcher {
            rpc_url,
            lcd_url,
            http_client: http_client.unwrap_or(Client::new()),
            new_txs: vec![],
            cached_txs: HashMap::new(),
            handles: vec![],
        }
    }

    pub async fn receive(&mut self) -> broadcast::Receiver<u32> {
        let (tx, rx) = broadcast::channel(CHANNEL_CAPACITY);

        self.handles.push(tokio::spawn(async move {
            loop {
                let number: u32 = 12;
                tx.send(number).unwrap();
                sleep(INTERVAL).await;
            }
        }));

        rx
    }

    async fn decode_tx(&self, tx_str: &String) -> Result<Tx, errors::RequestError> {
        let res = self
            .http_client
            .post(format!("{}/txs/decode", self.lcd_url))
            .json(&hashmap! { "tx" => tx_str })
            .send()
            .await?
            .json::<responses::DecodeTxResponse>()
            .await?;
        Ok(res.result)
    }

    async fn get_tx_hash(&self, tx_str: &String) -> Result<String, errors::TxHashDecodeError> {
        // create a Sha256 object
        let mut hasher = Sha256::new();
        hasher.update(decode(tx_str)?);
        let tx_bytes = hasher.finalize();
        Ok(format!("{:X}", tx_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::Watcher;
    use tokio_test::block_on;

    const TX_STR: &str = "Cp0BCo4BChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEm4KLHRlcnJhMXBqMmpwZ3l1Mmo4YW1nNXBkZnZqYzhkdjNnMHpkazcyaHpxZXdyEix0ZXJyYTE0bDQ2anJkZ2RoYXc0Y2VqdWt4NTBuZHAwaHNzOTVla3Qya2ZtdxoQCgV1bHVuYRIHMjEwOTk5NRIKMTg5NTQyMjk1MRJoClEKRgofL2Nvc21vcy5jcnlwdG8uc2VjcDI1NmsxLlB1YktleRIjCiEDmsiXXHtxyCbFAeeOfN7j4Ur4Z45hLi9zuNQ53j7mZ0ASBAoCCAEYiAgSEwoNCgV1bHVuYRIEMTA0ORDl0gUaQPT/oKGfhP5eHLYttAspV675WAshoWqPWe91x7VEy7/pWTmwmcsvxw4zYRrTO5Zh3mvaoO/MNOs/uJHhQLn2im0=";
    const TX_HASH: &str = "B884E37E0FF71E9FC81ACE90A54E85A7F5644FCB9BF50BBB5BA52835222DA9D1";

    #[test]
    fn tx_hash_is_correctly_computed() {
        let w = Watcher::new("test".to_string(), "test".to_string(), None);
        block_on(async {
            assert_eq!(w.get_tx_hash(&TX_STR.to_string()).await.unwrap(), TX_HASH);
        });
    }

    #[test]
    fn tx_str_is_correctly_decoded() {
        let w = Watcher::new(
            "http://localhost:26657".to_string(),
            "https://lcd.terra.dev".to_string(),
            None,
        );
        block_on(async { w.decode_tx(&TX_STR.to_string()).await.unwrap() });
    }
}
