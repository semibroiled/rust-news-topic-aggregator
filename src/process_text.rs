// NLP Processing
use nlprule::{Rules, Tokenizer};
use rust_bert::pipelines::summarization::SummarizationModel;

// Import Type
use crate::search_news::Article;

// Error Handling
use anyhow::{Context, Result};

// Logging
use log::info;

// pub fn infer_named_entities(text: &sentence) -> Result<Vec<String>> {
//     // Load NLP rules for NER
//     let rules = Rules::new("en").context("Failed to initialize Rules")?;

//     // Tokenize text and apply rules
//     let tokenizer = Tokenizer::new("en");
//     let tokens = tokenizer.tokenize(text.as_str());
//     let doc = rules.apply(tokens.as_slice());

//     // Extract Named Entities
//     let named_entities: Vec<String> = doc.matches.iter().map(|m| m.message.clone()).collect();

//     Ok(named_entities)
// }

pub async fn generate_summary(articles: &[Article]) -> Result<String> {
    let summarization_model = SummarizationModel::new(Default::default())
        .context("Failed to initialize Summarization Model.")?;

    let mut all_text = String::new();

    for article in articles.iter().take(15) {
        all_text.push_str(format!("{}.", article.title).as_str());
        all_text.push_str(".");
    }

    info!("Attempting summarisation...");
    let summary = summarization_model.summarize([all_text.as_str()]).join(" ");
    Ok(summary)
}
