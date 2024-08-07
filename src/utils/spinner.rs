use colored::*;
use crossterm::{
    cursor,
    style::{Color, SetForegroundColor},
    terminal, ExecutableCommand,
};
use futures::executor::block_on;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

pub struct Spinner {
    spin_chars: Vec<char>,
    interval: Duration,
    message: Option<String>,
    color: Option<Color>,
    stopped: Arc<Mutex<bool>>,
}

impl Spinner {
    // Constructor
    pub fn new(style: &str, interval_ms: Option<u64>) -> Self {
        let spin_chars: Vec<char> = match style {
            "dots" => vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            "line" => vec!['-', '\\', '|', '/'],
            "circle" => vec!['◐', '◓', '◑', '◒'],
            "arrows" => vec!['←', '↑', '→', '↓'],
            _ => vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'], //Default
        };

        let interval: u64 = match interval_ms {
            Some(interval_ms) => interval_ms,
            _ => 20,
        };

        Spinner {
            spin_chars,
            interval: Duration::from_millis(interval),
            message: None,
            color: None,
            stopped: Arc::new(Mutex::new(false)),
        }
    }

    // Custom Message
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self //TODO: Add Loading... by default if no message passed
    }

    // Custom Color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    // Simulate Async Task
    pub async fn task_sim(&self) {
        sleep(Duration::from_secs(5)).await; // Simulate 5 second task
    }

    /// Start the spinner asynchronously
    pub async fn start(&self) {
        let mut stdout = stdout();

        // Hide cursor
        if let Err(e) = stdout.execute(cursor::Hide) {
            eprintln!("Failed to hide cursor: {:?}", e);
        }

        let spin_chars = self.spin_chars.clone();
        let interval = self.interval;
        let message = self.message.clone().unwrap_or_else(|| "".to_string());
        let color = self.color;

        let stopped = self.stopped.clone();

        // Start spinner thread
        let spinner_handle = thread::spawn(move || {
            let mut i = 0;
            loop {
                if *stopped.lock().unwrap() {
                    break;
                }

                let spinner_char = spin_chars[i % spin_chars.len()];

                // Use a local stdout reference within the loop
                let mut local_stdout = std::io::stdout();

                // Set color if specified
                if let Some(color) = color {
                    if let Err(e) = local_stdout.execute(SetForegroundColor(color)) {
                        eprintln!("Failed to set color: {:?}", e);
                    }
                }

                print!("{} {}\r", spinner_char.to_string().bold(), &message);

                if let Err(e) = local_stdout.flush() {
                    eprintln!("Failed to flush stdout: {:?}", e);
                }
                std::thread::sleep(interval);

                i += 1;
            }
        });

        // Let the spinner run for some time (simulating async tasks)
        self.task_sim().await;

        // Stop the spinner
        self.stop();
        spinner_handle.join().unwrap();

        // Reset color and show cursor again
        if let Err(e) = stdout.execute(crossterm::style::ResetColor) {
            eprintln!("Failed to reset color: {:?}", e);
        }
        if let Err(e) = stdout.execute(cursor::Show) {
            eprintln!("Failed to show cursor: {:?}", e);
        }

        let msg = self.message.clone().unwrap();
        println!("{} {}", "✔".green(), msg);
    }


    // Stop Spinner
    pub fn stop(&self) {
        let mut stopped = self.stopped.lock().unwrap();
        *stopped = true;
    }
}
