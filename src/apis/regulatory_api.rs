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

/// struct for passing parameters to the method [`get_direct_holders`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetDirectHoldersParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>
}

impl GetDirectHoldersParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetDirectHoldersParamsBuilder {
        GetDirectHoldersParamsBuilder::default()
    }
}

/// Builder for [`GetDirectHoldersParams`]
#[derive(Clone, Debug, Default)]
pub struct GetDirectHoldersParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>
}

impl GetDirectHoldersParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
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
    /// Exchange where instrument is traded
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetDirectHoldersParams {
        GetDirectHoldersParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_fund_holders`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetFundHoldersParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>
}

impl GetFundHoldersParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetFundHoldersParamsBuilder {
        GetFundHoldersParamsBuilder::default()
    }
}

/// Builder for [`GetFundHoldersParams`]
#[derive(Clone, Debug, Default)]
pub struct GetFundHoldersParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>
}

impl GetFundHoldersParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
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
    /// Exchange where instrument is traded
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetFundHoldersParams {
        GetFundHoldersParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_insider_transactions`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetInsiderTransactionsParams {
    /// The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `TSLA`. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded, e.g., `Nasdaq`, `NSE`
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., United States or US.
    pub country: Option<String>
}

impl GetInsiderTransactionsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetInsiderTransactionsParamsBuilder {
        GetInsiderTransactionsParamsBuilder::default()
    }
}

/// Builder for [`GetInsiderTransactionsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetInsiderTransactionsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `TSLA`. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded, e.g., `Nasdaq`, `NSE`
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., United States or US.
    country: Option<String>
}

impl GetInsiderTransactionsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `TSLA`. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
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
    /// Exchange where instrument is traded, e.g., `Nasdaq`, `NSE`
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// Country where instrument is traded, e.g., United States or US.
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetInsiderTransactionsParams {
        GetInsiderTransactionsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_institutional_holders`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetInstitutionalHoldersParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>
}

impl GetInstitutionalHoldersParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetInstitutionalHoldersParamsBuilder {
        GetInstitutionalHoldersParamsBuilder::default()
    }
}

/// Builder for [`GetInstitutionalHoldersParams`]
#[derive(Clone, Debug, Default)]
pub struct GetInstitutionalHoldersParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>
}

impl GetInstitutionalHoldersParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
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
    /// Exchange where instrument is traded
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetInstitutionalHoldersParams {
        GetInstitutionalHoldersParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_source_sanctioned_entities`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetSourceSanctionedEntitiesParams {
    /// Sanctions source
    pub source: String
}

impl GetSourceSanctionedEntitiesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetSourceSanctionedEntitiesParamsBuilder {
        GetSourceSanctionedEntitiesParamsBuilder::default()
    }
}

/// Builder for [`GetSourceSanctionedEntitiesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetSourceSanctionedEntitiesParamsBuilder {
    /// Sanctions source
    source: String
}

impl GetSourceSanctionedEntitiesParamsBuilder {
    /// Sanctions source
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetSourceSanctionedEntitiesParams {
        GetSourceSanctionedEntitiesParams {
            source: self.source
        }
    }
}

/// struct for passing parameters to the method [`get_tax_info`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetTaxInfoParams {
    /// The ticker symbol of an instrument for which data is requested, e.g., `SKYQ`, `AIRE`, `ALM:BME`, `HSI:HKEX`.
    pub symbol: Option<String>,
    /// The ISIN of an instrument for which data is requested
    pub isin: Option<String>,
    /// The FIGI of an instrument for which data is requested
    pub figi: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The exchange name where the instrument is traded, e.g., `Nasdaq`, `Euronext`
    pub exchange: Option<String>,
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON`
    pub mic_code: Option<String>
}

impl GetTaxInfoParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetTaxInfoParamsBuilder {
        GetTaxInfoParamsBuilder::default()
    }
}

/// Builder for [`GetTaxInfoParams`]
#[derive(Clone, Debug, Default)]
pub struct GetTaxInfoParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested, e.g., `SKYQ`, `AIRE`, `ALM:BME`, `HSI:HKEX`.
    symbol: Option<String>,
    /// The ISIN of an instrument for which data is requested
    isin: Option<String>,
    /// The FIGI of an instrument for which data is requested
    figi: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The exchange name where the instrument is traded, e.g., `Nasdaq`, `Euronext`
    exchange: Option<String>,
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON`
    mic_code: Option<String>
}

