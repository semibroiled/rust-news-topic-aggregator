# Rust News Processing Application

This Rust application is designed to fetch, process, and display news articles. It consists of three main components:

1. **Main Application (`main.rs`)**
2. **News Search Module (`search_news.rs`)**
3. **Text Processing Module (`process_text.rs`)**

## Table of Contents

- [Rust News Processing Application](#rust-news-processing-application)
  - [Table of Contents](#table-of-contents)
  - [Project Description](#project-description)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Main Application (`main.rs`)](#main-application-mainrs)
    - [News Search Module (`search_news.rs`)](#news-search-module-search_newsrs)
    - [Text Processing Module (`process_text.rs`)](#text-processing-module-process_textrs)
  - [License](#license)

## Project Description

The Rust News Processing Application allows users to search for news articles based on specific criteria, process the retrieved text, and display the results in a user-friendly manner. It leverages Rust's performance and safety features to handle data efficiently.

## Features

- Search for news articles using predefined criteria.
- Process and analyze the text of the retrieved articles.
- Display the processed information in a structured format.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- [Rust](https://www.rust-lang.org/tools/install) (version 1.54.0 or later)
- Internet connection (for fetching news articles)

## Installation

To install and set up this project locally, follow these steps:

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/rust-news-processing.git
   cd rust-news-processing```

2. ***Build the Project*

    Ensure you have Rust installed. If not, install it from [here](https://www.rust-lang.org/tools/install).
    `cargo build --release`

## Usage

To run the application, execute the following command in your terminal:

```bash
cargo run --release```

The application will start and you will be prompted to enter search criteria for fetching news articles.

Enter `!exit` instead of a topic to exit application.

### Main Application (`main.rs`)

The `main.rs` file serves as the entry point for the application. It initializes the necessary modules and starts the workflow for searching and processing news articles.

### News Search Module (`search_news.rs`)

The `search_news.rs` file contains functions and logic for querying news sources and retrieving articles based on user input.

### Text Processing Module (`process_text.rs`)

The `process_text.rs` file includes functions for processing and analyzing the text of the retrieved news articles. This includes text cleaning, summarization, and other text-related operations.

This is currently not implemented due to constraints to running bert in silicon mac.


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.