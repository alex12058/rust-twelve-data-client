use twelve_data_client::{
    apis::{configuration::{Configuration, ApiKey}, time_series_api},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("TWELVE_DATA_API_KEY")
        .expect("TWELVE_DATA_API_KEY environment variable not set");

    // Configure the client with API key using Authorization header
    let config = Configuration {
        base_path: "https://api.twelvedata.com".to_string(),
        api_key: Some(ApiKey {
            prefix: Some("apikey".to_string()),
            key: api_key,
        }),
        ..Default::default()
    };

    // Use builder pattern to construct parameters
    let params = time_series_api::GetTimeSeriesParams::builder()
        .interval("1day")
        .symbol("AAPL")
        .outputsize(10)
        .build();

    // Make the API call
    println!("Fetching time series data for AAPL...");
    
    match time_series_api::get_time_series(&config, params).await {
        Ok(response) => {
            println!("Success!");
            println!("Status: {:?}", response.status);
            
            if let Some(meta) = response.meta {
                println!("Meta: {:#?}", meta);
            }
            
            if let Some(values) = response.values {
                println!("Number of data points: {}", values.len());
                if !values.is_empty() {
                    println!("First data point: {:#?}", values[0]);
                }
            }
        }
        Err(e) => {
            println!("Error occurred:");
            println!("{:#?}", e);
        }
    }

    Ok(())
}
