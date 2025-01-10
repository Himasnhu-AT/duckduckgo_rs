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
