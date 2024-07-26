// HTTP Request
use reqwest::Client;

// JSON Handling
use serde::Deserialize;

// Error Handling
use anyhow::{Context, Result};

// Logging
use log::info;

pub struct ApiParams<'a> {
    pub query: &'a str,
    pub url: &'a str,
    pub api_key: &'a str,
    pub language: &'a str,
    pub sort_by: &'a str,
}

#[derive(Deserialize)]
pub struct NewsAPIResponse {
    pub articles: Vec<Article>, //List of Articles
}
#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
    pub url: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
}

pub async fn call_api(params: ApiParams<'_>) -> Result<String> {
    info!("Initializing client...");
    let client: Client = Client::new(); // New HTTP Client

    info!("Running GET Request...");
    let response: String = client
        .get(params.url) // GET Request to URL
        .query(&[
            ("q", params.query),
            ("apiKey", params.api_key),
            ("sortBy", params.sort_by),
            ("language", params.language),
        ]) // Add query params
        .header("User-Agent", "news-topic-aggregator")
        .send() // Send Request
        .await // Await for Response
        .context("Failed to send request")?
        .text() // Deserialize JSON Response
        .await // Await Deserialization
        .context("Failed to convert response to text")?;

    println!("{}", response);

    Ok(response)
}
