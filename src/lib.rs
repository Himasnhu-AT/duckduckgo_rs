use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub description: String,
    pub url: String,
}

pub async fn search_duckduckgo(client: &Client, query: &str) -> Result<Vec<SearchResult>> {
    let encoded_query = urlencoding::encode(query);
    let url = format!("https://html.duckduckgo.com/html/?q={}", encoded_query);

    let response = client
        .get(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        )
        .send()
        .await?;

    // Check the response status
    if !response.status().is_success() {
        eprintln!("Failed to fetch search results: {}", response.status());
        return Ok(Vec::new());
    }

    let html = response.text().await?;
    // Print the HTML response for debugging
    println!("HTML Response: {}", html);

    let document = Html::parse_document(&html);

    let result_selector = Selector::parse(".result").unwrap();
    let title_selector = Selector::parse(".result__title").unwrap();
    let snippet_selector = Selector::parse(".result__snippet").unwrap();
    let url_selector = Selector::parse(".result__url").unwrap();

    let mut results = Vec::new();

    for element in document.select(&result_selector) {
        let title = element
            .select(&title_selector)
            .next()
            .and_then(|e| Some(e.text().collect::<String>().trim().to_string()))
            .unwrap_or_default();

        let description = element
            .select(&snippet_selector)
            .next()
            .and_then(|e| Some(e.text().collect::<String>().trim().to_string()))
            .unwrap_or_default();

        let mut url = element
            .select(&url_selector)
            .next()
            .and_then(|e| Some(e.text().collect::<String>().trim().to_string()))
            .unwrap_or_default();

        if !title.is_empty() && !description.is_empty() && !url.is_empty() {
            if !url.starts_with("http") {
                url = format!("https://{}", url);
            }

            results.push(SearchResult {
                title,
                description,
                url,
            });
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use tokio;

    #[tokio::test]
    async fn test_search_duckduckgo() {
        let http_client = Client::new();
        let query = "rust programming";

        match search_duckduckgo(&http_client, query).await {
            Ok(results) => {
                assert!(!results.is_empty(), "Results should not be empty");
                for result in results {
                    assert!(!result.title.is_empty(), "Title should not be empty");
                    assert!(!result.description.is_empty(), "Description should not be empty");
                    assert!(!result.url.is_empty(), "URL should not be empty");
                }
            }
            Err(e) => {
                panic!("Error occurred: {}", e);
            }
        }
    }
}
