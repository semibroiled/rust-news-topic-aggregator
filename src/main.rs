use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

use csv::Writer;
use std::fs::File;
use std::io::Write;
use tokio;

const NEWS_API_URL: &str = "https://newsapi.org/v2/everything";
const API_KEY: &str = "my key";

#[derive(Deserialize)]
struct NewsAPIResponse {
    articles: Vec<Article>, //List of Articles
}
#[derive(Deserialize)]
struct Article {
    title: String,
    url: String,
    published_at: String,
}

async fn search_news(query: &str) -> Result<Vec<Article>, Box<dyn Error>> {
    let client = Client::new(); // New HTTP Client

    let response: NewsAPIResponse = client
        .get(NEWS_API_URL) // GET Request to URL
        .query(&[
            ("q", query),
            ("apiKey", API_KEY),
            ("from", "2023-06-01"),
            ("sortBy", "relevancy"),
        ]) // Add query params
        .send() // Send Request
        .await? // Await for Response
        .json::<NewsAPIResponse>() // Deserialize JSON Response
        .await?; // Await Deserialization

    Ok(response.articles) // Return articles
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // Code Block

    // User Input for Topic
    println!("Enter a topic to search for news:");

    let mut topic: String = String::new(); // String because we dont know length of user input

    std::io::stdin().read_line(&mut topic)?; // Store input in topic

    let topic: &str = topic.trim(); // Remove leading and trailing whitespaces

    // Search for topic
    let articles: Vec<Article> = search_news(topic).await?;

    // Write CSV File
    let mut wrt: Writer<File> = Writer::from_path(format!("{}_news.csv", topic))?; // Create CSV Writer, write to path named after topic

    wrt.write_record(&["Title", "URL", "Published At"])?; // Write CSV Header

    for article in &articles {
        wrt.write_record(&[&article.title,&article.url, &article.published_at])?; // Write each articles details to CSV
    }

    wrt.flush()?; // Ensure all Data written to file

    // Top 15 Articles
    print
}
