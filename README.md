# twelvepool

Watch for new txs in a Terra node mempool.

## Example

```rust
use twelvepool::Watcher;

#[tokio::main]
async fn main() {
    let mut watcher = Watcher::new(
        String::from("http://localhost:26657"),  // RPC address
        String::from("http://localhost:1317"),   // LCD address
        None,                                    // Optional reqwest client
        None,                                    // Optional interval duration (default to 100ms)
    )
    .run();

    loop {
        if let Some(mempool_item) = watcher.recv().await {
            if mempool_item.tx.memo == "my memo" {
                println!("tx with our memo found (tx hash {})", mempool_item.tx_hash);
            }
        }
    }
}
```
