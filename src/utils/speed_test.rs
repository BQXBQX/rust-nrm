use super::Logger;
use colored::Colorize;
use reqwest;
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

        let registry_count = registries.len();
        // Store initial positions for each registry
        let positions: Arc<Mutex<Vec<(String, usize)>>> = Arc::new(Mutex::new(
            registries
                .iter()
                .enumerate()
                .map(|(i, (name, _))| (name.clone(), i))
                .collect(),
        ));

        // Print the initial list of registries being tested
        for (_index, (name, _)) in registries.iter().enumerate() {
            println!(
                "{} {} {}",
                "⋯".dimmed(), // Loading indicator
                name.bold(),
                "->".dimmed(),
            );
        }

        // Create a shared vector for results
        let results = Arc::new(Mutex::new(Vec::new()));

        // Create a vector of tasks for parallel execution
        let mut handles = Vec::new();
        for (name, url) in registries.iter() {
            let name = name.clone();
            let url = url.clone();
            let client = self.client.clone();
            let results = Arc::clone(&results);
            let positions = Arc::clone(&positions);

            let handle = task::spawn(async move {
                let start = Instant::now();
                let result = client.get(&url).send().await;
                let elapsed = start.elapsed().as_secs_f64();

                let test_result = SpeedTestResult {
                    name: name.clone(),
                    url,
                    response_time: elapsed,
                    is_success: result.is_ok(),
                };

                // Get the original position of this registry
                let pos = {
                    let positions = positions.lock().await;
                    positions
                        .iter()
                        .find(|(n, _)| n == &name)
                        .map(|(_, i)| *i)
                        .unwrap_or(0)
                };

                // Move cursor to the correct position and update the result
                print!("\x1B[{}A", registry_count - pos);

                let status = if test_result.is_success {
                    "✓".green()
                } else {
                    "✗".red()
                };

                let time_str = Self::format_time(test_result.response_time);
                let time_display = time_str.normal();

                println!(
                    "\x1B[2K{} {} {} {}", // \x1B[2K clears the entire line
                    status,
                    test_result.name.bold(),
                    "->".dimmed(),
                    time_display
                );

                // Move cursor back down
                print!("\x1B[{}B", registry_count - pos);

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
