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
twelve-data-client = "0.2"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use twelve_data_client::apis::{configuration, time_series_api};
use twelve_data_client::apis::time_series_api::GetTimeSeriesParams;

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

    // Make API call
    let response = time_series_api::get_time_series(&config, params).await?;
    
    println!("Symbol: {:?}", response.meta.unwrap().symbol);
    println!("Data points: {}", response.values.unwrap().len());
    
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

## Builder Pattern Example

Instead of this (many individual parameters):
```rust
// Old style - difficult to use
get_time_series(
    &config,
    "AAPL",
    "1day",
    None, None, None, None, None, 
    Some(10), 
    None, None, None, None, None, None, None, None, None
).await?
```

You get this (clean builder pattern):
```rust
// New style - ergonomic and clear
let params = GetTimeSeriesParams::builder()
    .symbol("AAPL")
    .interval("1day")
    .outputsize(10)
    .build();

get_time_series(&config, params).await?
```

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

This client was automatically generated from the [Twelve Data OpenAPI specification](https://api.twelvedata.com/doc/swagger/openapi.json) using a modified version of [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator).

### Generation Process

The client was generated using [openapi-generator-rust-builders](https://github.com/alex12058/openapi-generator-rust-builders), a fork of OpenAPI Generator that adds automatic builder pattern generation for all Rust API operations.

**Generator Location**: The generator is included as a git submodule at `./openapi-generator-rust-builders/`

**Build and generate:**
```bash
# Build the modified generator
cd openapi-generator-rust-builders
./mvnw clean package -DskipTests

# Generate the client
cd ..
java -jar openapi-generator-rust-builders/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate \
  -i openapi.json \
  -g rust \
  -o . \
  --additional-properties=packageName=twelve_data_client,packageVersion=0.1.0,library=reqwest
```

### OpenAPI Specification Modification

The original Twelve Data OpenAPI spec required one minor modification to work with this generator:

**Issue**: The spec defined API key authentication in two places:
- As a query parameter: `?apikey=value`
- As an Authorization header: `Authorization: apikey value`

This caused the generator to add the API key twice, which the Twelve Data API rejected.

**Solution**: Removed the `queryParameters` security scheme from the OpenAPI spec, keeping only the header-based authentication:

```json
{
  "security": [
    {
      "ApiKeyAuth": []
    }
  ],
  "securitySchemes": {
    "ApiKeyAuth": {
      "type": "apiKey",
      "in": "header",
      "name": "Authorization"
    }
  }
}
```

The modified OpenAPI specification is saved as `openapi.json` in this repository.

### Regenerating the Client

To regenerate the client after OpenAPI spec updates:

1. Download the latest spec from Twelve Data:
   ```bash
   curl https://api.twelvedata.com/doc/swagger/openapi.json -o openapi.json
   ```

2. Apply the security modification (remove query parameter security scheme)

3. Rebuild the generator (if needed):
   ```bash
   cd openapi-generator-rust-builders
   ./mvnw clean package -DskipTests
   cd ..
   ```

4. Generate the client:
   ```bash
   java -jar openapi-generator-rust-builders/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate \
     -i openapi.json \
     -g rust \
     -o . \
     --additional-properties=packageName=twelve_data_client,packageVersion=0.1.0,library=reqwest
   ```

5. Add tokio dev-dependency back to Cargo.toml (it gets overwritten):
   ```toml
   [dev-dependencies]
   tokio = { version = "1", features = ["full"] }
   ```

## Examples

See the `examples/` directory for complete working examples:

- [`time_series_builder.rs`](examples/time_series_builder.rs) - Fetching historical time series data with builder pattern

Run an example:
```bash
export TWELVE_DATA_API_KEY="your_api_key"
cargo run --example time_series_builder
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
