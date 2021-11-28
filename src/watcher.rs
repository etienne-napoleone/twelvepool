use std::collections::HashMap;
use std::vec;

use crate::cache::Cache;
use crate::terra::Terra;
use crate::tx::Tx;

use futures::{stream, StreamExt};
use reqwest::Client;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

const INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub struct Watcher {
    terra: Terra,
    new_txs: Vec<Tx>,
    cached_txs: HashMap<String, Tx>,
    cache: Cache,
}

impl Watcher {
    pub fn new(rpc_url: String, lcd_url: String, http_client: Option<Client>) -> Watcher {
        Watcher {
            terra: Terra::new(rpc_url, lcd_url, http_client.unwrap_or(Client::new())),
            new_txs: vec![],
            cached_txs: HashMap::new(),
            cache: Cache::new(30),
        }
    }

    pub async fn run(mut self) -> mpsc::UnboundedReceiver<Tx> {
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            loop {
                match self.terra.get_unconfirmed_txs().await {
                    Ok(tx_strings) => {
                        let mut raw_txs = self.get_tx_hashes(tx_strings).await;
                        raw_txs.retain(|tx_hash, _| {
                            if self.cache.get(&tx_hash).is_none() {
                                true
                            } else {
                                log::debug!("tx {} already sent", tx_hash);
                                false
                            }
                        });

                        let txs = self.get_decoded_txs(raw_txs).await;
                        txs.iter()
                            .for_each(|(tx_hash, tx)| match sender.send(tx.clone()) {
                                Ok(_) => {
                                    log::info!("new tx {}", tx_hash);
                                    self.cache.set(tx_hash.clone(), tx.clone());
                                }
                                Err(err) => {
                                    log::error!("couldn't send tx {}: {}", tx_hash, err)
                                }
                            });
                    }
                    Err(err) => log::error!("couldn't get unconfirmed txs: {}", err),
                }
                self.cache.clear_expired();
                sleep(INTERVAL).await;
            }
        });

        receiver
    }

    async fn get_tx_hashes(&self, tx_strings: Vec<String>) -> HashMap<String, String> {
        let mut raw_txs: HashMap<String, String> = HashMap::new();

        let items: Vec<Option<(String, String)>> = stream::iter(tx_strings)
            .map(|tx_string| async move {
                if let Ok(tx_hash) = self.terra.get_tx_hash(&tx_string).await {
                    Some((tx_hash, tx_string))
                } else {
                    None
                }
            })
            .buffer_unordered(usize::MAX)
            .collect()
            .await;

        items.into_iter().for_each(|item| {
            if item.is_some() {
                let (tx_hash, tx_string) = item.unwrap();
                raw_txs.insert(tx_hash, tx_string);
            }
        });

        raw_txs
    }

    async fn get_decoded_txs(&self, raw_txs: HashMap<String, String>) -> HashMap<String, Tx> {
        let mut txs: HashMap<String, Tx> = HashMap::new();
        let items: Vec<Option<(String, Tx)>> = stream::iter(raw_txs)
            .map(|(tx_hash, tx_string)| async move {
                if let Ok(tx) = self.terra.decode_tx(&tx_string).await {
                    Some((tx_hash, tx))
                } else {
                    None
                }
            })
            .buffer_unordered(usize::MAX)
            .collect()
            .await;

        items.into_iter().for_each(|item| {
            if item.is_some() {
                let (tx_hash, tx) = item.unwrap();
                txs.insert(tx_hash, tx);
            }
        });

        txs
    }
}
