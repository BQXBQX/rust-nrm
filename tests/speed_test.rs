use rust_nrm::utils::speed_test::{SpeedTestResult, SpeedTester};

#[tokio::test]
async fn test_speed_test_result() {
    let result = SpeedTestResult {
        name: "npm".to_string(),
        url: "https://registry.npmjs.org/".to_string(),
        response_time: 0.1,
        is_success: true,
    };

    assert_eq!(result.name, "npm");
    assert_eq!(result.url, "https://registry.npmjs.org/");
    assert_eq!(result.response_time, 0.1);
    assert!(result.is_success);
}

#[tokio::test]
async fn test_speed_tester() {
    let registries = vec![
        ("npm".to_string(), "https://registry.npmjs.org/".to_string()),
        ("taobao".to_string(), "https://registry.npmmirror.com/".to_string()),
    ];

    let tester = SpeedTester::new();
    let results = tester.test_all(&registries).await;

    assert_eq!(results.len(), 2);
    for result in results {
        assert!(result.is_success); // All registries should be accessible
        assert!(result.response_time > 0.0); // Duration should be positive
    }
}

#[tokio::test]
async fn test_speed_test_timeout() {
    let registries = vec![
        ("invalid".to_string(), "https://invalid.registry.test".to_string()),
    ];

    let tester = SpeedTester::new();
    let results = tester.test_all(&registries).await;

    assert_eq!(results.len(), 1);
    assert!(!results[0].is_success); // Should fail for invalid registry
}
