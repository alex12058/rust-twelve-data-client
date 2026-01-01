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

/// struct for passing parameters to the method [`get_analyst_ratings_light`]
#[derive(Clone, Debug, Default)]
pub struct GetAnalystRatingsLightParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by rating change action
    pub rating_change: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>
}

impl GetAnalystRatingsLightParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetAnalystRatingsLightParamsBuilder {
        GetAnalystRatingsLightParamsBuilder::default()
    }
}

/// Builder for [`GetAnalystRatingsLightParams`]
#[derive(Clone, Debug, Default)]
pub struct GetAnalystRatingsLightParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by rating change action
    rating_change: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>
}

impl GetAnalystRatingsLightParamsBuilder {
    /// Filter by symbol
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
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Filter by rating change action
    pub fn rating_change(mut self, rating_change: impl Into<String>) -> Self {
        self.rating_change = Some(rating_change.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetAnalystRatingsLightParams {
        GetAnalystRatingsLightParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            rating_change: self.rating_change,
            outputsize: self.outputsize,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_analyst_ratings_us_equities`]
#[derive(Clone, Debug, Default)]
pub struct GetAnalystRatingsUsEquitiesParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by rating change action
    pub rating_change: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>
}

impl GetAnalystRatingsUsEquitiesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetAnalystRatingsUsEquitiesParamsBuilder {
        GetAnalystRatingsUsEquitiesParamsBuilder::default()
    }
}

/// Builder for [`GetAnalystRatingsUsEquitiesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetAnalystRatingsUsEquitiesParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by rating change action
    rating_change: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>
}

impl GetAnalystRatingsUsEquitiesParamsBuilder {
    /// Filter by symbol
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
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Filter by rating change action
    pub fn rating_change(mut self, rating_change: impl Into<String>) -> Self {
        self.rating_change = Some(rating_change.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetAnalystRatingsUsEquitiesParams {
        GetAnalystRatingsUsEquitiesParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            rating_change: self.rating_change,
            outputsize: self.outputsize
        }
    }
}

/// struct for passing parameters to the method [`get_earnings_estimate`]
#[derive(Clone, Debug, Default)]
pub struct GetEarningsEstimateParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// The FIGI of an instrument for which data is requested
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>
}

impl GetEarningsEstimateParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEarningsEstimateParamsBuilder {
        GetEarningsEstimateParamsBuilder::default()
    }
}

/// Builder for [`GetEarningsEstimateParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEarningsEstimateParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// The FIGI of an instrument for which data is requested
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>
}

impl GetEarningsEstimateParamsBuilder {
    /// Filter by symbol
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// The FIGI of an instrument for which data is requested
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
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// Exchange where instrument is traded
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEarningsEstimateParams {
        GetEarningsEstimateParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            exchange: self.exchange
        }
    }
}

/// struct for passing parameters to the method [`get_edgar_filings_archive`]
#[derive(Clone, Debug, Default)]
pub struct GetEdgarFilingsArchiveParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by form types, example `8-K`, `EX-1.1`
    pub form_type: Option<String>,
    /// Filter by filled time from
    pub filled_from: Option<String>,
    /// Filter by filled time to
    pub filled_to: Option<String>,
    /// Page number
    pub page: Option<i64>,
    /// Number of records in response
    pub page_size: Option<i64>
}

impl GetEdgarFilingsArchiveParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEdgarFilingsArchiveParamsBuilder {
        GetEdgarFilingsArchiveParamsBuilder::default()
    }
}

/// Builder for [`GetEdgarFilingsArchiveParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEdgarFilingsArchiveParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by form types, example `8-K`, `EX-1.1`
    form_type: Option<String>,
    /// Filter by filled time from
    filled_from: Option<String>,
    /// Filter by filled time to
    filled_to: Option<String>,
    /// Page number
    page: Option<i64>,
    /// Number of records in response
    page_size: Option<i64>
}

impl GetEdgarFilingsArchiveParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
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
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// Filter by form types, example `8-K`, `EX-1.1`
    pub fn form_type(mut self, form_type: impl Into<String>) -> Self {
        self.form_type = Some(form_type.into());
        self
    }
    /// Filter by filled time from
    pub fn filled_from(mut self, filled_from: impl Into<String>) -> Self {
        self.filled_from = Some(filled_from.into());
        self
    }
    /// Filter by filled time to
    pub fn filled_to(mut self, filled_to: impl Into<String>) -> Self {
        self.filled_to = Some(filled_to.into());
        self
    }
    /// Page number
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    /// Number of records in response
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEdgarFilingsArchiveParams {
        GetEdgarFilingsArchiveParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            form_type: self.form_type,
            filled_from: self.filled_from,
            filled_to: self.filled_to,
            page: self.page,
            page_size: self.page_size
        }
    }
}

/// struct for passing parameters to the method [`get_eps_revisions`]
#[derive(Clone, Debug, Default)]
pub struct GetEpsRevisionsParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>
}

impl GetEpsRevisionsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEpsRevisionsParamsBuilder {
        GetEpsRevisionsParamsBuilder::default()
    }
}

/// Builder for [`GetEpsRevisionsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEpsRevisionsParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>
}

impl GetEpsRevisionsParamsBuilder {
    /// Filter by symbol
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
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEpsRevisionsParams {
        GetEpsRevisionsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            exchange: self.exchange
        }
    }
}

/// struct for passing parameters to the method [`get_eps_trend`]
#[derive(Clone, Debug, Default)]
pub struct GetEpsTrendParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>
}

impl GetEpsTrendParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEpsTrendParamsBuilder {
        GetEpsTrendParamsBuilder::default()
    }
}

/// Builder for [`GetEpsTrendParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEpsTrendParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>
}

impl GetEpsTrendParamsBuilder {
    /// Filter by symbol
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
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEpsTrendParams {
        GetEpsTrendParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            exchange: self.exchange
        }
    }
}

/// struct for passing parameters to the method [`get_growth_estimates`]
#[derive(Clone, Debug, Default)]
pub struct GetGrowthEstimatesParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// The FIGI of an instrument for which data is requested
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>
}

impl GetGrowthEstimatesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetGrowthEstimatesParamsBuilder {
        GetGrowthEstimatesParamsBuilder::default()
    }
}

/// Builder for [`GetGrowthEstimatesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetGrowthEstimatesParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// The FIGI of an instrument for which data is requested
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>
}

impl GetGrowthEstimatesParamsBuilder {
    /// Filter by symbol
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// The FIGI of an instrument for which data is requested
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
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// Exchange where instrument is traded
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetGrowthEstimatesParams {
        GetGrowthEstimatesParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            exchange: self.exchange
        }
    }
}

/// struct for passing parameters to the method [`get_price_target`]
#[derive(Clone, Debug, Default)]
pub struct GetPriceTargetParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>
}

impl GetPriceTargetParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetPriceTargetParamsBuilder {
        GetPriceTargetParamsBuilder::default()
    }
}

/// Builder for [`GetPriceTargetParams`]
#[derive(Clone, Debug, Default)]
pub struct GetPriceTargetParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>
}

impl GetPriceTargetParamsBuilder {
    /// Filter by symbol
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
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetPriceTargetParams {
        GetPriceTargetParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            exchange: self.exchange
        }
    }
}

/// struct for passing parameters to the method [`get_recommendations`]
#[derive(Clone, Debug, Default)]
pub struct GetRecommendationsParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// The FIGI of an instrument for which data is requested
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The exchange name where the instrument is traded, e.g., `Nasdaq`, `NSE`.
    pub exchange: Option<String>
}

impl GetRecommendationsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetRecommendationsParamsBuilder {
        GetRecommendationsParamsBuilder::default()
    }
}

/// Builder for [`GetRecommendationsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetRecommendationsParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// The FIGI of an instrument for which data is requested
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The exchange name where the instrument is traded, e.g., `Nasdaq`, `NSE`.
    exchange: Option<String>
}

impl GetRecommendationsParamsBuilder {
    /// Filter by symbol
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// The FIGI of an instrument for which data is requested
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
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// The exchange name where the instrument is traded, e.g., `Nasdaq`, `NSE`.
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetRecommendationsParams {
        GetRecommendationsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            exchange: self.exchange
        }
    }
}

/// struct for passing parameters to the method [`get_revenue_estimate`]
#[derive(Clone, Debug, Default)]
pub struct GetRevenueEstimateParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Number of decimal places for floating values. Should be in range [0,11] inclusive
    pub dp: Option<i64>
}

