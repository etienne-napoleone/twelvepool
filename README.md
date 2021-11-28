# twelvepool

Watch for new txs in the mempool.

## Example

```rust
use twelvepool::Watcher;

#[tokio::main]
async fn main() {
    let mut receiver = Watcher::new(
        String::from("http://localhost:26657"),
        String::from("http://localhost:1317"),
        None,
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
