/*
 * Twelve Data API
 *
 * ## Overview  Welcome to Twelve Data developer docs — your gateway to comprehensive financial market data through a powerful and easy-to-use API. Twelve Data provides access to financial markets across over 50 global countries, covering more than 1 million public instruments, including stocks, forex, ETFs, mutual funds, commodities, and cryptocurrencies.  ## Quickstart  To get started, you'll need to sign up for an API key. Once you have your API key, you can start making requests to the API.  ### Step 1: Create Twelve Data account  Sign up on the Twelve Data website to create your account [here](https://twelvedata.com/register). This gives you access to the API dashboard and your API key.  ### Step 2: Get your API key  After signing in, navigate to your [dashboard](https://twelvedata.com/account/api-keys) to find your unique API key. This key is required to authenticate all API and WebSocket requests.  ### Step 3: Make your first request  Try a simple API call with cURL to fetch the latest price for Apple (AAPL):  ``` curl \"https://api.twelvedata.com/price?symbol=AAPL&apikey=your_api_key\" ```  ### Step 4: Make a request from Python or Javascript  Use our client libraries or standard HTTP clients to make API calls programmatically. Here’s an example in [Python](https://github.com/twelvedata/twelvedata-python) and JavaScript:  #### Python (using official Twelve Data SDK):  ```python from twelvedata import TDClient  # Initialize client with your API key td = TDClient(apikey=\"your_api_key\")  # Get latest price for Apple price = td.price(symbol=\"AAPL\").as_json()  print(price) ```  #### JavaScript (Node.js):  ```javascript const fetch = require('node-fetch');  fetch('https://api.twelvedata.com/price?symbol=AAPL&apikey=your_api_key') &nbsp;&nbsp;.then(response => response.json()) &nbsp;&nbsp;.then(data => console.log(data)); ```  ### Step 5: Perform correlation analysis between Tesla and Microsoft prices  Fetch historical price data for Tesla (TSLA) and Microsoft (MSFT) and calculate the correlation of their closing prices:  ```python from twelvedata import TDClient import pandas as pd  # Initialize client with your API key td = TDClient(apikey=\"your_api_key\")  # Fetch historical price data for Tesla tsla_ts = td.time_series( &nbsp;&nbsp;&nbsp;&nbsp;symbol=\"TSLA\", &nbsp;&nbsp;&nbsp;&nbsp;interval=\"1day\", &nbsp;&nbsp;&nbsp;&nbsp;outputsize=100 ).as_pandas()  # Fetch historical price data for Microsoft msft_ts = td.time_series( &nbsp;&nbsp;&nbsp;&nbsp;symbol=\"MSFT\", &nbsp;&nbsp;&nbsp;&nbsp;interval=\"1day\", &nbsp;&nbsp;&nbsp;&nbsp;outputsize=100 ).as_pandas()  # Align data on datetime index combined = pd.concat( &nbsp;&nbsp;&nbsp;&nbsp;[tsla_ts['close'].astype(float), msft_ts['close'].astype(float)], &nbsp;&nbsp;&nbsp;&nbsp;axis=1, &nbsp;&nbsp;&nbsp;&nbsp;keys=[\"TSLA\", \"MSFT\"] ).dropna()  # Calculate correlation correlation = combined[\"TSLA\"].corr(combined[\"MSFT\"]) print(f\"Correlation of closing prices between TSLA and MSFT: {correlation:.2f}\") ```  ### Authentication  Authenticate your requests using one of these methods:  #### Query parameter method ``` GET https://api.twelvedata.com/endpoint?symbol=AAPL&apikey=your_api_key ```  #### HTTP header method (recommended) ``` Authorization: apikey your_api_key ```  ##### API key useful information <ul> <li> Demo API key (<code>apikey=demo</code>) available for demo requests</li> <li> Personal API key required for full access</li> <li> Premium endpoints and data require higher-tier plans (testable with <a href=\"https://twelvedata.com/exchanges\">trial symbols</a>)</li> </ul>  ### API endpoints   Service | Base URL | ---------|----------|  REST API | `https://api.twelvedata.com` |  WebSocket | `wss://ws.twelvedata.com` |  ### Parameter guidelines <ul> <li><b>Separator:</b> Use <code>&</code> to separate multiple parameters</li> <li><b>Case sensitivity:</b> Parameter names are case-insensitive</li>  <ul><li><code>symbol=AAPL</code> = <code>symbol=aapl</code></li></ul>  <li><b>Multiple values:</b> Separate with commas where supported</li> </ul>  ### Response handling  #### Default format All responses return JSON format by default unless otherwise specified.  #### Null values <b>Important:</b> Some response fields may contain `null` values when data is unavailable for specific metrics. This is expected behavior, not an error.  ##### Best Practices: <ul> <li>Always implement <code>null</code> value handling in your application</li> <li>Use defensive programming techniques for data processing</li> <li>Consider fallback values or error handling for critical metrics</li> </ul>  #### Error handling Structure your code to gracefully handle: <ul> <li>Network timeouts</li> <li>Rate limiting responses</li> <li>Invalid parameter errors</li> <li>Data unavailability periods</li> </ul>  ##### Best practices <ul> <li><b>Rate limits:</b> Adhere to your plan’s rate limits to avoid throttling. Check your dashboard for details.</li> <li><b>Error handling:</b> Implement retry logic for transient errors (e.g., <code>429 Too Many Requests</code>).</li> <li><b>Caching:</b> Cache responses for frequently accessed data to reduce API calls and improve performance.</li> <li><b>Secure storage:</b> Store your API key securely and never expose it in client-side code or public repositories.</li> </ul>  ## Errors  Twelve Data API employs a standardized error response format, delivering a JSON object with `code`, `message`, and `status` keys for clear and consistent error communication.  ### Codes  Below is a table of possible error codes, their HTTP status, meanings, and resolution steps:   Code | status | Meaning | Resolution |  --- | --- | --- | --- |  **400** | Bad Request | Invalid or incorrect parameter(s) provided. | Check the `message` in the response for details. Refer to the API Documenta­tion to correct the input. |  **401** | Unauthor­ized | Invalid or incorrect API key. | Verify your API key is correct. Sign up for a key <a href=\"https://twelvedata.com/account/api-keys\">here</a>. |  **403** | Forbidden | API key lacks permissions for the requested resource (upgrade required). | Upgrade your plan <a href=\"https://twelvedata.com/pricing\">here</a>. |  **404** | Not Found | Requested data could not be found. | Adjust parameters to be less strict as they may be too restrictive. |  **414** | Parameter Too Long | Input parameter array exceeds the allowed length. | Follow the `message` guidance to adjust the parameter length. |  **429** | Too Many Requests | API request limit reached for your key. | Wait briefly or upgrade your plan <a href=\"https://twelvedata.com/pricing\">here</a>. |  **500** | Internal Server Error | Server-side issue occurred; retry later. | Contact support <a href=\"https://twelvedata.com/contact\">here</a> for assistance. |  ### Example error response  Consider the following invalid request:  ``` https://api.twelvedata.com/time_series?symbol=AAPL&interval=0.99min&apikey=your_api_key ```  Due to the incorrect `interval` value, the API returns:  ```json { &nbsp;&nbsp;\"code\": 400, &nbsp;&nbsp;\"message\": \"Invalid **interval** provided: 0.99min. Supported intervals: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h, 8h, 1day, 1week, 1month\", &nbsp;&nbsp;\"status\": \"error\" } ```  Refer to the API Documentation for valid parameter values to resolve such errors.  ## Libraries  Twelve Data provides a growing ecosystem of libraries and integrations to help you build faster and smarter in your preferred environment. Official libraries are actively maintained by the Twelve Data team, while selected community-built libraries offer additional flexibility.  A full list is available on our [GitHub profile](https://github.com/search?q=twelvedata).  ### Official SDKs <ul> <li><b>Python:</b> <a href=\"https://github.com/twelvedata/twelvedata-python\">twelvedata-python</a></li> <li><b>R:</b> <a href=\"https://github.com/twelvedata/twelvedata-r-sdk\">twelvedata-r-sdk</a></li> </ul>  ### AI integrations <ul> <li><b>Twelve Data MCP Server:</b> <a href=\"https://github.com/twelvedata/mcp\">Repository</a> — Model Context Protocol (MCP) server that provides seamless integration with AI assistants and language models, enabling direct access to Twelve Data's financial market data within conversational interfaces and AI workflows.</li> </ul>  ### Spreadsheet add-ons <ul> <li><b>Excel:</b> <a href=\"https://twelvedata.com/excel\">Excel Add-in</a></li> <li><b>Google Sheets:</b> <a href=\"https://twelvedata.com/google-sheets\">Google Sheets Add-on</a></li> </ul>  ### Community libraries  The community has developed libraries in several popular languages. You can explore more community libraries on [GitHub](https://github.com/search?q=twelvedata). <ul> <li><b>C#:</b> <a href=\"https://github.com/pseudomarkets/TwelveDataSharp\">TwelveDataSharp</a></li> <li><b>JavaScript:</b> <a href=\"https://github.com/evzaboun/twelvedata\">twelvedata</a></li> <li><b>PHP:</b> <a href=\"https://github.com/ingelby/twelvedata\">twelvedata</a></li> <li><b>Go:</b> <a href=\"https://github.com/soulgarden/twelvedata\">twelvedata</a></li> <li><b>TypeScript:</b> <a href=\"https://github.com/Clyde-Goodall/twelve-data-wrapper\">twelve-data-wrapper</a></li> </ul>  ### Other Twelve Data repositories <ul> <li><b>searchindex</b> <i>(Go)</i>: <a href=\"https://github.com/twelvedata/searchindex\">Repository</a> — In-memory search index by strings</li> <li><b>ws-tools</b> <i>(Python)</i>: <a href=\"https://github.com/twelvedata/ws-tools\">Repository</a> — Utility tools for WebSocket stream handling</li> </ul>  ### API specification <ul> <li><b>OpenAPI / Swagger:</b> Access the <a href=\"https://api.twelvedata.com/doc/swagger/openapi.json\">complete API specification</a> in OpenAPI format. You can use this file to automatically generate client libraries in your preferred programming language, explore the API interactively via Swagger tools, or integrate Twelve Data seamlessly into your AI and LLM workflows.</li> </ul>
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};

/// struct for passing parameters to the method [`get_mutual_funds_world`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldParamsBuilder {
        GetMutualFundsWorldParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldParams {
        GetMutualFundsWorldParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_world_composition`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldCompositionParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldCompositionParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldCompositionParamsBuilder {
        GetMutualFundsWorldCompositionParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldCompositionParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldCompositionParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldCompositionParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldCompositionParams {
        GetMutualFundsWorldCompositionParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_world_performance`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldPerformanceParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldPerformanceParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldPerformanceParamsBuilder {
        GetMutualFundsWorldPerformanceParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldPerformanceParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldPerformanceParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldPerformanceParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldPerformanceParams {
        GetMutualFundsWorldPerformanceParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_world_purchase_info`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldPurchaseInfoParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldPurchaseInfoParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldPurchaseInfoParamsBuilder {
        GetMutualFundsWorldPurchaseInfoParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldPurchaseInfoParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldPurchaseInfoParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldPurchaseInfoParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldPurchaseInfoParams {
        GetMutualFundsWorldPurchaseInfoParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_world_ratings`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldRatingsParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldRatingsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldRatingsParamsBuilder {
        GetMutualFundsWorldRatingsParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldRatingsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldRatingsParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldRatingsParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldRatingsParams {
        GetMutualFundsWorldRatingsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_world_risk`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldRiskParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldRiskParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldRiskParamsBuilder {
        GetMutualFundsWorldRiskParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldRiskParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldRiskParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldRiskParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldRiskParams {
        GetMutualFundsWorldRiskParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_world_summary`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldSummaryParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldSummaryParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldSummaryParamsBuilder {
        GetMutualFundsWorldSummaryParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldSummaryParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldSummaryParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldSummaryParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldSummaryParams {
        GetMutualFundsWorldSummaryParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_world_sustainability`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldSustainabilityParams {
    /// Symbol ticker of mutual fund
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub dp: Option<i64>
}

impl GetMutualFundsWorldSustainabilityParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsWorldSustainabilityParamsBuilder {
        GetMutualFundsWorldSustainabilityParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsWorldSustainabilityParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsWorldSustainabilityParamsBuilder {
    /// Symbol ticker of mutual fund
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    dp: Option<i64>
}

impl GetMutualFundsWorldSustainabilityParamsBuilder {
    /// Symbol ticker of mutual fund
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        
        self
    }
    /// Filter by financial instrument global identifier (FIGI)
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        
        self
    }
    /// Filter by international securities identification number (ISIN)
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        
        self
    }
    /// Number of decimal places for floating values. Accepts value in range [0,11]
    pub fn dp(mut self, dp: i64) -> Self {
        
        self.dp = Some(dp);
        
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsWorldSustainabilityParams {
        GetMutualFundsWorldSustainabilityParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            dp: self.dp
        }
    }
}


/// struct for typed errors of method [`get_mutual_funds_world`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_world_composition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldCompositionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_world_performance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldPerformanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_world_purchase_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldPurchaseInfoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_world_ratings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldRatingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_world_risk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldRiskError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_world_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldSummaryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_world_sustainability`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsWorldSustainabilityError {
    UnknownValue(serde_json::Value),
}


/// The mutual full data endpoint provides detailed information about global mutual funds. It returns a comprehensive dataset that includes a summary of the fund, its performance metrics, risk assessment, ratings, asset composition, purchase details, and sustainability factors. This endpoint is essential for users seeking in-depth insights into mutual funds on a global scale, allowing them to evaluate various aspects such as investment performance, risk levels, and environmental impact.
pub async fn get_mutual_funds_world(configuration: &configuration::Configuration, params: GetMutualFundsWorldParams) -> Result<models::GetMutualFundsWorld200Response, Error<GetMutualFundsWorldError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorld200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorld200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds compositions endpoint provides detailed information about the portfolio composition of a specified mutual fund. It returns data on sector allocations, individual holdings, and their respective weighted exposures. This endpoint is useful for users seeking to understand the investment distribution and risk profile of a mutual fund.
pub async fn get_mutual_funds_world_composition(configuration: &configuration::Configuration, params: GetMutualFundsWorldCompositionParams) -> Result<models::GetMutualFundsWorldComposition200Response, Error<GetMutualFundsWorldCompositionError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world/composition", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorldComposition200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorldComposition200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldCompositionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds performances endpoint provides comprehensive performance data for mutual funds globally. It returns metrics such as trailing returns, annual returns, quarterly returns, and load-adjusted returns.
pub async fn get_mutual_funds_world_performance(configuration: &configuration::Configuration, params: GetMutualFundsWorldPerformanceParams) -> Result<models::GetMutualFundsWorldPerformance200Response, Error<GetMutualFundsWorldPerformanceError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world/performance", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorldPerformance200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorldPerformance200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldPerformanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds purchase information endpoint provides detailed purchasing details for global mutual funds. It returns data on minimum investment requirements, current pricing, and a list of brokerages where the mutual fund can be purchased. This endpoint is useful for users looking to understand the entry requirements and options available for investing in specific mutual funds.
pub async fn get_mutual_funds_world_purchase_info(configuration: &configuration::Configuration, params: GetMutualFundsWorldPurchaseInfoParams) -> Result<models::GetMutualFundsWorldPurchaseInfo200Response, Error<GetMutualFundsWorldPurchaseInfoError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world/purchase_info", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorldPurchaseInfo200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorldPurchaseInfo200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldPurchaseInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds ratings endpoint provides detailed ratings for mutual funds across global markets. It returns data on the performance and quality of mutual funds, including ratings calculated in-house by Twelve Data and from various financial institutions.
pub async fn get_mutual_funds_world_ratings(configuration: &configuration::Configuration, params: GetMutualFundsWorldRatingsParams) -> Result<models::GetMutualFundsWorldRatings200Response, Error<GetMutualFundsWorldRatingsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world/ratings", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorldRatings200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorldRatings200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldRatingsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds risk endpoint provides detailed risk metrics for global mutual funds. It returns data such as standard deviation, beta, and Sharpe ratio, which help assess the volatility and risk profile of mutual funds across different markets.
pub async fn get_mutual_funds_world_risk(configuration: &configuration::Configuration, params: GetMutualFundsWorldRiskParams) -> Result<models::GetMutualFundsWorldRisk200Response, Error<GetMutualFundsWorldRiskError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world/risk", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorldRisk200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorldRisk200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldRiskError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds summary endpoint provides a concise overview of global mutual funds, including key details such as fund name, symbol, asset class, and region. This endpoint is useful for quickly obtaining essential information about various mutual funds worldwide, aiding in the comparison and selection of funds for investment portfolios.
pub async fn get_mutual_funds_world_summary(configuration: &configuration::Configuration, params: GetMutualFundsWorldSummaryParams) -> Result<models::GetMutualFundsWorldSummary200Response, Error<GetMutualFundsWorldSummaryError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world/summary", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorldSummary200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorldSummary200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldSummaryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds sustainability endpoint provides detailed information on the sustainability and Environmental, Social, and Governance (ESG) ratings of global mutual funds. It returns data such as ESG scores, sustainability metrics, and fund identifiers.
pub async fn get_mutual_funds_world_sustainability(configuration: &configuration::Configuration, params: GetMutualFundsWorldSustainabilityParams) -> Result<models::GetMutualFundsWorldSustainability200Response, Error<GetMutualFundsWorldSustainabilityError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/mutual_funds/world/sustainability", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsWorldSustainability200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsWorldSustainability200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsWorldSustainabilityError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

