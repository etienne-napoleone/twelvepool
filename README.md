# twelvepool

Watch for new txs in a Terra node mempool.

## Example

```rust
use twelvepool::Watcher;

#[tokio::main]
async fn main() {
    let mut receiver = Watcher::new(
        String::from("http://localhost:26657"),  // RPC address
        String::from("http://localhost:1317"),   // LCD address
        None,                                    // Optional reqwest client
        None,                                    // Optional interval duration (default to 100ms)
    )
    .run();

    loop {
        if let Some(tx) = receiver.recv().await {
            if tx.memo == "my memo" {
                println!("tx found");
            }
        }
    }
}
```
