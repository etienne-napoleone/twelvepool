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
        self.store.insert(key, stamped).map(|(_, v)| v)
    }

    pub fn get(&mut self, key: &str) -> Option<&Tx> {
        let status = {
            let mut val = self.store.get_mut(key);
            if let Some(&mut (instant, _)) = val.as_mut() {
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
            Status::NotFound => None,
            Status::Found => self.store.get(key).map(|stamped| &stamped.1),
            Status::Expired => {
                self.store.remove(key).unwrap();
                None
            }
        }
    }

    pub fn clear_expired(&mut self) {
        self.store
            .retain(|_, v| v.0.elapsed().as_secs() < self.ttl_seconds);
    }
}

#[cfg(test)]
mod tests {
    use super::Cache;
    use crate::tx::{Fee, Tx};
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

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