impl GetRevenueEstimateParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetRevenueEstimateParamsBuilder {
        GetRevenueEstimateParamsBuilder::default()
    }
}

/// Builder for [`GetRevenueEstimateParams`]
#[derive(Clone, Debug, Default)]
pub struct GetRevenueEstimateParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Number of decimal places for floating values. Should be in range [0,11] inclusive
    dp: Option<i64>
}

impl GetRevenueEstimateParamsBuilder {
    /// Filter by symbol
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
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Number of decimal places for floating values. Should be in range [0,11] inclusive
    pub fn dp(mut self, dp: i64) -> Self {
        self.dp = Some(dp);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetRevenueEstimateParams {
        GetRevenueEstimateParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            country: self.country,
            exchange: self.exchange,
            dp: self.dp
        }
    }
}


/// struct for typed errors of method [`get_analyst_ratings_light`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAnalystRatingsLightError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_analyst_ratings_us_equities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAnalystRatingsUsEquitiesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_earnings_estimate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEarningsEstimateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_edgar_filings_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEdgarFilingsArchiveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_eps_revisions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEpsRevisionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_eps_trend`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEpsTrendError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_growth_estimates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGrowthEstimatesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_price_target`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPriceTargetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_recommendations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRecommendationsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_revenue_estimate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRevenueEstimateError {
    UnknownValue(serde_json::Value),
}


