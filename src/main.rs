use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

use csv::Writer;
use std::fs::File;
use tokio;

use dotenv::dotenv;
use std::env;

fn get_keys(key_name: &str) -> Result<String, Box<dyn Error>> {
    // Load environment variables from .env
    dotenv().ok();

    // Get Keys
    let key: String = env::var(key_name)?;

    Ok(key)
}

const NEWS_API_URL: &str = "https://newsapi.org/v2/everything";

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

async fn search_news(query: &str, api_key: &str) -> Result<Vec<Article>, Box<dyn Error>> {
    let client = Client::new(); // New HTTP Client

    let response: String = client
        .get(NEWS_API_URL) // GET Request to URL
        .query(&[
            ("q", query),
            ("apiKey", api_key),
            ("sortBy", "relevancy"),
        ]) // Add query params
        .header("User-Agent", "news-topic-aggregator")
        .send() // Send Request
        .await? // Await for Response
        .text() // Deserialize JSON Response
        .await?; // Await Deserialization
        
    println!("{}", response);
    let news_api_response: NewsAPIResponse = serde_json::from_str(&response)?;
    Ok(news_api_response.articles)
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // Get API Token
    let api_key: String = get_keys("API_KEY")?;

    // Code Block

    // User Input for Topic
    println!("Enter a topic to search for news:");

    let mut topic: String = String::new(); // String because we dont know length of user input

    std::io::stdin().read_line(&mut topic)?; // Store input in topic

    let topic: &str = topic.trim(); // Remove leading and trailing whitespaces

    // Search for topic
    let articles: Vec<Article> = search_news(topic, &api_key).await?;

    // Write CSV File
    let mut wrt: Writer<File> = Writer::from_path(format!("{}_news.csv", topic))?; // Create CSV Writer, write to path named after topic

    wrt.write_record(["Title", "URL", "Published At"])?; // Write CSV Header

    for article in &articles {
        wrt.write_record([&article.title, &article.url, &article.published_at])?;
        // Write each articles details to CSV
    }

    wrt.flush()?; // Ensure all Data written to file

    // Top 15 Articles
    println!("Top 15 Article for '{}':", topic);

    for article in articles.iter().take(15) {
        println!(
            "{} - {} - {} ",
            article.title, article.url, article.published_at
        );
    }

    Ok(()) // Ensures main fcn returns Ok(())
}
