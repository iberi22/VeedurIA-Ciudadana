# Socrata SDK (Rust)

A high-performance, asynchronous Rust client for the Socrata Open Data API (SODA).

## Features
- **Async/Await**: Built on top of `reqwest` and `tokio`.
- **Generic**: Works with any data structure that implements `serde::Deserialize`.
- **Type-Safe**: Leverage Rust's type system to ensure data integrity.
- **Minimal Dependencies**: Optimized for performance and small binary size.

## Usage

```rust
use socrata_sdk::SocrataClient;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyData {
    id: String,
    // ...
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SocrataClient::new("https://www.datos.gov.co", Some("MY_APP_TOKEN".to_string()));
    let results: Vec<MyData> = client.fetch("abcd-1234", 10, 0, None, None).await?;
    println!("{:?}", results);
    Ok(())
}
```

## License
MIT
