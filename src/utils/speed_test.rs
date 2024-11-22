use colored::Colorize;
use reqwest;
use std::time::Instant;
use super::Logger;

#[derive(Debug)]
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
            client: reqwest::Client::new(),
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
        let mut results = Vec::new();

        Logger::info("Testing registry speeds...");

        for (name, url) in registries {
            let result = self.test_registry(name, url).await;
            results.push(result);
        }

        // Sort by response time
        results.sort_by(|a, b| a.response_time.partial_cmp(&b.response_time).unwrap());

        // Print results
        Logger::list("Registry Speed Test Results:");
        
        for (index, result) in results.iter().enumerate() {
            let status = if result.is_success {
                "✓".green()
            } else {
                "✗".red()
            };

            let time_str = Self::format_time(result.response_time);
            let time_display = match index {
                0 => time_str.green().bold(),   // Fastest
                1 => time_str.yellow(),         // Second
                2 => time_str.blue(),           // Third
                _ => time_str.normal(),
            };

            println!(
                "{} {} {} {} {}",
                format!("[{}]", index + 1).bold(),
                status,
                result.name.bold(),
                "->".dimmed(),
                time_display
            );
        }

        if let Some(fastest) = results.first() {
            if fastest.is_success {
                Logger::success(&format!(
                    "Fastest registry is {} ({})",
                    fastest.name.bold(),
                    Self::format_time(fastest.response_time)
                ));
            }
        }

        results
    }
}
