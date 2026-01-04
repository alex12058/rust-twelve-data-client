use twelve_data_client::apis::{
    configuration::{ApiKey, Configuration},
    time_series_api,
};
use twelve_data_client::models::GetTimeSeries200ResponseEnum;

fn get_config() -> Configuration {
    let api_key = std::env::var("TWELVE_DATA_API_KEY")
        .expect("TWELVE_DATA_API_KEY environment variable not set");

    Configuration {
        base_path: "https://api.twelvedata.com".to_string(),
        api_key: Some(ApiKey {
            prefix: Some("apikey".to_string()),
            key: api_key,
        }),
        ..Default::default()
    }
}

#[tokio::test]
async fn test_time_series_json_response() {
    let config = get_config();
    let params = time_series_api::GetTimeSeriesParams::builder()
        .interval("1day")
        .symbol("AAPL")
        .outputsize(5)
        .build();

    let response = time_series_api::get_time_series(&config, params)
        .await
        .expect("API call failed");

    match response {
        GetTimeSeries200ResponseEnum::GetTimeSeries200Response(res) => {
            assert_eq!(res.status, Some("ok".to_string()));
            assert!(res.meta.is_some());
            assert!(res.values.is_some());
            assert_eq!(res.values.as_ref().unwrap().len(), 5);
        }
        _ => panic!("Expected JSON response"),
    }
}

#[tokio::test]
async fn test_time_series_error_response() {
    let config = get_config();
    let params = time_series_api::GetTimeSeriesParams::builder()
        .interval("0.99min") // Invalid interval
        .symbol("AAPL")
        .build();

    let response = time_series_api::get_time_series(&config, params)
        .await
        .expect("API call failed");

    match response {
        GetTimeSeries200ResponseEnum::ApiError(err) => {
            assert_eq!(err.code, 400);
            assert!(err.message.contains("Invalid **interval** provided"));
        }
        _ => panic!("Expected error response"),
    }
}

#[tokio::test]
async fn test_time_series_csv_response() {
    let config = get_config();
    let params = time_series_api::GetTimeSeriesParams::builder()
        .interval("1day")
        .symbol("AAPL")
        .outputsize(5)
        .format("CSV")
        .build();

    let response = time_series_api::get_time_series(&config, params)
        .await
        .expect("API call failed");

    match response {
        GetTimeSeries200ResponseEnum::Text(csv_data) => {
            let lines: Vec<&str> = csv_data.lines().collect();
            assert!(!lines.is_empty(), "CSV response should not be empty");
            // CSV should have header and data rows
            assert!(
                lines[0].contains("datetime") || lines[0].contains("open"),
                "CSV header should contain 'datetime' or 'open'"
            );
        }
        _ => panic!("Expected CSV text response, but got a different response type"),
    }
}
