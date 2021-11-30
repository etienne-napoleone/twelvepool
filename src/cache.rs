use std::collections::HashMap;
use std::time::Instant;

use crate::Tx;

type StampedValue = (Instant, Tx);

#[derive(Debug)]
enum Status {
    NotFound,
    Found,
    Expired,
}

#[derive(Clone, Debug)]
pub struct Cache {
    store: HashMap<String, StampedValue>,
    ttl_seconds: u64,
}

impl Cache {
    pub fn new(ttl_seconds: u64) -> Cache {
        Cache {
            store: HashMap::new(),
            ttl_seconds,
        }
    }

    pub fn set(&mut self, key: String, value: Tx) -> Option<Tx> {
        let stamped = (Instant::now(), value);
        self.store.insert(key.clone(), stamped).map(|(_, tx)| {
            log::trace!("inserted key {}", key);
            tx
        })
    }

    pub fn get(&mut self, key: &str) -> Option<&Tx> {
        let status = {
            let mut stamped = self.store.get_mut(key);
            if let Some(&mut (instant, _)) = stamped.as_mut() {
                if instant.elapsed().as_secs() < self.ttl_seconds {
                    Status::Found
                } else {
                    Status::Expired
                }
            } else {
                Status::NotFound
            }
        };
        match status {
            Status::NotFound => {
                log::trace!("key {} not found", key);
                None
            }
            Status::Found => self.store.get(key).map(|stamped| {
                log::trace!("key {} found", key);
                &stamped.1
            }),
            Status::Expired => {
                self.store.remove(key).unwrap();
                log::trace!("key {} has expired", key);
                None
            }
        }
    }

    pub fn clear_expired(&mut self) -> usize {
        let last_length = self.store.len();
        self.store
            .retain(|_, tx| tx.0.elapsed().as_secs() < self.ttl_seconds);
        let new_length = self.store.len();
        log::trace!("expired keys cleared ({} -> {})", last_length, new_length);
        last_length - new_length
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    use super::Cache;
    use crate::tx::{Fee, Tx};

    const MOCKED_TX: Tx = Tx {
        msg: vec![],
        fee: Fee {},
        signatures: vec![],
        memo: String::new(),
        timeout_height: String::new(),
    };

    #[test]
    fn set_item() {
        let mut cache = Cache::new(1000);
        cache.set("key".to_string(), MOCKED_TX);
        assert_eq!(cache.store.get("key").unwrap().1, MOCKED_TX);
    }

    #[test]
    fn get_item() {
        let mut cache = Cache::new(1000);
        let value = (Instant::now(), MOCKED_TX);
        cache.store.insert("key".to_string(), value);
        assert_eq!(cache.get("key").unwrap(), &MOCKED_TX);
    }

    #[test]
    fn get_not_found_item() {
        let mut cache = Cache::new(0);
        assert!(cache.get("key").is_none());
    }

    #[test]
    fn get_expired_item() {
        let mut cache = Cache::new(0);
        let value = (Instant::now(), MOCKED_TX);
        cache.store.insert("key".to_string(), value);
        assert!(cache.get("key").is_none());
    }

    #[test]
    fn clear_expired_items() {
        let mut cache = Cache::new(1);
        let value = (Instant::now(), MOCKED_TX);
        cache.store.insert("key".to_string(), value);
        cache.clear_expired();
        assert!(cache.store.contains_key("key"));
        sleep(Duration::from_secs(2));
        assert!(cache.store.contains_key("key"));
        cache.clear_expired();
        assert!(!cache.store.contains_key("key"));
    }
}
