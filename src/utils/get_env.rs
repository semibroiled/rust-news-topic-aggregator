// ENV Handling
use dotenv::dotenv;
use std::env;

// Logging
use log::info;

// Error Handling
use anyhow::{Context, Result};

/// Get ENV Variables
///
/// # Arguments
///
/// * `key_name` - String slice, name of the key.
///
/// # Return
///
/// * `Result<String>` - Environment variable value.
///
/// # Examples
///
/// ```rust
/// let key_value = get_keys("MY_ENV_KEY").expect("Failed to get key");
/// println!("The value is: {}", key_value);
/// ```
pub fn get_keys(key_name: &str) -> Result<String> {
    // Load environment variables from .env
    info!("Loading Envrionment Variables...");
    dotenv().ok();

    // Retrieve Keys
    info!("Getting {}...", key_name);
    let key: String = env::var(key_name).context(format!("Failed to get key: {}", key_name))?;

    // Assert key is not empty
    assert!(!key.is_empty(), "Key is empty");

    Ok(key)
}