/// The analyst ratings snapshot endpoint provides a streamlined summary of ratings from analyst firms for both US and international markets. It delivers essential data on analyst recommendations, including buy, hold, and sell ratings, allowing users to quickly assess the general sentiment of analysts towards a particular stock.
pub async fn get_analyst_ratings_light(configuration: &configuration::Configuration, params: GetAnalystRatingsLightParams) -> Result<models::GetAnalystRatingsLight200Response, Error<GetAnalystRatingsLightError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_rating_change = params.rating_change;
    let p_query_outputsize = params.outputsize;
    let p_query_country = params.country;

    let uri_str = format!("{}/analyst_ratings/light", configuration.base_path);
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
    if let Some(ref param_value) = p_query_rating_change {
        req_builder = req_builder.query(&[("rating_change", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAnalystRatingsLight200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAnalystRatingsLight200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAnalystRatingsLightError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The analyst ratings US equities endpoint provides detailed information on analyst ratings for U.S. stocks. It returns data on the latest ratings issued by various analyst firms, including the rating itself, the firm issuing the rating, and any changes in the rating. This endpoint is useful for users tracking analyst opinions on U.S. equities, allowing them to see how professional analysts view the potential performance of specific stocks.
pub async fn get_analyst_ratings_us_equities(configuration: &configuration::Configuration, params: GetAnalystRatingsUsEquitiesParams) -> Result<models::GetAnalystRatingsUsEquities200Response, Error<GetAnalystRatingsUsEquitiesError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_rating_change = params.rating_change;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/analyst_ratings/us_equities", configuration.base_path);
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
    if let Some(ref param_value) = p_query_rating_change {
        req_builder = req_builder.query(&[("rating_change", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetAnalystRatingsUsEquities200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetAnalystRatingsUsEquities200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAnalystRatingsUsEquitiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The earnings estimate endpoint provides access to analysts' projected earnings per share (EPS) for a specific company, covering both upcoming quarterly and annual periods. This data is crucial for users who need to track and compare expected financial performance across different timeframes, aiding in the evaluation of a company's future profitability.
pub async fn get_earnings_estimate(configuration: &configuration::Configuration, params: GetEarningsEstimateParams) -> Result<models::GetEarningsEstimate200Response, Error<GetEarningsEstimateError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_exchange = params.exchange;

    let uri_str = format!("{}/earnings_estimate", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEarningsEstimate200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEarningsEstimate200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEarningsEstimateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The EDGAR fillings endpoint provides access to a comprehensive collection of financial documents submitted to the SEC, including real-time and historical forms, filings, and exhibits. Users can retrieve detailed information about company disclosures, financial statements, and regulatory submissions, enabling them to access essential compliance and financial data directly from the SEC's EDGAR system.
pub async fn get_edgar_filings_archive(configuration: &configuration::Configuration, params: GetEdgarFilingsArchiveParams) -> Result<models::GetEdgarFilingsArchive200Response, Error<GetEdgarFilingsArchiveError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_form_type = params.form_type;
    let p_query_filled_from = params.filled_from;
    let p_query_filled_to = params.filled_to;
    let p_query_page = params.page;
    let p_query_page_size = params.page_size;

    let uri_str = format!("{}/edgar_filings/archive", configuration.base_path);
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
    if let Some(ref param_value) = p_query_form_type {
        req_builder = req_builder.query(&[("form_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_filled_from {
        req_builder = req_builder.query(&[("filled_from", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_filled_to {
        req_builder = req_builder.query(&[("filled_to", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEdgarFilingsArchive200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEdgarFilingsArchive200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEdgarFilingsArchiveError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The EPS revisions endpoint provides updated analyst forecasts for a company's earnings per share (EPS) on both a quarterly and annual basis. It delivers data on how these EPS predictions have changed over the past week and month, allowing users to track recent adjustments in analyst expectations. This endpoint is useful for monitoring shifts in market sentiment regarding a company's financial performance.
pub async fn get_eps_revisions(configuration: &configuration::Configuration, params: GetEpsRevisionsParams) -> Result<models::GetEpsRevisions200Response, Error<GetEpsRevisionsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_exchange = params.exchange;

    let uri_str = format!("{}/eps_revisions", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEpsRevisions200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEpsRevisions200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEpsRevisionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The EPS trend endpoint provides detailed historical data on Earnings Per Share (EPS) trends over specified periods. It returns a comprehensive breakdown of estimated EPS changes, allowing users to track and analyze the progression of a company's earnings performance over time. This endpoint is ideal for users seeking to understand historical EPS fluctuations and assess financial growth patterns.
pub async fn get_eps_trend(configuration: &configuration::Configuration, params: GetEpsTrendParams) -> Result<models::GetEpsTrend200Response, Error<GetEpsTrendError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_exchange = params.exchange;

    let uri_str = format!("{}/eps_trend", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEpsTrend200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEpsTrend200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEpsTrendError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The growth estimates endpoint provides consensus analyst projections on a company's growth rates over various timeframes. It aggregates and averages estimates from multiple analysts, focusing on key financial metrics such as earnings per share and revenue. This endpoint is useful for obtaining a comprehensive view of expected company performance based on expert analysis.
pub async fn get_growth_estimates(configuration: &configuration::Configuration, params: GetGrowthEstimatesParams) -> Result<models::GetGrowthEstimates200Response, Error<GetGrowthEstimatesError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_exchange = params.exchange;

    let uri_str = format!("{}/growth_estimates", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetGrowthEstimates200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetGrowthEstimates200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGrowthEstimatesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The price target endpoint provides detailed projections of a security's future price as estimated by financial analysts. It returns data including the high, low, and average price targets. This endpoint is useful for users seeking to understand potential future valuations of specific securities based on expert analysis.
pub async fn get_price_target(configuration: &configuration::Configuration, params: GetPriceTargetParams) -> Result<models::GetPriceTarget200Response, Error<GetPriceTargetError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_exchange = params.exchange;

    let uri_str = format!("{}/price_target", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetPriceTarget200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetPriceTarget200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPriceTargetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The recommendations endpoint provides a summary of analyst opinions for a specific stock, delivering an average recommendation categorized as Strong Buy, Buy, Hold, or Sell. It also includes a numerical recommendation score, offering a quick overview of market sentiment based on expert analysis.
pub async fn get_recommendations(configuration: &configuration::Configuration, params: GetRecommendationsParams) -> Result<models::GetRecommendations200Response, Error<GetRecommendationsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_exchange = params.exchange;

    let uri_str = format!("{}/recommendations", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetRecommendations200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetRecommendations200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRecommendationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The revenue estimate endpoint provides a company's projected quarterly and annual revenue figures based on analysts' estimates. This data is useful for users seeking insights into expected company performance, allowing them to compare forecasted sales with historical data or other companies' estimates.
pub async fn get_revenue_estimate(configuration: &configuration::Configuration, params: GetRevenueEstimateParams) -> Result<models::GetRevenueEstimate200Response, Error<GetRevenueEstimateError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_country = params.country;
    let p_query_exchange = params.exchange;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/revenue_estimate", configuration.base_path);
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
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetRevenueEstimate200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetRevenueEstimate200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRevenueEstimateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