impl GetTaxInfoParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested, e.g., `SKYQ`, `AIRE`, `ALM:BME`, `HSI:HKEX`.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// The ISIN of an instrument for which data is requested
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The FIGI of an instrument for which data is requested
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub fn cusip(mut self, cusip: impl Into<String>) -> Self {
        self.cusip = Some(cusip.into());
        self
    }
    /// The exchange name where the instrument is traded, e.g., `Nasdaq`, `Euronext`
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON`
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetTaxInfoParams {
        GetTaxInfoParams {
            symbol: self.symbol,
            isin: self.isin,
            figi: self.figi,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code
        }
    }
}


/// struct for typed errors of method [`get_direct_holders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDirectHoldersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_fund_holders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFundHoldersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_insider_transactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInsiderTransactionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_institutional_holders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstitutionalHoldersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_source_sanctioned_entities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSourceSanctionedEntitiesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tax_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTaxInfoError {
    UnknownValue(serde_json::Value),
}


/// The direct holders endpoint provides detailed information about the number of shares directly held by individuals or entities as recorded in a company's official share registry. This data is essential for understanding the distribution of stock ownership within a company, helping users identify major shareholders and assess shareholder concentration.
pub async fn get_direct_holders(configuration: &configuration::Configuration, params: GetDirectHoldersParams) -> Result<models::GetDirectHolders200Response, Error<GetDirectHoldersError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/direct_holders", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetDirectHolders200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetDirectHolders200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDirectHoldersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The fund holders endpoint provides detailed information about the proportion of a company's stock that is owned by mutual fund holders. It returns data on the number of shares held, the percentage of total shares outstanding, and the names of the mutual funds involved. This endpoint is useful for users looking to understand mutual fund investment in a specific company.
pub async fn get_fund_holders(configuration: &configuration::Configuration, params: GetFundHoldersParams) -> Result<models::GetFundHolders200Response, Error<GetFundHoldersError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/fund_holders", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetFundHolders200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetFundHolders200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFundHoldersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The insider transaction endpoint provides detailed data on trades executed by company insiders, such as executives and directors. It returns information including the insider's name, their role, the transaction type, the number of shares, the transaction date, and the price per share. This endpoint is useful for tracking insider activity and understanding potential insider sentiment towards a company's stock.
pub async fn get_insider_transactions(configuration: &configuration::Configuration, params: GetInsiderTransactionsParams) -> Result<models::GetInsiderTransactions200Response, Error<GetInsiderTransactionsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/insider_transactions", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetInsiderTransactions200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetInsiderTransactions200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInsiderTransactionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The institutional holders endpoint provides detailed information on the percentage and amount of a company's stock owned by institutional investors, such as pension funds, insurance companies, and investment firms. This data is essential for understanding the influence and involvement of large entities in a company's ownership structure.
pub async fn get_institutional_holders(configuration: &configuration::Configuration, params: GetInstitutionalHoldersParams) -> Result<models::GetInstitutionalHolders200Response, Error<GetInstitutionalHoldersError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/institutional_holders", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetInstitutionalHolders200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetInstitutionalHolders200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInstitutionalHoldersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The sanctions entities endpoint provides a comprehensive list of entities sanctioned by a specified authority, such as OFAC, UK, EU, or AU. Users can retrieve detailed information about individuals, organizations, and other entities subject to sanctions from the chosen source, facilitating compliance and risk management processes.
pub async fn get_source_sanctioned_entities(configuration: &configuration::Configuration, params: GetSourceSanctionedEntitiesParams) -> Result<models::GetSourceSanctionedEntities200Response, Error<GetSourceSanctionedEntitiesError>> {
    // Extract parameters from params struct
    let p_path_source = params.source;

    let uri_str = format!("{}/sanctions/{source}", configuration.base_path, source=crate::apis::urlencode(p_path_source));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetSourceSanctionedEntities200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetSourceSanctionedEntities200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSourceSanctionedEntitiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The tax information endpoint provides detailed tax-related data for a specified financial instrument, including applicable tax rates and relevant tax codes. This information is essential for users needing to understand the tax implications associated with trading or investing in specific instruments.
pub async fn get_tax_info(configuration: &configuration::Configuration, params: GetTaxInfoParams) -> Result<models::GetTaxInfo200Response, Error<GetTaxInfoError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_isin = params.isin;
    let p_query_figi = params.figi;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;

    let uri_str = format!("{}/tax_info", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_isin {
        req_builder = req_builder.query(&[("isin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_figi {
        req_builder = req_builder.query(&[("figi", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cusip {
        req_builder = req_builder.query(&[("cusip", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTaxInfo200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTaxInfo200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTaxInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

