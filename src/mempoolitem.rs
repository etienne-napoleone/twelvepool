use std::time::SystemTime;

use crate::tx::Tx;

pub struct MempoolItem {
    pub timestamp: SystemTime,
    pub tx: Tx,
    pub tx_hash: String,
}

impl MempoolItem {
    pub fn new(tx: Tx, tx_hash: String) -> MempoolItem {
        MempoolItem {
            timestamp: SystemTime::now(),
            tx,
            tx_hash,
        }
    }
}
