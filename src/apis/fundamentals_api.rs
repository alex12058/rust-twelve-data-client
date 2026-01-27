/*
 * Twelve Data API
 *
 * ## Overview  Welcome to Twelve Data developer docs — your gateway to comprehensive financial market data through a powerful and easy-to-use API. Twelve Data provides access to financial markets across over 50 global countries, covering more than 1 million public instruments, including stocks, forex, ETFs, mutual funds, commodities, and cryptocurrencies.  ## Quickstart  To get started, you'll need to sign up for an API key. Once you have your API key, you can start making requests to the API.  ### Step 1: Create Twelve Data account  Sign up on the Twelve Data website to create your account [here](https://twelvedata.com/register). This gives you access to the API dashboard and your API key.  ### Step 2: Get your API key  After signing in, navigate to your [dashboard](https://twelvedata.com/account/api-keys) to find your unique API key. This key is required to authenticate all API and WebSocket requests.  ### Step 3: Make your first request  Try a simple API call with cURL to fetch the latest price for Apple (AAPL):  ``` curl \"https://api.twelvedata.com/price?symbol=AAPL&apikey=your_api_key\" ```  ### Step 4: Make a request from Python or Javascript  Use our client libraries or standard HTTP clients to make API calls programmatically. Here’s an example in [Python](https://github.com/twelvedata/twelvedata-python) and JavaScript:  #### Python (using official Twelve Data SDK):  ```python from twelvedata import TDClient  # Initialize client with your API key td = TDClient(apikey=\"your_api_key\")  # Get latest price for Apple price = td.price(symbol=\"AAPL\").as_json()  print(price) ```  #### JavaScript (Node.js):  ```javascript const fetch = require('node-fetch');  fetch('https://api.twelvedata.com/price?symbol=AAPL&apikey=your_api_key') &nbsp;&nbsp;.then(response => response.json()) &nbsp;&nbsp;.then(data => console.log(data)); ```  ### Step 5: Perform correlation analysis between Tesla and Microsoft prices  Fetch historical price data for Tesla (TSLA) and Microsoft (MSFT) and calculate the correlation of their closing prices:  ```python from twelvedata import TDClient import pandas as pd  # Initialize client with your API key td = TDClient(apikey=\"your_api_key\")  # Fetch historical price data for Tesla tsla_ts = td.time_series( &nbsp;&nbsp;&nbsp;&nbsp;symbol=\"TSLA\", &nbsp;&nbsp;&nbsp;&nbsp;interval=\"1day\", &nbsp;&nbsp;&nbsp;&nbsp;outputsize=100 ).as_pandas()  # Fetch historical price data for Microsoft msft_ts = td.time_series( &nbsp;&nbsp;&nbsp;&nbsp;symbol=\"MSFT\", &nbsp;&nbsp;&nbsp;&nbsp;interval=\"1day\", &nbsp;&nbsp;&nbsp;&nbsp;outputsize=100 ).as_pandas()  # Align data on datetime index combined = pd.concat( &nbsp;&nbsp;&nbsp;&nbsp;[tsla_ts['close'].astype(float), msft_ts['close'].astype(float)], &nbsp;&nbsp;&nbsp;&nbsp;axis=1, &nbsp;&nbsp;&nbsp;&nbsp;keys=[\"TSLA\", \"MSFT\"] ).dropna()  # Calculate correlation correlation = combined[\"TSLA\"].corr(combined[\"MSFT\"]) print(f\"Correlation of closing prices between TSLA and MSFT: {correlation:.2f}\") ```  ### Authentication  Authenticate your requests using one of these methods:  #### Query parameter method ``` GET https://api.twelvedata.com/endpoint?symbol=AAPL&apikey=your_api_key ```  #### HTTP header method (recommended) ``` Authorization: apikey your_api_key ```  ##### API key useful information <ul> <li> Demo API key (<code>apikey=demo</code>) available for demo requests</li> <li> Personal API key required for full access</li> <li> Premium endpoints and data require higher-tier plans (testable with <a href=\"https://twelvedata.com/exchanges\">trial symbols</a>)</li> </ul>  ### API endpoints   Service | Base URL | ---------|----------|  REST API | `https://api.twelvedata.com` |  WebSocket | `wss://ws.twelvedata.com` |  ### Parameter guidelines <ul> <li><b>Separator:</b> Use <code>&</code> to separate multiple parameters</li> <li><b>Case sensitivity:</b> Parameter names are case-insensitive</li>  <ul><li><code>symbol=AAPL</code> = <code>symbol=aapl</code></li></ul>  <li><b>Multiple values:</b> Separate with commas where supported</li> </ul>  ### Response handling  #### Default format All responses return JSON format by default unless otherwise specified.  #### Null values <b>Important:</b> Some response fields may contain `null` values when data is unavailable for specific metrics. This is expected behavior, not an error.  ##### Best Practices: <ul> <li>Always implement <code>null</code> value handling in your application</li> <li>Use defensive programming techniques for data processing</li> <li>Consider fallback values or error handling for critical metrics</li> </ul>  #### Error handling Structure your code to gracefully handle: <ul> <li>Network timeouts</li> <li>Rate limiting responses</li> <li>Invalid parameter errors</li> <li>Data unavailability periods</li> </ul>  ##### Best practices <ul> <li><b>Rate limits:</b> Adhere to your plan’s rate limits to avoid throttling. Check your dashboard for details.</li> <li><b>Error handling:</b> Implement retry logic for transient errors (e.g., <code>429 Too Many Requests</code>).</li> <li><b>Caching:</b> Cache responses for frequently accessed data to reduce API calls and improve performance.</li> <li><b>Secure storage:</b> Store your API key securely and never expose it in client-side code or public repositories.</li> </ul>  ## Errors  Twelve Data API employs a standardized error response format, delivering a JSON object with `code`, `message`, and `status` keys for clear and consistent error communication.  ### Codes  Below is a table of possible error codes, their HTTP status, meanings, and resolution steps:   Code | status | Meaning | Resolution |  --- | --- | --- | --- |  **400** | Bad Request | Invalid or incorrect parameter(s) provided. | Check the `message` in the response for details. Refer to the API Documenta­tion to correct the input. |  **401** | Unauthor­ized | Invalid or incorrect API key. | Verify your API key is correct. Sign up for a key <a href=\"https://twelvedata.com/account/api-keys\">here</a>. |  **403** | Forbidden | API key lacks permissions for the requested resource (upgrade required). | Upgrade your plan <a href=\"https://twelvedata.com/pricing\">here</a>. |  **404** | Not Found | Requested data could not be found. | Adjust parameters to be less strict as they may be too restrictive. |  **414** | Parameter Too Long | Input parameter array exceeds the allowed length. | Follow the `message` guidance to adjust the parameter length. |  **429** | Too Many Requests | API request limit reached for your key. | Wait briefly or upgrade your plan <a href=\"https://twelvedata.com/pricing\">here</a>. |  **500** | Internal Server Error | Server-side issue occurred; retry later. | Contact support <a href=\"https://twelvedata.com/contact\">here</a> for assistance. |  ### Example error response  Consider the following invalid request:  ``` https://api.twelvedata.com/time_series?symbol=AAPL&interval=0.99min&apikey=your_api_key ```  Due to the incorrect `interval` value, the API returns:  ```json { &nbsp;&nbsp;\"code\": 400, &nbsp;&nbsp;\"message\": \"Invalid **interval** provided: 0.99min. Supported intervals: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h, 8h, 1day, 1week, 1month\", &nbsp;&nbsp;\"status\": \"error\" } ```  Refer to the API Documentation for valid parameter values to resolve such errors.  ## Libraries  Twelve Data provides a growing ecosystem of libraries and integrations to help you build faster and smarter in your preferred environment. Official libraries are actively maintained by the Twelve Data team, while selected community-built libraries offer additional flexibility.  A full list is available on our [GitHub profile](https://github.com/search?q=twelvedata).  ### Official SDKs <ul> <li><b>Python:</b> <a href=\"https://github.com/twelvedata/twelvedata-python\">twelvedata-python</a></li> <li><b>R:</b> <a href=\"https://github.com/twelvedata/twelvedata-r-sdk\">twelvedata-r-sdk</a></li> </ul>  ### AI integrations <ul> <li><b>Twelve Data MCP Server:</b> <a href=\"https://github.com/twelvedata/mcp\">Repository</a> — Model Context Protocol (MCP) server that provides seamless integration with AI assistants and language models, enabling direct access to Twelve Data's financial market data within conversational interfaces and AI workflows.</li> </ul>  ### Spreadsheet add-ons <ul> <li><b>Excel:</b> <a href=\"https://twelvedata.com/excel\">Excel Add-in</a></li> <li><b>Google Sheets:</b> <a href=\"https://twelvedata.com/google-sheets\">Google Sheets Add-on</a></li> </ul>  ### Community libraries  The community has developed libraries in several popular languages. You can explore more community libraries on [GitHub](https://github.com/search?q=twelvedata). <ul> <li><b>C#:</b> <a href=\"https://github.com/pseudomarkets/TwelveDataSharp\">TwelveDataSharp</a></li> <li><b>JavaScript:</b> <a href=\"https://github.com/evzaboun/twelvedata\">twelvedata</a></li> <li><b>PHP:</b> <a href=\"https://github.com/ingelby/twelvedata\">twelvedata</a></li> <li><b>Go:</b> <a href=\"https://github.com/soulgarden/twelvedata\">twelvedata</a></li> <li><b>TypeScript:</b> <a href=\"https://github.com/Clyde-Goodall/twelve-data-wrapper\">twelve-data-wrapper</a></li> </ul>  ### Other Twelve Data repositories <ul> <li><b>searchindex</b> <i>(Go)</i>: <a href=\"https://github.com/twelvedata/searchindex\">Repository</a> — In-memory search index by strings</li> <li><b>ws-tools</b> <i>(Python)</i>: <a href=\"https://github.com/twelvedata/ws-tools\">Repository</a> — Utility tools for WebSocket stream handling</li> </ul>  ### API specification <ul> <li><b>OpenAPI / Swagger:</b> Access the <a href=\"https://api.twelvedata.com/doc/swagger/openapi.json\">complete API specification</a> in OpenAPI format. You can use this file to automatically generate client libraries in your preferred programming language, explore the API interactively via Swagger tools, or integrate Twelve Data seamlessly into your AI and LLM workflows.</li> </ul>
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for passing parameters to the method [`get_balance_sheet`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetBalanceSheetParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The reporting period for the balane sheet data
    pub period: Option<String>,
    /// Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetBalanceSheetParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetBalanceSheetParamsBuilder {
        GetBalanceSheetParamsBuilder::default()
    }
}

/// Builder for [`GetBalanceSheetParams`]
#[derive(Clone, Debug, Default)]
pub struct GetBalanceSheetParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The reporting period for the balane sheet data
    period: Option<String>,
    /// Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetBalanceSheetParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The reporting period for the balane sheet data
    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = Some(period.into());
        self
    }
    /// Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetBalanceSheetParams {
        GetBalanceSheetParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            period: self.period,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_balance_sheet_consolidated`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetBalanceSheetConsolidatedParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The reporting period for the balance sheet data.
    pub period: Option<String>,
    /// Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetBalanceSheetConsolidatedParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetBalanceSheetConsolidatedParamsBuilder {
        GetBalanceSheetConsolidatedParamsBuilder::default()
    }
}

/// Builder for [`GetBalanceSheetConsolidatedParams`]
#[derive(Clone, Debug, Default)]
pub struct GetBalanceSheetConsolidatedParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The reporting period for the balance sheet data.
    period: Option<String>,
    /// Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetBalanceSheetConsolidatedParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The reporting period for the balance sheet data.
    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = Some(period.into());
        self
    }
    /// Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetBalanceSheetConsolidatedParams {
        GetBalanceSheetConsolidatedParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            period: self.period,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_cash_flow`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetCashFlowParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The reporting period for the cash flow statements
    pub period: Option<String>,
    /// Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetCashFlowParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetCashFlowParamsBuilder {
        GetCashFlowParamsBuilder::default()
    }
}

/// Builder for [`GetCashFlowParams`]
#[derive(Clone, Debug, Default)]
pub struct GetCashFlowParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The reporting period for the cash flow statements
    period: Option<String>,
    /// Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetCashFlowParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The reporting period for the cash flow statements
    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = Some(period.into());
        self
    }
    /// Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetCashFlowParams {
        GetCashFlowParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            period: self.period,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_cash_flow_consolidated`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetCashFlowConsolidatedParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The reporting period for the cash flow statements
    pub period: Option<String>,
    /// Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetCashFlowConsolidatedParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetCashFlowConsolidatedParamsBuilder {
        GetCashFlowConsolidatedParamsBuilder::default()
    }
}

/// Builder for [`GetCashFlowConsolidatedParams`]
#[derive(Clone, Debug, Default)]
pub struct GetCashFlowConsolidatedParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The reporting period for the cash flow statements
    period: Option<String>,
    /// Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetCashFlowConsolidatedParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The reporting period for the cash flow statements
    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = Some(period.into());
        self
    }
    /// Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetCashFlowConsolidatedParams {
        GetCashFlowConsolidatedParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            period: self.period,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_dividends`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetDividendsParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Specifies the time range for which to retrieve dividend data. Accepts values such as `last` (most recent dividend), `next` (upcoming dividend), `1m` - `5y` for respective periods, or `full` for all available data. If provided together with `start_date` and/or `end_date`, this parameter takes precedence.
    pub range: Option<String>,
    /// Start date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence.
    pub start_date: Option<String>,
    /// End date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence.
    pub end_date: Option<String>,
    /// Specifies if there should be an adjustment
    pub adjust: Option<bool>,
}

impl GetDividendsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetDividendsParamsBuilder {
        GetDividendsParamsBuilder::default()
    }
}

/// Builder for [`GetDividendsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetDividendsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// Specifies the time range for which to retrieve dividend data. Accepts values such as `last` (most recent dividend), `next` (upcoming dividend), `1m` - `5y` for respective periods, or `full` for all available data. If provided together with `start_date` and/or `end_date`, this parameter takes precedence.
    range: Option<String>,
    /// Start date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence.
    start_date: Option<String>,
    /// End date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence.
    end_date: Option<String>,
    /// Specifies if there should be an adjustment
    adjust: Option<bool>,
}

impl GetDividendsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// Specifies the time range for which to retrieve dividend data. Accepts values such as `last` (most recent dividend), `next` (upcoming dividend), `1m` - `5y` for respective periods, or `full` for all available data. If provided together with `start_date` and/or `end_date`, this parameter takes precedence.
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }
    /// Start date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence.
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence.
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Specifies if there should be an adjustment
    pub fn adjust(mut self, adjust: bool) -> Self {
        self.adjust = Some(adjust);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetDividendsParams {
        GetDividendsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            range: self.range,
            start_date: self.start_date,
            end_date: self.end_date,
            adjust: self.adjust,
        }
    }
}

/// struct for passing parameters to the method [`get_dividends_calendar`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetDividendsCalendarParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Start date for the dividends calendar query. Only dividends with ex-dates on or after this date will be returned. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for the dividends calendar query. Only dividends with ex-dates on or before this date will be returned. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum
    pub outputsize: Option<i64>,
    /// Page number
    pub page: Option<i64>,
}

impl GetDividendsCalendarParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetDividendsCalendarParamsBuilder {
        GetDividendsCalendarParamsBuilder::default()
    }
}

/// Builder for [`GetDividendsCalendarParams`]
#[derive(Clone, Debug, Default)]
pub struct GetDividendsCalendarParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// Start date for the dividends calendar query. Only dividends with ex-dates on or after this date will be returned. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for the dividends calendar query. Only dividends with ex-dates on or before this date will be returned. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum
    outputsize: Option<i64>,
    /// Page number
    page: Option<i64>,
}

impl GetDividendsCalendarParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// Start date for the dividends calendar query. Only dividends with ex-dates on or after this date will be returned. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for the dividends calendar query. Only dividends with ex-dates on or before this date will be returned. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }
    /// Page number
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetDividendsCalendarParams {
        GetDividendsCalendarParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
            page: self.page,
        }
    }
}

/// struct for passing parameters to the method [`get_earnings`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetEarningsParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The asset class to which the instrument belongs
    pub r#type: Option<String>,
    /// Type of earning, returns only 1 record. When is not empty, dates and outputsize parameters are ignored
    pub period: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `1000`. Default `10` when no date parameters are set, otherwise set to maximum
    pub outputsize: Option<i64>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>,
    /// The date from which the data is requested. The date format is `YYYY-MM-DD`.
    pub start_date: Option<String>,
    /// The date to which the data is requested. The date format is `YYYY-MM-DD`.
    pub end_date: Option<String>,
    /// The number of decimal places in the response data. Should be in range [0,11] inclusive
    pub dp: Option<i64>,
}

impl GetEarningsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEarningsParamsBuilder {
        GetEarningsParamsBuilder::default()
    }
}

/// Builder for [`GetEarningsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEarningsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The asset class to which the instrument belongs
    r#type: Option<String>,
    /// Type of earning, returns only 1 record. When is not empty, dates and outputsize parameters are ignored
    period: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `1000`. Default `10` when no date parameters are set, otherwise set to maximum
    outputsize: Option<i64>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>,
    /// The date from which the data is requested. The date format is `YYYY-MM-DD`.
    start_date: Option<String>,
    /// The date to which the data is requested. The date format is `YYYY-MM-DD`.
    end_date: Option<String>,
    /// The number of decimal places in the response data. Should be in range [0,11] inclusive
    dp: Option<i64>,
}

impl GetEarningsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The asset class to which the instrument belongs
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    /// Type of earning, returns only 1 record. When is not empty, dates and outputsize parameters are ignored
    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = Some(period.into());
        self
    }
    /// Number of data points to retrieve. Supports values in the range from `1` to `1000`. Default `10` when no date parameters are set, otherwise set to maximum
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }
    /// The format of the response data
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
    /// The separator used in the CSV response data
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }
    /// The date from which the data is requested. The date format is `YYYY-MM-DD`.
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// The date to which the data is requested. The date format is `YYYY-MM-DD`.
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// The number of decimal places in the response data. Should be in range [0,11] inclusive
    pub fn dp(mut self, dp: i64) -> Self {
        self.dp = Some(dp);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEarningsParams {
        GetEarningsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            r#type: self.r#type,
            period: self.period,
            outputsize: self.outputsize,
            format: self.format,
            delimiter: self.delimiter,
            start_date: self.start_date,
            end_date: self.end_date,
            dp: self.dp,
        }
    }
}

/// struct for passing parameters to the method [`get_earnings_calendar`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetEarningsCalendarParams {
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Value can be JSON or CSV
    pub format: Option<String>,
    /// Specify the delimiter used when downloading the CSV file
    pub delimiter: Option<String>,
    /// Can be used separately and together with end_date. Format `2006-01-02` or `2006-01-02T15:04:05`
    pub start_date: Option<String>,
    /// Can be used separately and together with start_date. Format `2006-01-02` or `2006-01-02T15:04:05`
    pub end_date: Option<String>,
    /// Specifies the number of decimal places for floating values. Should be in range [0,11] inclusive
    pub dp: Option<i64>,
}

impl GetEarningsCalendarParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEarningsCalendarParamsBuilder {
        GetEarningsCalendarParamsBuilder::default()
    }
}

/// Builder for [`GetEarningsCalendarParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEarningsCalendarParamsBuilder {
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// Value can be JSON or CSV
    format: Option<String>,
    /// Specify the delimiter used when downloading the CSV file
    delimiter: Option<String>,
    /// Can be used separately and together with end_date. Format `2006-01-02` or `2006-01-02T15:04:05`
    start_date: Option<String>,
    /// Can be used separately and together with start_date. Format `2006-01-02` or `2006-01-02T15:04:05`
    end_date: Option<String>,
    /// Specifies the number of decimal places for floating values. Should be in range [0,11] inclusive
    dp: Option<i64>,
}

impl GetEarningsCalendarParamsBuilder {
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
    /// Value can be JSON or CSV
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
    /// Specify the delimiter used when downloading the CSV file
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }
    /// Can be used separately and together with end_date. Format `2006-01-02` or `2006-01-02T15:04:05`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// Can be used separately and together with start_date. Format `2006-01-02` or `2006-01-02T15:04:05`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Specifies the number of decimal places for floating values. Should be in range [0,11] inclusive
    pub fn dp(mut self, dp: i64) -> Self {
        self.dp = Some(dp);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEarningsCalendarParams {
        GetEarningsCalendarParams {
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            format: self.format,
            delimiter: self.delimiter,
            start_date: self.start_date,
            end_date: self.end_date,
            dp: self.dp,
        }
    }
}

/// struct for passing parameters to the method [`get_income_statement`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetIncomeStatementParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The reporting period for the income statement data
    pub period: Option<String>,
    /// Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetIncomeStatementParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetIncomeStatementParamsBuilder {
        GetIncomeStatementParamsBuilder::default()
    }
}

/// Builder for [`GetIncomeStatementParams`]
#[derive(Clone, Debug, Default)]
pub struct GetIncomeStatementParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The reporting period for the income statement data
    period: Option<String>,
    /// Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetIncomeStatementParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The reporting period for the income statement data
    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = Some(period.into());
        self
    }
    /// Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetIncomeStatementParams {
        GetIncomeStatementParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            period: self.period,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_income_statement_consolidated`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetIncomeStatementConsolidatedParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The reporting period for the income statement data
    pub period: Option<String>,
    /// Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetIncomeStatementConsolidatedParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetIncomeStatementConsolidatedParamsBuilder {
        GetIncomeStatementConsolidatedParamsBuilder::default()
    }
}

/// Builder for [`GetIncomeStatementConsolidatedParams`]
#[derive(Clone, Debug, Default)]
pub struct GetIncomeStatementConsolidatedParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The reporting period for the income statement data
    period: Option<String>,
    /// Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetIncomeStatementConsolidatedParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The reporting period for the income statement data
    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = Some(period.into());
        self
    }
    /// Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetIncomeStatementConsolidatedParams {
        GetIncomeStatementConsolidatedParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            period: self.period,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_ipo_calendar`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetIpoCalendarParams {
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The earliest IPO date to include in the results. Format: `2006-01-02`
    pub start_date: Option<String>,
    /// The latest IPO date to include in the results. Format: `2006-01-02`
    pub end_date: Option<String>,
}

impl GetIpoCalendarParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetIpoCalendarParamsBuilder {
        GetIpoCalendarParamsBuilder::default()
    }
}

/// Builder for [`GetIpoCalendarParams`]
#[derive(Clone, Debug, Default)]
pub struct GetIpoCalendarParamsBuilder {
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The earliest IPO date to include in the results. Format: `2006-01-02`
    start_date: Option<String>,
    /// The latest IPO date to include in the results. Format: `2006-01-02`
    end_date: Option<String>,
}

impl GetIpoCalendarParamsBuilder {
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
    /// The earliest IPO date to include in the results. Format: `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// The latest IPO date to include in the results. Format: `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetIpoCalendarParams {
        GetIpoCalendarParams {
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            start_date: self.start_date,
            end_date: self.end_date,
        }
    }
}

/// struct for passing parameters to the method [`get_key_executives`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetKeyExecutivesParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
}

impl GetKeyExecutivesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetKeyExecutivesParamsBuilder {
        GetKeyExecutivesParamsBuilder::default()
    }
}

/// Builder for [`GetKeyExecutivesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetKeyExecutivesParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
}

impl GetKeyExecutivesParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    pub fn build(self) -> GetKeyExecutivesParams {
        GetKeyExecutivesParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
        }
    }
}

/// struct for passing parameters to the method [`get_last_changes`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetLastChangesParams {
    /// Endpoint name
    pub endpoint: String,
    /// The starting date and time for data selection, in `2006-01-02T15:04:05` format
    pub start_date: Option<String>,
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Page number
    pub page: Option<i64>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetLastChangesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetLastChangesParamsBuilder {
        GetLastChangesParamsBuilder::default()
    }
}

/// Builder for [`GetLastChangesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetLastChangesParamsBuilder {
    /// Endpoint name
    endpoint: String,
    /// The starting date and time for data selection, in `2006-01-02T15:04:05` format
    start_date: Option<String>,
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Page number
    page: Option<i64>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetLastChangesParamsBuilder {
    /// Endpoint name
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }
    /// The starting date and time for data selection, in `2006-01-02T15:04:05` format
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// Filter by symbol
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
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
    /// Page number
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetLastChangesParams {
        GetLastChangesParams {
            endpoint: self.endpoint,
            start_date: self.start_date,
            symbol: self.symbol,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            page: self.page,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_logo`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetLogoParams {
    /// The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `BTC/USD`, `EUR/USD`.
    pub symbol: String,
    /// The exchange name where the instrument is traded, e.g., `NASDAQ`, `NSE`
    pub exchange: Option<String>,
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON`
    pub mic_code: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
}

impl GetLogoParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetLogoParamsBuilder {
        GetLogoParamsBuilder::default()
    }
}

/// Builder for [`GetLogoParams`]
#[derive(Clone, Debug, Default)]
pub struct GetLogoParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `BTC/USD`, `EUR/USD`.
    symbol: String,
    /// The exchange name where the instrument is traded, e.g., `NASDAQ`, `NSE`
    exchange: Option<String>,
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON`
    mic_code: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
}

impl GetLogoParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `BTC/USD`, `EUR/USD`.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }
    /// The exchange name where the instrument is traded, e.g., `NASDAQ`, `NSE`
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON`
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetLogoParams {
        GetLogoParams {
            symbol: self.symbol,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
        }
    }
}

/// struct for passing parameters to the method [`get_market_cap`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetMarketCapParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Start date for market capitalization data retrieval. Data will be returned from this date onwards. Format `2006-01-02`
    pub start_date: Option<String>,
    /// End date for market capitalization data retrieval. Data will be returned up to and including this date. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Page number
    pub page: Option<i64>,
    /// Number of records in response
    pub outputsize: Option<i64>,
}

impl GetMarketCapParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMarketCapParamsBuilder {
        GetMarketCapParamsBuilder::default()
    }
}

/// Builder for [`GetMarketCapParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMarketCapParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Start date for market capitalization data retrieval. Data will be returned from this date onwards. Format `2006-01-02`
    start_date: Option<String>,
    /// End date for market capitalization data retrieval. Data will be returned up to and including this date. Format `2006-01-02`
    end_date: Option<String>,
    /// Page number
    page: Option<i64>,
    /// Number of records in response
    outputsize: Option<i64>,
}

impl GetMarketCapParamsBuilder {
    /// Filter by symbol
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// Start date for market capitalization data retrieval. Data will be returned from this date onwards. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for market capitalization data retrieval. Data will be returned up to and including this date. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Page number
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    /// Number of records in response
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMarketCapParams {
        GetMarketCapParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            start_date: self.start_date,
            end_date: self.end_date,
            page: self.page,
            outputsize: self.outputsize,
        }
    }
}

/// struct for passing parameters to the method [`get_profile`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetProfileParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
}

impl GetProfileParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetProfileParamsBuilder {
        GetProfileParamsBuilder::default()
    }
}

/// Builder for [`GetProfileParams`]
#[derive(Clone, Debug, Default)]
pub struct GetProfileParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
}

impl GetProfileParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    pub fn build(self) -> GetProfileParams {
        GetProfileParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
        }
    }
}

/// struct for passing parameters to the method [`get_splits`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetSplitsParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Range of data to be returned
    pub range: Option<String>,
    /// The starting date for data selection. Format `2006-01-02`
    pub start_date: Option<String>,
    /// The ending date for data selection. Format `2006-01-02`
    pub end_date: Option<String>,
}

impl GetSplitsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetSplitsParamsBuilder {
        GetSplitsParamsBuilder::default()
    }
}

/// Builder for [`GetSplitsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetSplitsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// Range of data to be returned
    range: Option<String>,
    /// The starting date for data selection. Format `2006-01-02`
    start_date: Option<String>,
    /// The ending date for data selection. Format `2006-01-02`
    end_date: Option<String>,
}

impl GetSplitsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// Range of data to be returned
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }
    /// The starting date for data selection. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// The ending date for data selection. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetSplitsParams {
        GetSplitsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            range: self.range,
            start_date: self.start_date,
            end_date: self.end_date,
        }
    }
}

/// struct for passing parameters to the method [`get_splits_calendar`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetSplitsCalendarParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The starting date (inclusive) for filtering split events in the calendar. Format `2006-01-02`
    pub start_date: Option<String>,
    /// The ending date (inclusive) for filtering split events in the calendar. Format `2006-01-02`
    pub end_date: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum
    pub outputsize: Option<i64>,
    /// Page number
    pub page: Option<String>,
}

impl GetSplitsCalendarParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetSplitsCalendarParamsBuilder {
        GetSplitsCalendarParamsBuilder::default()
    }
}

/// Builder for [`GetSplitsCalendarParams`]
#[derive(Clone, Debug, Default)]
pub struct GetSplitsCalendarParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The starting date (inclusive) for filtering split events in the calendar. Format `2006-01-02`
    start_date: Option<String>,
    /// The ending date (inclusive) for filtering split events in the calendar. Format `2006-01-02`
    end_date: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum
    outputsize: Option<i64>,
    /// Page number
    page: Option<String>,
}

impl GetSplitsCalendarParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    /// The starting date (inclusive) for filtering split events in the calendar. Format `2006-01-02`
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// The ending date (inclusive) for filtering split events in the calendar. Format `2006-01-02`
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }
    /// Page number
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetSplitsCalendarParams {
        GetSplitsCalendarParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            start_date: self.start_date,
            end_date: self.end_date,
            outputsize: self.outputsize,
            page: self.page,
        }
    }
}

/// struct for passing parameters to the method [`get_statistics`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetStatisticsParams {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
}

impl GetStatisticsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetStatisticsParamsBuilder {
        GetStatisticsParamsBuilder::default()
    }
}

/// Builder for [`GetStatisticsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetStatisticsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country where instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
}

impl GetStatisticsParamsBuilder {
    /// Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan
    pub fn figi(mut self, figi: impl Into<String>) -> Self {
        self.figi = Some(figi.into());
        self
    }
    /// Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
    pub fn isin(mut self, isin: impl Into<String>) -> Self {
        self.isin = Some(isin.into());
        self
    }
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section
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
    pub fn build(self) -> GetStatisticsParams {
        GetStatisticsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
        }
    }
}

/// struct for typed errors of method [`get_balance_sheet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBalanceSheetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_balance_sheet_consolidated`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBalanceSheetConsolidatedError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cash_flow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCashFlowError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cash_flow_consolidated`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCashFlowConsolidatedError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dividends`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDividendsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dividends_calendar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDividendsCalendarError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_earnings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEarningsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_earnings_calendar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEarningsCalendarError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_income_statement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncomeStatementError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_income_statement_consolidated`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncomeStatementConsolidatedError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ipo_calendar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIpoCalendarError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_key_executives`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetKeyExecutivesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_last_changes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLastChangesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_logo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_market_cap`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMarketCapError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_profile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProfileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_splits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSplitsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_splits_calendar`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSplitsCalendarError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_statistics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStatisticsError {
    UnknownValue(serde_json::Value),
}

/// The balance sheet endpoint provides a detailed financial statement for a company, outlining its assets, liabilities, and shareholders' equity. This endpoint returns structured data that includes current and non-current assets, total liabilities, and equity figures, enabling users to assess a company's financial health and stability.
pub async fn get_balance_sheet(
    configuration: &configuration::Configuration,
    params: GetBalanceSheetParams,
) -> Result<models::GetBalanceSheetResponse, Error<GetBalanceSheetError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_period = params.period;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/balance_sheet", configuration.base_path);
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
    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetBalanceSheetResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBalanceSheetResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBalanceSheetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The balance sheet consolidated endpoint provides a detailed overview of a company's raw balance sheet, including a comprehensive summary of its assets, liabilities, and shareholders' equity. This endpoint is useful for retrieving financial data that reflects the overall financial position of a company, allowing users to access critical information about its financial health and structure.
pub async fn get_balance_sheet_consolidated(
    configuration: &configuration::Configuration,
    params: GetBalanceSheetConsolidatedParams,
) -> Result<models::GetBalanceSheetConsolidatedResponse, Error<GetBalanceSheetConsolidatedError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_period = params.period;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/balance_sheet/consolidated", configuration.base_path);
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
    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetBalanceSheetConsolidatedResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBalanceSheetConsolidatedResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBalanceSheetConsolidatedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The cash flow endpoint provides detailed information on a company's cash flow activities, including the net cash and cash equivalents moving in and out of the business. This data includes operating, investing, and financing cash flows, offering a comprehensive view of the company's liquidity and financial health.
pub async fn get_cash_flow(
    configuration: &configuration::Configuration,
    params: GetCashFlowParams,
) -> Result<models::GetCashFlowResponse, Error<GetCashFlowError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_period = params.period;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/cash_flow", configuration.base_path);
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
    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetCashFlowResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCashFlowResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCashFlowError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The cash flow consolidated endpoint provides raw data on a company's consolidated cash flow, including the net cash and cash equivalents moving in and out of the business. It returns information on operating, investing, and financing activities, helping users track liquidity and financial health over a specified period.
pub async fn get_cash_flow_consolidated(
    configuration: &configuration::Configuration,
    params: GetCashFlowConsolidatedParams,
) -> Result<models::GetCashFlowConsolidatedResponse, Error<GetCashFlowConsolidatedError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_period = params.period;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/cash_flow/consolidated", configuration.base_path);
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
    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetCashFlowConsolidatedResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCashFlowConsolidatedResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCashFlowConsolidatedError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The dividends endpoint provides historical dividend data for a specified stock, in many cases covering over a decade. It returns information on dividend payouts, including the amount, payment date, and frequency. This endpoint is ideal for users tracking dividend histories or evaluating the income potential of stocks.
pub async fn get_dividends(
    configuration: &configuration::Configuration,
    params: GetDividendsParams,
) -> Result<models::GetDividendsResponse, Error<GetDividendsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_range = params.range;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_adjust = params.adjust;

    let uri_str = format!("{}/dividends", configuration.base_path);
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
    if let Some(ref param_value) = p_query_range {
        req_builder = req_builder.query(&[("range", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_adjust {
        req_builder = req_builder.query(&[("adjust", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetDividendsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetDividendsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDividendsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The dividends calendar endpoint provides a detailed schedule of upcoming and past dividend events for specified date ranges. By using the `start_date` and `end_date` parameters, users can retrieve a list of companies issuing dividends, including the ex-dividend date, payment date, and dividend amount. This endpoint is ideal for tracking dividend payouts and planning investment strategies based on dividend schedules.
pub async fn get_dividends_calendar(
    configuration: &configuration::Configuration,
    params: GetDividendsCalendarParams,
) -> Result<models::GetDividendsCalendarResponse, Error<GetDividendsCalendarError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;
    let p_query_page = params.page;

    let uri_str = format!("{}/dividends_calendar", configuration.base_path);
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
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetDividendsCalendarResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetDividendsCalendarResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDividendsCalendarError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The earnings endpoint provides comprehensive earnings data for a specified company, including both the estimated and actual Earnings Per Share (EPS) figures. This endpoint delivers historical earnings information, allowing users to track a company's financial performance over time.
pub async fn get_earnings(
    configuration: &configuration::Configuration,
    params: GetEarningsParams,
) -> Result<models::GetEarningsResponse, Error<GetEarningsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_type = params.r#type;
    let p_query_period = params.period;
    let p_query_outputsize = params.outputsize;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/earnings", configuration.base_path);
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
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Ok(models::GetEarningsResponse::Text(content)),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEarningsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEarningsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The earnings calendar endpoint provides a schedule of company earnings announcements for a specified date range. By default, it returns earnings data for the current day. Users can customize the date range using the `start_date` and `end_date` parameters to retrieve earnings information for specific periods. This endpoint is useful for tracking upcoming earnings reports and planning around key financial announcements.
pub async fn get_earnings_calendar(
    configuration: &configuration::Configuration,
    params: GetEarningsCalendarParams,
) -> Result<models::GetEarningsCalendarResponse, Error<GetEarningsCalendarError>> {
    // Extract parameters from params struct
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_dp = params.dp;

    let uri_str = format!("{}/earnings_calendar", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Ok(models::GetEarningsCalendarResponse::Text(content)),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEarningsCalendarResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEarningsCalendarError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The income statement endpoint provides detailed financial data on a company's income statement, including revenues, expenses, and net income for specified periods, either annually or quarterly. This endpoint is essential for retrieving comprehensive financial performance metrics of a company, allowing users to access historical and current financial results.
pub async fn get_income_statement(
    configuration: &configuration::Configuration,
    params: GetIncomeStatementParams,
) -> Result<models::GetIncomeStatementResponse, Error<GetIncomeStatementError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_period = params.period;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/income_statement", configuration.base_path);
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
    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetIncomeStatementResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetIncomeStatementResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIncomeStatementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The income statement consolidated endpoint provides a company's raw income statement, detailing revenue, expenses, and net income for specified periods, either annually or quarterly. This data is essential for evaluating a company's financial performance over time, allowing users to access comprehensive financial results in a structured format.
pub async fn get_income_statement_consolidated(
    configuration: &configuration::Configuration,
    params: GetIncomeStatementConsolidatedParams,
) -> Result<
    models::GetIncomeStatementConsolidatedResponse,
    Error<GetIncomeStatementConsolidatedError>,
> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_period = params.period;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/income_statement/consolidated", configuration.base_path);
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
    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetIncomeStatementConsolidatedResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetIncomeStatementConsolidatedResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIncomeStatementConsolidatedError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The IPO Calendar endpoint provides detailed information on initial public offerings (IPOs), including those that have occurred in the past, are happening today, or are scheduled for the future. Users can access data such as company names, IPO dates, and offering details, allowing them to track and monitor IPO activity efficiently.
pub async fn get_ipo_calendar(
    configuration: &configuration::Configuration,
    params: GetIpoCalendarParams,
) -> Result<models::GetIpoCalendarResponse, Error<GetIpoCalendarError>> {
    // Extract parameters from params struct
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;

    let uri_str = format!("{}/ipo_calendar", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetIpoCalendarResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetIpoCalendarResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIpoCalendarError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The key executives endpoint provides detailed information about a company's key executives identified by a specific stock symbol. It returns data such as names, titles, and roles of the executives, which can be useful for understanding the leadership structure of the company.
pub async fn get_key_executives(
    configuration: &configuration::Configuration,
    params: GetKeyExecutivesParams,
) -> Result<models::GetKeyExecutivesResponse, Error<GetKeyExecutivesError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/key_executives", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetKeyExecutivesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetKeyExecutivesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetKeyExecutivesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The last change endpoint provides the most recent updates to fundamental data for a specified dataset. It returns a timestamp indicating when the data was last modified, allowing users to efficiently manage API requests by only fetching new data when changes occur. This helps optimize data retrieval and reduce unnecessary API credit usage.
pub async fn get_last_changes(
    configuration: &configuration::Configuration,
    params: GetLastChangesParams,
) -> Result<models::GetLastChangesResponse, Error<GetLastChangesError>> {
    // Extract parameters from params struct
    let p_path_endpoint = params.endpoint;
    let p_query_start_date = params.start_date;
    let p_query_symbol = params.symbol;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_page = params.page;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!(
        "{}/last_change/{endpoint}",
        configuration.base_path,
        endpoint = crate::apis::urlencode(p_path_endpoint)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
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
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetLastChangesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetLastChangesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetLastChangesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The logo endpoint provides the official logo image for a specified company, cryptocurrency, or forex pair. This endpoint is useful for integrating visual branding elements into financial applications, websites, or reports, ensuring that users can easily identify and associate the correct logo with the respective financial asset.
pub async fn get_logo(
    configuration: &configuration::Configuration,
    params: GetLogoParams,
) -> Result<models::GetLogoResponse, Error<GetLogoError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/logo", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_query_symbol.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetLogoResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetLogoResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetLogoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The Market Capitalization History endpoint provides historical data on a company's market capitalization over a specified time period. It returns a time series of market cap values, allowing users to track changes in a company's market value.
pub async fn get_market_cap(
    configuration: &configuration::Configuration,
    params: GetMarketCapParams,
) -> Result<models::GetMarketCapResponse, Error<GetMarketCapError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_page = params.page;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/market_cap", configuration.base_path);
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
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetMarketCapResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMarketCapResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMarketCapError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The profile endpoint provides detailed company information, including the company's name, industry, sector, CEO, headquarters location, and market capitalization. This data is useful for obtaining a comprehensive overview of a company's business and financial standing.
pub async fn get_profile(
    configuration: &configuration::Configuration,
    params: GetProfileParams,
) -> Result<models::GetProfileResponse, Error<GetProfileError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/profile", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetProfileResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetProfileResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetProfileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The splits endpoint provides historical data on stock split events for a specified company. It returns details including the date of each split and the corresponding split factor.
pub async fn get_splits(
    configuration: &configuration::Configuration,
    params: GetSplitsParams,
) -> Result<models::GetSplitsResponse, Error<GetSplitsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_range = params.range;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;

    let uri_str = format!("{}/splits", configuration.base_path);
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
    if let Some(ref param_value) = p_query_range {
        req_builder = req_builder.query(&[("range", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetSplitsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetSplitsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSplitsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The splits calendar endpoint provides a detailed calendar of stock split events within a specified date range. By setting the `start_date` and `end_date` parameters, users can retrieve a list of upcoming or past stock splits, including the company name, split ratio, and effective date. This endpoint is useful for tracking changes in stock structure and planning investment strategies around these events.
pub async fn get_splits_calendar(
    configuration: &configuration::Configuration,
    params: GetSplitsCalendarParams,
) -> Result<models::GetSplitsCalendarResponse, Error<GetSplitsCalendarError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_outputsize = params.outputsize;
    let p_query_page = params.page;

    let uri_str = format!("{}/splits_calendar", configuration.base_path);
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
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetSplitsCalendarResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetSplitsCalendarResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSplitsCalendarError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The statistics endpoint provides a comprehensive snapshot of a company's key financial statistics, including valuation metrics, revenue figures, profit margins, and other essential financial data. This endpoint is ideal for users seeking detailed insights into a company's financial health and performance metrics.
pub async fn get_statistics(
    configuration: &configuration::Configuration,
    params: GetStatisticsParams,
) -> Result<models::GetStatisticsResponse, Error<GetStatisticsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/statistics", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/*` content type response that cannot be converted to `models::GetStatisticsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetStatisticsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetStatisticsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
