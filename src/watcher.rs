use std::collections::HashMap;
use std::vec;

use crate::cache::Cache;
use crate::terra::Terra;
use crate::tx::Tx;

use reqwest::Client;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tokio_stream::{self as stream, StreamExt};

const CHANNEL_CAPACITY: usize = 20;
const INTERVAL: Duration = Duration::from_secs(2);

#[derive(Debug)]
pub struct Watcher {
    terra: Terra,
    new_txs: Vec<Tx>,
    cached_txs: HashMap<String, Tx>,
    handles: Vec<JoinHandle<()>>,
    cache: Cache,
}

impl Watcher {
    pub fn new(rpc_url: String, lcd_url: String, http_client: Option<Client>) -> Watcher {
        Watcher {
            terra: Terra::new(rpc_url, lcd_url, http_client.unwrap_or(Client::new())),
            new_txs: vec![],
            cached_txs: HashMap::new(),
            handles: vec![],
            cache: Cache::new(30),
        }
    }

    pub async fn receive(mut self) -> broadcast::Receiver<Tx> {
        let (sender, receiver) = broadcast::channel(CHANNEL_CAPACITY);

        self.handles.push(tokio::spawn(async move {
            loop {
                match self.terra.get_unconfirmed_txs().await {
                    Ok(tx_strings) => {
                        let mut stream = stream::iter(tx_strings);
                        while let Some(tx_string) = stream.next().await {
                            match self.terra.get_tx_hash(&tx_string).await {
                                Ok(hash) => match self.cache.get(&hash) {
                                    Some(_) => log::trace!("tx {} already sent", hash),
                                    None => match self.terra.decode_tx(&tx_string).await {
                                        Ok(tx) => match sender.send(tx.clone()) {
                                            Ok(_) => {
                                                log::debug!("sent tx {}", hash);
                                                self.cache.set(hash, tx);
                                            }
                                            Err(err) => {
                                                log::error!("couldn't send tx {}: {}", hash, err)
                                            }
                                        },
                                        Err(err) => log::error!("could not decode tx: {}", err),
                                    },
                                },
                                Err(err) => log::error!("couldn't decode tx hash: {}", err),
                            }
                        }
                    }
                    Err(err) => log::error!("couldn't get txs: {}", err),
                }
                self.cache.clear_expired();
                sleep(INTERVAL).await;
            }
        }));

        receiver
    }
}
