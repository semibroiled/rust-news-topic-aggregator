// Import local modules
mod search_news;
use search_news::{call_api, ApiParams};

mod utils;
use utils::get_env::get_keys;

use std::{error::Error, fs};

// JSON Handling
use serde::Deserialize;

// For Logging and Debugging
use anyhow::{Context, Result};
use log::info;

// CSV Handling
use csv::Writer;

// File System Handling
use std::fs::File;
use std::path::Path;

use tokio;


const NEWS_API_URL: &str = "https://newsapi.org/v2/everything";

#[derive(Deserialize)]
struct NewsAPIResponse {
    articles: Vec<Article>, //List of Articles
}
#[derive(Deserialize)]
struct Article {
    title: String,
    url: String,
    #[serde(rename = "publishedAt")]
    published_at: String,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Init Logging Backend
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Starting application...");

    // Get API Token
    let api_key: String = get_keys("API_KEY").context("Failed to get API Key: {}")?;

    // Set Language
    let language: &str = "en";

    // Set destination folder
    let file_destination: &str = "history";

    // Code Block

    // User Input for Topic
    println!("Enter a topic to search for news:");

    let mut topic: String = String::new(); // String because we dont know length of user input

    std::io::stdin()
        .read_line(&mut topic)
        .context("Failed to read input")?; // Store input in topic

    let topic: &str = topic.trim(); // Remove leading and trailing whitespaces

    // Define Params for API Call
    let params = ApiParams {
        query: topic,
        url: NEWS_API_URL,
        api_key: &api_key,
        language,
        sort_by: "relevancy",
    };
    // Search for topic
    let api_response: String = call_api(params)
        .await
        .with_context(|| format!("Failed to fetch api response for query: {}", topic))?;

    // Deserialize JSON to Lists
    let news_api_response: NewsAPIResponse =
        serde_json::from_str(&api_response).context("Failed to deserialize response")?;
    let articles: Vec<Article> = news_api_response.articles;

    // Write CSV File

    // Define File Path
    let topic_alnum: String = topic.to_string().chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    let file_name: String = format!("{}/{}_{}.csv", file_destination, topic_alnum, language);
    let file_path: &Path = Path::new(&file_name);

    // Creat Dirs if they don't exist
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to creat parent directory: {:?}", parent))?;
    }

    let file: File = File::create(file_path)
        .with_context(|| format!("Failed to create file: {:?}", file_path))?;
    let mut wrt: Writer<File> = Writer::from_writer(file); // Create CSV Writer, write to path named after topic

    wrt.write_record(["#","Title", "URL", "Published At"])?; // Write CSV Header

    for (index, article) in articles.iter().enumerate() {
        wrt.write_record([index.to_string().as_str(),article.title.as_str(), article.url.as_str(), article.published_at.as_str()])?;
        // Write each articles details to CSV
    }

    wrt.flush()?; // Ensure all Data written to file

    // Top 15 Articles
    println!("Top 15 Article for '{}':", topic);

    for (index, article) in articles.iter().enumerate().take(15) {

        println!(
            "{}. {} : <{}> - [{}] ",
            index+1, article.title, article.url, article.published_at
        );
    }

    Ok(()) // Ensures main fcn returns Ok(())
}
