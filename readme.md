# duckduckgo_rs

`duckduckgo_rs` is a Rust library that provides an interface to interact with the DuckDuckGo search engine. This library allows you to perform web searches and retrieve search results programmatically.

## Features

- Perform web searches using DuckDuckGo.
- Retrieve search results including title, description, and URL.
- Easy to use with asynchronous support.

## Installation

Add `duckduckgo_rs` to your `Cargo.toml`:

```toml
[dependencies]
duckduckgo_rs = "0.0.1"
```

## Usage

### Example

Here's an example of how to use the `duckduckgo_rs` library to perform a search:

```rust
use anyhow::Result;
use duckduckgo_rs::search_duckduckgo;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client = Client::new();
    let query = "rust programming"; // Example query

    match search_duckduckgo(&http_client, query).await {
        Ok(results) => {
            for result in results {
                println!("Title: {}", result.title);
                println!("Description: {}", result.description);
                println!("URL: {}", result.url);
                println!();
            }
        }
        Err(e) => {
            eprintln!("Error occurred: {}", e);
        }
    }

    Ok(())
}
```

### Running the Example

To run the example, use the following command:

```bash
cargo run --example main
```

### Testing

To run the tests, use the following command:

```bash
cargo test
```

### Documentation

To generate and open the documentation, use the following command:

```bash
cargo doc --open --release
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the terms of the BSD 3-Clause License.
