use super::Logger;
use colored::Colorize;
use reqwest;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::task;

#[derive(Debug, Clone)]
pub struct SpeedTestResult {
    pub name: String,
    pub url: String,
    pub response_time: f64,
    pub is_success: bool,
}

#[derive(Clone)]
pub struct SpeedTester {
    client: reqwest::Client,
}

impl SpeedTester {
    pub fn new() -> Self {
        SpeedTester {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5)) // 5 seconds timeout
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    pub async fn test_registry(&self, name: &str, url: &str) -> SpeedTestResult {
        let start = Instant::now();
        let result = self.client.get(url).send().await;
        let elapsed = start.elapsed().as_secs_f64();

        SpeedTestResult {
            name: name.to_string(),
            url: url.to_string(),
            response_time: elapsed,
            is_success: result.is_ok(),
        }
    }

    fn format_time(seconds: f64) -> String {
        format!("{:.0}ms", seconds * 1000.0)
    }

    pub async fn test_all(&self, registries: &[(String, String)]) -> Vec<SpeedTestResult> {
        Logger::info("Testing registry speeds...");
        println!(); // Add a blank line for better readability

        // Create a shared vector for results
        let results = Arc::new(Mutex::new(Vec::new()));

        // Create an atomic counter for ranking
        let speed_rank = Arc::new(AtomicUsize::new(1));

        // Create a vector of tasks for parallel execution
        let mut handles: Vec<task::JoinHandle<()>> = Vec::new();
        for (name, url) in registries.iter() {
            let name = name.clone();
            let url = url.clone();
            let client = self.clone();
            let results = Arc::clone(&results);
            let speed_rank = Arc::clone(&speed_rank);

            let handle = task::spawn(async move {
                let test_result = client.test_registry(&name, &url).await;

                let status = if test_result.is_success {
                    "✓".green()
                } else {
                    "✗".red()
                };

                let time_str = Self::format_time(test_result.response_time);
                let time_display = time_str.normal();
                let current_rank = speed_rank.fetch_add(1, Ordering::Relaxed);
                let rank = format!("#{}", current_rank).bold();

                println!(
                    "{} {} {} {} {}",
                    rank,
                    status,
                    test_result.name.bold(),
                    "->".dimmed(),
                    time_display
                );

                // Store the result
                let mut results = results.lock().await;
                results.push(test_result);
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let _ = handle.await;
        }

        // Get all results
        let final_results = results.lock().await.clone();

        // Find the fastest successful registry
        let fastest = final_results
            .iter()
            .filter(|r| r.is_success)
            .min_by(|a, b| a.response_time.partial_cmp(&b.response_time).unwrap());

        // Show the fastest registry
        if let Some(fastest) = fastest {
            println!(); // Add a blank line
            Logger::success(&format!(
                "Fastest registry is {} ({})",
                fastest.name.bold(),
                Self::format_time(fastest.response_time)
            ));
        }

        final_results
    }
}
