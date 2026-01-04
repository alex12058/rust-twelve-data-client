# Twelve Data Rust Client

A Rust API client for [Twelve Data](https://twelvedata.com/) — comprehensive financial market data API with access to stocks, forex, ETFs, mutual funds, commodities, and cryptocurrencies across 50+ countries.

**This client features automatic builder pattern generation for all API operations**, making it ergonomic to work with endpoints that have many optional parameters.

## Features

- ✅ **Builder Pattern API**: Clean, chainable parameter builders instead of functions with 20+ parameters
- ✅ **Full API Coverage**: Complete access to all Twelve Data endpoints (time series, fundamentals, ETFs, technical indicators, etc.)
- ✅ **Type-Safe**: Strongly typed request/response models generated from OpenAPI spec
- ✅ **Async**: Built on `reqwest` with async/await support via `tokio`
- ✅ **Authentication**: API key authentication via Authorization header

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
twelve-data-client = "0.4"
tokio = { version = "1.49.0", features = ["full"] }
```

## Quick Start

```rust
use twelve_data_client::apis::{configuration, time_series_api};
use twelve_data_client::apis::time_series_api::GetTimeSeriesParams;
use twelve_data_client::models::GetTimeSeriesResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure authentication
    let mut config = configuration::Configuration::new();
    config.api_key = Some(configuration::ApiKey {
        prefix: Some("apikey".to_string()),
        key: std::env::var("TWELVE_DATA_API_KEY")?,
    });

    // Build parameters using builder pattern
    let params = GetTimeSeriesParams::builder()
        .symbol("AAPL")
        .interval("1day")
        .outputsize(10)
        .build();

    // Make API call and handle response
    let response = time_series_api::get_time_series(&config, params).await?;
    
    match response {
        GetTimeSeriesResponse::TimeSeries(data) => {
            println!("Symbol: {:?}", data.meta.as_ref().unwrap().symbol);
            println!("Data points: {}", data.values.as_ref().unwrap().len());
        }
        GetTimeSeriesResponse::ApiError(err) => {
            eprintln!("API Error: {}", err.message.unwrap_or_default());
        }
        GetTimeSeriesResponse::Text(csv) => {
            println!("CSV Response:\n{}", csv);
        }
    }
    
    Ok(())
}
```

## Authentication

Get your API key from the [Twelve Data dashboard](https://twelvedata.com/account/api-keys).

Set it as an environment variable:
```bash
export TWELVE_DATA_API_KEY="your_api_key_here"
```

Or configure it directly:
```rust
config.api_key = Some(configuration::ApiKey {
    prefix: Some("apikey".to_string()),
    key: "your_api_key_here".to_string(),
});
```

## Response Types

Most endpoints return an enum with variants for different response scenarios:

- **Data variant**: Contains the actual response data (e.g., `TimeSeries`)
- **`ApiError` variant**: Contains error information when the API returns an error
- **`Text` variant**: Contains raw text (CSV) response when using `format` parameter

This provides type-safe error handling and format flexibility without needing to manually parse errors.

## Available APIs

This client provides access to all Twelve Data endpoints organized by module:

- **`time_series_api`** - Historical OHLCV data
- **`fundamentals_api`** - Company financials, balance sheets, income statements
- **`reference_data_api`** - Symbols, exchanges, currencies
- **`technical_indicator_api`** - Technical analysis indicators
- **`market_data_api`** - Real-time quotes and prices
- **`etfs_api`** - ETF-specific data
- **`mutual_funds_api`** - Mutual fund data
- **`analysis_api`** - Analysis endpoints
- **`regulatory_api`** - Regulatory filings
- **`advanced_api`** - Advanced data endpoints

## Code Generation

This client is auto-generated from the [Twelve Data OpenAPI spec](https://api.twelvedata.com/doc/swagger/openapi.json) using [openapi-generator-rust-builders](https://github.com/alex12058/openapi-generator-rust-builders), which adds automatic builder pattern support.

### Requirements

Before regenerating the client, ensure you have:
- **Java 21 or later** - Required to run the OpenAPI Generator
- **Python 3.6+** - Required for spec transformation scripts
- **Maven** - Included with the generator (uses `./mvnw` wrapper)

### Regenerating the Client

Run the provided script to fetch the latest spec, apply transformations, and regenerate:

```bash
bash regenerate.sh
```

This script:
1. Fetches the latest OpenAPI spec from Twelve Data
2. Removes the `queryParameter` security scheme (best practice: API keys should only be in headers, not query parameters)
3. Ensures all 200 responses include `ApiError` variant for error handling
4. Adds `text/csv` response type to endpoints supporting the `format` parameter
5. Renames response schemas to cleaner names (e.g., `GetTimeSeries_200_response` → `time_series_data`)
6. Regenerates the Rust client with the modified spec
7. Formats the generated code

### First-Time Setup

If this is your first time generating the client, build the generator first:

```bash
cd openapi-generator-rust-builders
./mvnw clean package -DskipTests
cd ..
bash regenerate.sh
```

## Examples

See the `tests/` directory for complete working examples:

- [`time_series_builder.rs`](tests/time_series_builder.rs) - Fetching historical time series data with builder pattern, including JSON, error, and CSV response handling

Run the tests:
```bash
export TWELVE_DATA_API_KEY="your_api_key"
cargo test --test time_series_builder
```

## Documentation

- [Twelve Data API Documentation](https://twelvedata.com/docs)
- [API Reference](https://api.twelvedata.com/doc/swagger/)
- [Get API Key](https://twelvedata.com/account/api-keys)

## License

This client library is licensed under the Unlicense.

The Twelve Data API is a commercial service. See [Twelve Data pricing](https://twelvedata.com/pricing) for details.

## Links

- [Twelve Data Website](https://twelvedata.com/)
- [Twelve Data GitHub](https://github.com/twelvedata)
- [OpenAPI Generator Fork (with Rust Builders)](https://github.com/alex12058/openapi-generator-rust-builders)
- [Base OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator)
