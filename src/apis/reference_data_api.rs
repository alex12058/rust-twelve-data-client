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

/// struct for passing parameters to the method [`get_bonds`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetBondsParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    pub show_plan: Option<bool>,
    /// Page number of the results to fetch
    pub page: Option<i64>,
    /// Determines the number of data points returned in the output
    pub outputsize: Option<i64>
}

impl GetBondsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetBondsParamsBuilder {
        GetBondsParamsBuilder::default()
    }
}

/// Builder for [`GetBondsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetBondsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    show_plan: Option<bool>,
    /// Page number of the results to fetch
    page: Option<i64>,
    /// Determines the number of data points returned in the output
    outputsize: Option<i64>
}

impl GetBondsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
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
    /// Adds info on which plan symbol is available
    pub fn show_plan(mut self, show_plan: bool) -> Self {
        self.show_plan = Some(show_plan);
        self
    }
    /// Page number of the results to fetch
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    /// Determines the number of data points returned in the output
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetBondsParams {
        GetBondsParams {
            symbol: self.symbol,
            exchange: self.exchange,
            country: self.country,
            format: self.format,
            delimiter: self.delimiter,
            show_plan: self.show_plan,
            page: self.page,
            outputsize: self.outputsize
        }
    }
}

/// struct for passing parameters to the method [`get_commodities`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetCommoditiesParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by category of commodity
    pub category: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>
}

impl GetCommoditiesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetCommoditiesParamsBuilder {
        GetCommoditiesParamsBuilder::default()
    }
}

/// Builder for [`GetCommoditiesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetCommoditiesParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by category of commodity
    category: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>
}

impl GetCommoditiesParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by category of commodity
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
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

    /// Build the parameter struct
    pub fn build(self) -> GetCommoditiesParams {
        GetCommoditiesParams {
            symbol: self.symbol,
            category: self.category,
            format: self.format,
            delimiter: self.delimiter
        }
    }
}

/// struct for passing parameters to the method [`get_cross_listings`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetCrossListingsParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: String,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market identifier code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Country to which stock exchange belongs to
    pub country: Option<String>
}

impl GetCrossListingsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetCrossListingsParamsBuilder {
        GetCrossListingsParamsBuilder::default()
    }
}

/// Builder for [`GetCrossListingsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetCrossListingsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: String,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market identifier code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Country to which stock exchange belongs to
    country: Option<String>
}

impl GetCrossListingsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }
    /// Exchange where instrument is traded
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Market identifier code (MIC) under ISO 10383 standard
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// Country to which stock exchange belongs to
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetCrossListingsParams {
        GetCrossListingsParams {
            symbol: self.symbol,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_cryptocurrencies`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetCryptocurrenciesParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by exchange name. E.g. `Binance`, `Coinbase`, etc.
    pub exchange: Option<String>,
    /// Filter by currency base
    pub currency_base: Option<String>,
    /// Filter by currency quote
    pub currency_quote: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>
}

impl GetCryptocurrenciesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetCryptocurrenciesParamsBuilder {
        GetCryptocurrenciesParamsBuilder::default()
    }
}

/// Builder for [`GetCryptocurrenciesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetCryptocurrenciesParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by exchange name. E.g. `Binance`, `Coinbase`, etc.
    exchange: Option<String>,
    /// Filter by currency base
    currency_base: Option<String>,
    /// Filter by currency quote
    currency_quote: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>
}

impl GetCryptocurrenciesParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by exchange name. E.g. `Binance`, `Coinbase`, etc.
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Filter by currency base
    pub fn currency_base(mut self, currency_base: impl Into<String>) -> Self {
        self.currency_base = Some(currency_base.into());
        self
    }
    /// Filter by currency quote
    pub fn currency_quote(mut self, currency_quote: impl Into<String>) -> Self {
        self.currency_quote = Some(currency_quote.into());
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

    /// Build the parameter struct
    pub fn build(self) -> GetCryptocurrenciesParams {
        GetCryptocurrenciesParams {
            symbol: self.symbol,
            exchange: self.exchange,
            currency_base: self.currency_base,
            currency_quote: self.currency_quote,
            format: self.format,
            delimiter: self.delimiter
        }
    }
}

/// struct for passing parameters to the method [`get_cryptocurrency_exchanges`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetCryptocurrencyExchangesParams {
    /// The format of the response data
    pub format: Option<String>,
    /// Specify the delimiter used when downloading the CSV file
    pub delimiter: Option<String>
}

impl GetCryptocurrencyExchangesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetCryptocurrencyExchangesParamsBuilder {
        GetCryptocurrencyExchangesParamsBuilder::default()
    }
}

/// Builder for [`GetCryptocurrencyExchangesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetCryptocurrencyExchangesParamsBuilder {
    /// The format of the response data
    format: Option<String>,
    /// Specify the delimiter used when downloading the CSV file
    delimiter: Option<String>
}

impl GetCryptocurrencyExchangesParamsBuilder {
    /// The format of the response data
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
    /// Specify the delimiter used when downloading the CSV file
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetCryptocurrencyExchangesParams {
        GetCryptocurrencyExchangesParams {
            format: self.format,
            delimiter: self.delimiter
        }
    }
}

/// struct for passing parameters to the method [`get_earliest_timestamp`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetEarliestTimestampParams {
    /// Interval between two consecutive points in time series.
    pub interval: String,
    /// Symbol ticker of the instrument.
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI).
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Exchange where instrument is traded.
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard.
    pub mic_code: Option<String>,
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    pub timezone: Option<String>
}

impl GetEarliestTimestampParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEarliestTimestampParamsBuilder {
        GetEarliestTimestampParamsBuilder::default()
    }
}

/// Builder for [`GetEarliestTimestampParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEarliestTimestampParamsBuilder {
    /// Interval between two consecutive points in time series.
    interval: String,
    /// Symbol ticker of the instrument.
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI).
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Exchange where instrument is traded.
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard.
    mic_code: Option<String>,
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    timezone: Option<String>
}

impl GetEarliestTimestampParamsBuilder {
    /// Interval between two consecutive points in time series.
    pub fn interval(mut self, interval: impl Into<String>) -> Self {
        self.interval = interval.into();
        self
    }
    /// Symbol ticker of the instrument.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by financial instrument global identifier (FIGI).
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
    /// Exchange where instrument is traded.
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Market Identifier Code (MIC) under ISO 10383 standard.
    pub fn mic_code(mut self, mic_code: impl Into<String>) -> Self {
        self.mic_code = Some(mic_code.into());
        self
    }
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEarliestTimestampParams {
        GetEarliestTimestampParams {
            interval: self.interval,
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            exchange: self.exchange,
            mic_code: self.mic_code,
            timezone: self.timezone
        }
    }
}

/// struct for passing parameters to the method [`get_etf`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetEtfParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    pub cik: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    pub show_plan: Option<bool>,
    /// Include delisted identifiers
    pub include_delisted: Option<bool>
}

impl GetEtfParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEtfParamsBuilder {
        GetEtfParamsBuilder::default()
    }
}

/// Builder for [`GetEtfParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEtfParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    cik: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    show_plan: Option<bool>,
    /// Include delisted identifiers
    include_delisted: Option<bool>
}

impl GetEtfParamsBuilder {
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
    /// The CIK of an instrument for which data is requested
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
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
    /// Adds info on which plan symbol is available
    pub fn show_plan(mut self, show_plan: bool) -> Self {
        self.show_plan = Some(show_plan);
        self
    }
    /// Include delisted identifiers
    pub fn include_delisted(mut self, include_delisted: bool) -> Self {
        self.include_delisted = Some(include_delisted);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEtfParams {
        GetEtfParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            cik: self.cik,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            format: self.format,
            delimiter: self.delimiter,
            show_plan: self.show_plan,
            include_delisted: self.include_delisted
        }
    }
}

/// struct for passing parameters to the method [`get_etfs_family`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetEtfsFamilyParams {
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by investment company that manages the fund
    pub fund_family: Option<String>
}

impl GetEtfsFamilyParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEtfsFamilyParamsBuilder {
        GetEtfsFamilyParamsBuilder::default()
    }
}

/// Builder for [`GetEtfsFamilyParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEtfsFamilyParamsBuilder {
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by investment company that manages the fund
    fund_family: Option<String>
}

impl GetEtfsFamilyParamsBuilder {
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// Filter by investment company that manages the fund
    pub fn fund_family(mut self, fund_family: impl Into<String>) -> Self {
        self.fund_family = Some(fund_family.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEtfsFamilyParams {
        GetEtfsFamilyParams {
            country: self.country,
            fund_family: self.fund_family
        }
    }
}

/// struct for passing parameters to the method [`get_etfs_list`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetEtfsListParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    pub cik: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by investment company that manages the fund
    pub fund_family: Option<String>,
    /// Filter by the type of fund
    pub fund_type: Option<String>,
    /// Page number
    pub page: Option<i64>,
    /// Number of records in response
    pub outputsize: Option<i64>
}

impl GetEtfsListParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEtfsListParamsBuilder {
        GetEtfsListParamsBuilder::default()
    }
}

/// Builder for [`GetEtfsListParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEtfsListParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    cik: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by investment company that manages the fund
    fund_family: Option<String>,
    /// Filter by the type of fund
    fund_type: Option<String>,
    /// Page number
    page: Option<i64>,
    /// Number of records in response
    outputsize: Option<i64>
}

impl GetEtfsListParamsBuilder {
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
    /// The CIK of an instrument for which data is requested
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// Filter by investment company that manages the fund
    pub fn fund_family(mut self, fund_family: impl Into<String>) -> Self {
        self.fund_family = Some(fund_family.into());
        self
    }
    /// Filter by the type of fund
    pub fn fund_type(mut self, fund_type: impl Into<String>) -> Self {
        self.fund_type = Some(fund_type.into());
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
    pub fn build(self) -> GetEtfsListParams {
        GetEtfsListParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            cik: self.cik,
            country: self.country,
            fund_family: self.fund_family,
            fund_type: self.fund_type,
            page: self.page,
            outputsize: self.outputsize
        }
    }
}

/// struct for passing parameters to the method [`get_etfs_type`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetEtfsTypeParams {
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by the type of fund
    pub fund_type: Option<String>
}

impl GetEtfsTypeParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetEtfsTypeParamsBuilder {
        GetEtfsTypeParamsBuilder::default()
    }
}

/// Builder for [`GetEtfsTypeParams`]
#[derive(Clone, Debug, Default)]
pub struct GetEtfsTypeParamsBuilder {
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by the type of fund
    fund_type: Option<String>
}

impl GetEtfsTypeParamsBuilder {
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// Filter by the type of fund
    pub fn fund_type(mut self, fund_type: impl Into<String>) -> Self {
        self.fund_type = Some(fund_type.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetEtfsTypeParams {
        GetEtfsTypeParams {
            country: self.country,
            fund_type: self.fund_type
        }
    }
}

/// struct for passing parameters to the method [`get_exchange_schedule`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetExchangeScheduleParams {
    /// Filter by exchange name
    pub mic_name: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// <p> If a date is provided, the API returns the schedule for the specified date; otherwise, it returns the default (common) schedule. </p> The date can be specified in one of the following formats: <ul> <li>An exact date (e.g., <code>2021-10-27</code>)</li> <li>A human-readable keyword: <code>today</code> or <code>yesterday</code></li> <li>A full datetime string in UTC (e.g., <code>2025-04-11T20:00:00</code>) to retrieve the schedule corresponding to the day in the specified time.</li> </ul> When using a datetime value, the resulting schedule will correspond to the local calendar day at the specified time. For example, <code>2025-04-11T20:00:00 UTC</code> corresponds to: <ul> <li><code>2025-04-11</code> in the <code>America/New_York</code> timezone</li> <li><code>2025-04-12</code> in the <code>Australia/Sydney</code> timezone</li> </ul>
    pub date: Option<String>
}

impl GetExchangeScheduleParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetExchangeScheduleParamsBuilder {
        GetExchangeScheduleParamsBuilder::default()
    }
}

/// Builder for [`GetExchangeScheduleParams`]
#[derive(Clone, Debug, Default)]
pub struct GetExchangeScheduleParamsBuilder {
    /// Filter by exchange name
    mic_name: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// <p> If a date is provided, the API returns the schedule for the specified date; otherwise, it returns the default (common) schedule. </p> The date can be specified in one of the following formats: <ul> <li>An exact date (e.g., <code>2021-10-27</code>)</li> <li>A human-readable keyword: <code>today</code> or <code>yesterday</code></li> <li>A full datetime string in UTC (e.g., <code>2025-04-11T20:00:00</code>) to retrieve the schedule corresponding to the day in the specified time.</li> </ul> When using a datetime value, the resulting schedule will correspond to the local calendar day at the specified time. For example, <code>2025-04-11T20:00:00 UTC</code> corresponds to: <ul> <li><code>2025-04-11</code> in the <code>America/New_York</code> timezone</li> <li><code>2025-04-12</code> in the <code>Australia/Sydney</code> timezone</li> </ul>
    date: Option<String>
}

impl GetExchangeScheduleParamsBuilder {
    /// Filter by exchange name
    pub fn mic_name(mut self, mic_name: impl Into<String>) -> Self {
        self.mic_name = Some(mic_name.into());
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
    /// <p> If a date is provided, the API returns the schedule for the specified date; otherwise, it returns the default (common) schedule. </p> The date can be specified in one of the following formats: <ul> <li>An exact date (e.g., <code>2021-10-27</code>)</li> <li>A human-readable keyword: <code>today</code> or <code>yesterday</code></li> <li>A full datetime string in UTC (e.g., <code>2025-04-11T20:00:00</code>) to retrieve the schedule corresponding to the day in the specified time.</li> </ul> When using a datetime value, the resulting schedule will correspond to the local calendar day at the specified time. For example, <code>2025-04-11T20:00:00 UTC</code> corresponds to: <ul> <li><code>2025-04-11</code> in the <code>America/New_York</code> timezone</li> <li><code>2025-04-12</code> in the <code>Australia/Sydney</code> timezone</li> </ul>
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetExchangeScheduleParams {
        GetExchangeScheduleParams {
            mic_name: self.mic_name,
            mic_code: self.mic_code,
            country: self.country,
            date: self.date
        }
    }
}

/// struct for passing parameters to the method [`get_exchanges`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetExchangesParams {
    /// The asset class to which the instrument belongs
    pub r#type: Option<String>,
    /// Filter by exchange name
    pub name: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    pub show_plan: Option<bool>
}

impl GetExchangesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetExchangesParamsBuilder {
        GetExchangesParamsBuilder::default()
    }
}

/// Builder for [`GetExchangesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetExchangesParamsBuilder {
    /// The asset class to which the instrument belongs
    r#type: Option<String>,
    /// Filter by exchange name
    name: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    show_plan: Option<bool>
}

impl GetExchangesParamsBuilder {
    /// The asset class to which the instrument belongs
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    /// Filter by exchange name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
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
    /// Adds info on which plan symbol is available
    pub fn show_plan(mut self, show_plan: bool) -> Self {
        self.show_plan = Some(show_plan);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetExchangesParams {
        GetExchangesParams {
            r#type: self.r#type,
            name: self.name,
            code: self.code,
            country: self.country,
            format: self.format,
            delimiter: self.delimiter,
            show_plan: self.show_plan
        }
    }
}

/// struct for passing parameters to the method [`get_forex_pairs`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetForexPairsParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by currency base
    pub currency_base: Option<String>,
    /// Filter by currency quote
    pub currency_quote: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>
}

impl GetForexPairsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetForexPairsParamsBuilder {
        GetForexPairsParamsBuilder::default()
    }
}

/// Builder for [`GetForexPairsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetForexPairsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by currency base
    currency_base: Option<String>,
    /// Filter by currency quote
    currency_quote: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>
}

impl GetForexPairsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by currency base
    pub fn currency_base(mut self, currency_base: impl Into<String>) -> Self {
        self.currency_base = Some(currency_base.into());
        self
    }
    /// Filter by currency quote
    pub fn currency_quote(mut self, currency_quote: impl Into<String>) -> Self {
        self.currency_quote = Some(currency_quote.into());
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

    /// Build the parameter struct
    pub fn build(self) -> GetForexPairsParams {
        GetForexPairsParams {
            symbol: self.symbol,
            currency_base: self.currency_base,
            currency_quote: self.currency_quote,
            format: self.format,
            delimiter: self.delimiter
        }
    }
}

/// struct for passing parameters to the method [`get_funds`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetFundsParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    pub cik: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    pub show_plan: Option<bool>,
    /// Page number of the results to fetch
    pub page: Option<i64>,
    /// Determines the number of data points returned in the output
    pub outputsize: Option<i64>
}

impl GetFundsParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetFundsParamsBuilder {
        GetFundsParamsBuilder::default()
    }
}

/// Builder for [`GetFundsParams`]
#[derive(Clone, Debug, Default)]
pub struct GetFundsParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    cik: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    show_plan: Option<bool>,
    /// Page number of the results to fetch
    page: Option<i64>,
    /// Determines the number of data points returned in the output
    outputsize: Option<i64>
}

impl GetFundsParamsBuilder {
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
    /// The CIK of an instrument for which data is requested
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }
    /// Filter by exchange name
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
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
    /// Adds info on which plan symbol is available
    pub fn show_plan(mut self, show_plan: bool) -> Self {
        self.show_plan = Some(show_plan);
        self
    }
    /// Page number of the results to fetch
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    /// Determines the number of data points returned in the output
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetFundsParams {
        GetFundsParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            cik: self.cik,
            exchange: self.exchange,
            country: self.country,
            format: self.format,
            delimiter: self.delimiter,
            show_plan: self.show_plan,
            page: self.page,
            outputsize: self.outputsize
        }
    }
}

/// struct for passing parameters to the method [`get_market_state`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetMarketStateParams {
    /// The exchange name where the instrument is traded.
    pub exchange: Option<String>,
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded.
    pub code: Option<String>,
    /// The country where the exchange is located. Takes country name or alpha code.
    pub country: Option<String>
}

impl GetMarketStateParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMarketStateParamsBuilder {
        GetMarketStateParamsBuilder::default()
    }
}

/// Builder for [`GetMarketStateParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMarketStateParamsBuilder {
    /// The exchange name where the instrument is traded.
    exchange: Option<String>,
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded.
    code: Option<String>,
    /// The country where the exchange is located. Takes country name or alpha code.
    country: Option<String>
}

impl GetMarketStateParamsBuilder {
    /// The exchange name where the instrument is traded.
    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }
    /// The Market Identifier Code (MIC) of the exchange where the instrument is traded.
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    /// The country where the exchange is located. Takes country name or alpha code.
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMarketStateParams {
        GetMarketStateParams {
            exchange: self.exchange,
            code: self.code,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_family`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetMutualFundsFamilyParams {
    /// Filter by investment company that manages the fund
    pub fund_family: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>
}

impl GetMutualFundsFamilyParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsFamilyParamsBuilder {
        GetMutualFundsFamilyParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsFamilyParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsFamilyParamsBuilder {
    /// Filter by investment company that manages the fund
    fund_family: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>
}

impl GetMutualFundsFamilyParamsBuilder {
    /// Filter by investment company that manages the fund
    pub fn fund_family(mut self, fund_family: impl Into<String>) -> Self {
        self.fund_family = Some(fund_family.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsFamilyParams {
        GetMutualFundsFamilyParams {
            fund_family: self.fund_family,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_list`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetMutualFundsListParams {
    /// Filter by symbol
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    pub cik: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// Filter by investment company that manages the fund
    pub fund_family: Option<String>,
    /// Filter by the type of fund
    pub fund_type: Option<String>,
    /// Filter by performance rating from `0` to `5`
    pub performance_rating: Option<i64>,
    /// Filter by risk rating from `0` to `5`
    pub risk_rating: Option<i64>,
    /// Page number
    pub page: Option<i64>,
    /// Number of records in response
    pub outputsize: Option<i64>
}

impl GetMutualFundsListParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsListParamsBuilder {
        GetMutualFundsListParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsListParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsListParamsBuilder {
    /// Filter by symbol
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    cik: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// Filter by investment company that manages the fund
    fund_family: Option<String>,
    /// Filter by the type of fund
    fund_type: Option<String>,
    /// Filter by performance rating from `0` to `5`
    performance_rating: Option<i64>,
    /// Filter by risk rating from `0` to `5`
    risk_rating: Option<i64>,
    /// Page number
    page: Option<i64>,
    /// Number of records in response
    outputsize: Option<i64>
}

impl GetMutualFundsListParamsBuilder {
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
    /// The CIK of an instrument for which data is requested
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// Filter by investment company that manages the fund
    pub fn fund_family(mut self, fund_family: impl Into<String>) -> Self {
        self.fund_family = Some(fund_family.into());
        self
    }
    /// Filter by the type of fund
    pub fn fund_type(mut self, fund_type: impl Into<String>) -> Self {
        self.fund_type = Some(fund_type.into());
        self
    }
    /// Filter by performance rating from `0` to `5`
    pub fn performance_rating(mut self, performance_rating: i64) -> Self {
        self.performance_rating = Some(performance_rating);
        self
    }
    /// Filter by risk rating from `0` to `5`
    pub fn risk_rating(mut self, risk_rating: i64) -> Self {
        self.risk_rating = Some(risk_rating);
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
    pub fn build(self) -> GetMutualFundsListParams {
        GetMutualFundsListParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            cik: self.cik,
            country: self.country,
            fund_family: self.fund_family,
            fund_type: self.fund_type,
            performance_rating: self.performance_rating,
            risk_rating: self.risk_rating,
            page: self.page,
            outputsize: self.outputsize
        }
    }
}

/// struct for passing parameters to the method [`get_mutual_funds_type`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetMutualFundsTypeParams {
    /// Filter by the type of fund
    pub fund_type: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>
}

impl GetMutualFundsTypeParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetMutualFundsTypeParamsBuilder {
        GetMutualFundsTypeParamsBuilder::default()
    }
}

/// Builder for [`GetMutualFundsTypeParams`]
#[derive(Clone, Debug, Default)]
pub struct GetMutualFundsTypeParamsBuilder {
    /// Filter by the type of fund
    fund_type: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>
}

impl GetMutualFundsTypeParamsBuilder {
    /// Filter by the type of fund
    pub fn fund_type(mut self, fund_type: impl Into<String>) -> Self {
        self.fund_type = Some(fund_type.into());
        self
    }
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetMutualFundsTypeParams {
        GetMutualFundsTypeParams {
            fund_type: self.fund_type,
            country: self.country
        }
    }
}

/// struct for passing parameters to the method [`get_stocks`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetStocksParams {
    /// The ticker symbol of an instrument for which data is requested
    pub symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    pub figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    pub cik: Option<String>,
    /// Filter by exchange name
    pub exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The asset class to which the instrument belongs
    pub r#type: Option<String>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    pub show_plan: Option<bool>,
    /// Include delisted identifiers
    pub include_delisted: Option<bool>
}

impl GetStocksParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetStocksParamsBuilder {
        GetStocksParamsBuilder::default()
    }
}

/// Builder for [`GetStocksParams`]
#[derive(Clone, Debug, Default)]
pub struct GetStocksParamsBuilder {
    /// The ticker symbol of an instrument for which data is requested
    symbol: Option<String>,
    /// Filter by financial instrument global identifier (FIGI)
    figi: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// The CIK of an instrument for which data is requested
    cik: Option<String>,
    /// Filter by exchange name
    exchange: Option<String>,
    /// Filter by market identifier code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// Filter by country name or alpha code, e.g., `United States` or `US`
    country: Option<String>,
    /// The asset class to which the instrument belongs
    r#type: Option<String>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>,
    /// Adds info on which plan symbol is available
    show_plan: Option<bool>,
    /// Include delisted identifiers
    include_delisted: Option<bool>
}

impl GetStocksParamsBuilder {
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
    /// The CIK of an instrument for which data is requested
    pub fn cik(mut self, cik: impl Into<String>) -> Self {
        self.cik = Some(cik.into());
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
    /// The asset class to which the instrument belongs
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
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
    /// Adds info on which plan symbol is available
    pub fn show_plan(mut self, show_plan: bool) -> Self {
        self.show_plan = Some(show_plan);
        self
    }
    /// Include delisted identifiers
    pub fn include_delisted(mut self, include_delisted: bool) -> Self {
        self.include_delisted = Some(include_delisted);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetStocksParams {
        GetStocksParams {
            symbol: self.symbol,
            figi: self.figi,
            isin: self.isin,
            cusip: self.cusip,
            cik: self.cik,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            r#type: self.r#type,
            format: self.format,
            delimiter: self.delimiter,
            show_plan: self.show_plan,
            include_delisted: self.include_delisted
        }
    }
}

/// struct for passing parameters to the method [`get_symbol_search`]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GetSymbolSearchParams {
    /// Symbol to search. Supports: <ul> <li>Ticker symbol of instrument.</li> <li>International securities identification number (ISIN). <li>Financial instrument global identifier (FIGI). <li>Composite FIGI.</li> <li>Share Class FIGI.</li> </ul>
    pub symbol: String,
    /// Number of matches in response. Max <code>120</code>
    pub outputsize: Option<i64>,
    /// Adds info on which plan symbol is available.
    pub show_plan: Option<bool>
}

impl GetSymbolSearchParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetSymbolSearchParamsBuilder {
        GetSymbolSearchParamsBuilder::default()
    }
}

/// Builder for [`GetSymbolSearchParams`]
#[derive(Clone, Debug, Default)]
pub struct GetSymbolSearchParamsBuilder {
    /// Symbol to search. Supports: <ul> <li>Ticker symbol of instrument.</li> <li>International securities identification number (ISIN). <li>Financial instrument global identifier (FIGI). <li>Composite FIGI.</li> <li>Share Class FIGI.</li> </ul>
    symbol: String,
    /// Number of matches in response. Max <code>120</code>
    outputsize: Option<i64>,
    /// Adds info on which plan symbol is available.
    show_plan: Option<bool>
}

impl GetSymbolSearchParamsBuilder {
    /// Symbol to search. Supports: <ul> <li>Ticker symbol of instrument.</li> <li>International securities identification number (ISIN). <li>Financial instrument global identifier (FIGI). <li>Composite FIGI.</li> <li>Share Class FIGI.</li> </ul>
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }
    /// Number of matches in response. Max <code>120</code>
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }
    /// Adds info on which plan symbol is available.
    pub fn show_plan(mut self, show_plan: bool) -> Self {
        self.show_plan = Some(show_plan);
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetSymbolSearchParams {
        GetSymbolSearchParams {
            symbol: self.symbol,
            outputsize: self.outputsize,
            show_plan: self.show_plan
        }
    }
}


/// struct for typed errors of method [`get_bonds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBondsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_commodities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommoditiesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_countries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCountriesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cross_listings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCrossListingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cryptocurrencies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCryptocurrenciesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cryptocurrency_exchanges`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCryptocurrencyExchangesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_earliest_timestamp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEarliestTimestampError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_etf`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEtfError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_etfs_family`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEtfsFamilyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_etfs_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEtfsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_etfs_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEtfsTypeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_exchange_schedule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExchangeScheduleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_exchanges`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExchangesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_forex_pairs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetForexPairsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_funds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFundsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_instrument_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstrumentTypeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_intervals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIntervalsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_market_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMarketStateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_family`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsFamilyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_mutual_funds_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMutualFundsTypeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_stocks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStocksError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_symbol_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSymbolSearchError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_technical_indicators`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTechnicalIndicatorsError {
    UnknownValue(serde_json::Value),
}


/// The fixed income endpoint provides a daily updated list of available bonds. It returns an array containing detailed information about each bond, including identifiers, names, and other relevant attributes.
pub async fn get_bonds(configuration: &configuration::Configuration, params: GetBondsParams) -> Result<models::GetBonds200Response, Error<GetBondsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_exchange = params.exchange;
    let p_query_country = params.country;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_show_plan = params.show_plan;
    let p_query_page = params.page;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/bonds", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_show_plan {
        req_builder = req_builder.query(&[("show_plan", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBonds200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBonds200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBondsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The commodities endpoint provides a daily updated list of available commodity pairs, across precious metals, livestock, softs, grains, etc.
pub async fn get_commodities(configuration: &configuration::Configuration, params: GetCommoditiesParams) -> Result<models::GetCommodities200Response, Error<GetCommoditiesError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_category = params.category;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;

    let uri_str = format!("{}/commodities", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_category {
        req_builder = req_builder.query(&[("category", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCommodities200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCommodities200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCommoditiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The countries endpoint provides a comprehensive list of countries, including their ISO codes, official names, capitals, and currencies. This data is essential for applications requiring accurate country information for tasks such as localization, currency conversion, or geographic analysis.
pub async fn get_countries(configuration: &configuration::Configuration) -> Result<models::GetCountries200Response, Error<GetCountriesError>> {

    let uri_str = format!("{}/countries", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCountries200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCountries200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCountriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The cross_listings endpoint provides a daily updated list of cross-listed symbols for a specified financial instrument. Cross-listed symbols represent the same security available on multiple exchanges. This endpoint is useful for identifying all the exchanges where a particular security is traded, allowing users to access comprehensive trading information across different markets.
pub async fn get_cross_listings(configuration: &configuration::Configuration, params: GetCrossListingsParams) -> Result<models::GetCrossListings200Response, Error<GetCrossListingsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;

    let uri_str = format!("{}/cross_listings", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCrossListings200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCrossListings200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCrossListingsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The cryptocurrencies endpoint provides a daily updated list of all available cryptos. It returns an array containing detailed information about each cryptocurrency, including its symbol, name, and other relevant identifiers. This endpoint is useful for retrieving a comprehensive catalog of cryptocurrencies for applications that require up-to-date market listings or need to display available crypto assets to users.
pub async fn get_cryptocurrencies(configuration: &configuration::Configuration, params: GetCryptocurrenciesParams) -> Result<models::GetCryptocurrencies200Response, Error<GetCryptocurrenciesError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_exchange = params.exchange;
    let p_query_currency_base = params.currency_base;
    let p_query_currency_quote = params.currency_quote;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;

    let uri_str = format!("{}/cryptocurrencies", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_currency_base {
        req_builder = req_builder.query(&[("currency_base", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_currency_quote {
        req_builder = req_builder.query(&[("currency_quote", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCryptocurrencies200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCryptocurrencies200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCryptocurrenciesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The cryptocurrency exchanges endpoint provides a daily updated list of available cryptocurrency exchanges. It returns an array containing details about each exchange, such as exchange names and identifiers.
pub async fn get_cryptocurrency_exchanges(configuration: &configuration::Configuration, params: GetCryptocurrencyExchangesParams) -> Result<models::GetCryptocurrencyExchanges200Response, Error<GetCryptocurrencyExchangesError>> {
    // Extract parameters from params struct
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;

    let uri_str = format!("{}/cryptocurrency_exchanges", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCryptocurrencyExchanges200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCryptocurrencyExchanges200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCryptocurrencyExchangesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The earliest_timestamp endpoint provides the earliest available date and time for a specified financial instrument at a given data interval. This endpoint is useful for determining the starting point of historical data availability for various assets, such as stocks or currencies, allowing users to understand the time range covered by the data.
pub async fn get_earliest_timestamp(configuration: &configuration::Configuration, params: GetEarliestTimestampParams) -> Result<models::GetEarliestTimestamp200Response, Error<GetEarliestTimestampError>> {
    // Extract parameters from params struct
    let p_query_interval = params.interval;
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_timezone = params.timezone;

    let uri_str = format!("{}/earliest_timestamp", configuration.base_path);
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
    req_builder = req_builder.query(&[("interval", &p_query_interval.to_string())]);
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_timezone {
        req_builder = req_builder.query(&[("timezone", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEarliestTimestamp200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEarliestTimestamp200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEarliestTimestampError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The ETFs endpoint provides a daily updated list of all available Exchange-Traded Funds. It returns an array containing detailed information about each ETF, including its symbol, name, and other relevant identifiers. This endpoint is useful for retrieving a comprehensive catalog of ETFs for portfolio management, investment tracking, or financial analysis.
pub async fn get_etf(configuration: &configuration::Configuration, params: GetEtfParams) -> Result<models::GetEtf200Response, Error<GetEtfError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_cik = params.cik;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_show_plan = params.show_plan;
    let p_query_include_delisted = params.include_delisted;

    let uri_str = format!("{}/etfs", configuration.base_path);
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
    if let Some(ref param_value) = p_query_cik {
        req_builder = req_builder.query(&[("cik", &param_value.to_string())]);
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
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_show_plan {
        req_builder = req_builder.query(&[("show_plan", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_include_delisted {
        req_builder = req_builder.query(&[("include_delisted", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEtf200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEtf200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEtfError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve a comprehensive list of exchange-traded fund (ETF) families, providing users with detailed information on various ETF groups available in the market. This endpoint is ideal for users looking to explore different ETF categories, compare offerings, or integrate ETF family data into their financial applications.
pub async fn get_etfs_family(configuration: &configuration::Configuration, params: GetEtfsFamilyParams) -> Result<models::GetEtfsFamily200Response, Error<GetEtfsFamilyError>> {
    // Extract parameters from params struct
    let p_query_country = params.country;
    let p_query_fund_family = params.fund_family;

    let uri_str = format!("{}/etfs/family", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_fund_family {
        req_builder = req_builder.query(&[("fund_family", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEtfsFamily200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEtfsFamily200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEtfsFamilyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The ETFs directory endpoint provides a daily updated list of exchange-traded funds, sorted by total assets in descending order. This endpoint is useful for retrieving comprehensive ETF data, including fund names and asset values, to assist users in quickly identifying the ETFs available.
pub async fn get_etfs_list(configuration: &configuration::Configuration, params: GetEtfsListParams) -> Result<models::GetEtfsList200Response, Error<GetEtfsListError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_cik = params.cik;
    let p_query_country = params.country;
    let p_query_fund_family = params.fund_family;
    let p_query_fund_type = params.fund_type;
    let p_query_page = params.page;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/etfs/list", configuration.base_path);
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
    if let Some(ref param_value) = p_query_cik {
        req_builder = req_builder.query(&[("cik", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_fund_family {
        req_builder = req_builder.query(&[("fund_family", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_fund_type {
        req_builder = req_builder.query(&[("fund_type", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEtfsList200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEtfsList200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEtfsListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The ETFs Types endpoint provides a concise list of ETF categories by market (e.g., Singapore, United States), including types like \"Equity Precious Metals\" and \"Large Blend.\" It supports targeted investment research and portfolio diversification.
pub async fn get_etfs_type(configuration: &configuration::Configuration, params: GetEtfsTypeParams) -> Result<models::GetEtfsType200Response, Error<GetEtfsTypeError>> {
    // Extract parameters from params struct
    let p_query_country = params.country;
    let p_query_fund_type = params.fund_type;

    let uri_str = format!("{}/etfs/type", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_fund_type {
        req_builder = req_builder.query(&[("fund_type", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetEtfsType200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetEtfsType200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEtfsTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The exchanges schedule endpoint provides detailed information about various stock exchanges, including their trading hours and operational days. This data is essential for users who need to know when specific exchanges are open for trading, allowing them to plan their activities around the availability of these markets.
pub async fn get_exchange_schedule(configuration: &configuration::Configuration, params: GetExchangeScheduleParams) -> Result<models::GetExchangeSchedule200Response, Error<GetExchangeScheduleError>> {
    // Extract parameters from params struct
    let p_query_mic_name = params.mic_name;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_date = params.date;

    let uri_str = format!("{}/exchange_schedule", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_mic_name {
        req_builder = req_builder.query(&[("mic_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_mic_code {
        req_builder = req_builder.query(&[("mic_code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_date {
        req_builder = req_builder.query(&[("date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetExchangeSchedule200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetExchangeSchedule200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetExchangeScheduleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The exchanges endpoint provides a comprehensive list of all available equity exchanges. It returns an array containing detailed information about each exchange, such as exchange code, name, country, and timezone. This data is updated daily.
pub async fn get_exchanges(configuration: &configuration::Configuration, params: GetExchangesParams) -> Result<models::GetExchanges200Response, Error<GetExchangesError>> {
    // Extract parameters from params struct
    let p_query_type = params.r#type;
    let p_query_name = params.name;
    let p_query_code = params.code;
    let p_query_country = params.country;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_show_plan = params.show_plan;

    let uri_str = format!("{}/exchanges", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_code {
        req_builder = req_builder.query(&[("code", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_show_plan {
        req_builder = req_builder.query(&[("show_plan", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetExchanges200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetExchanges200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetExchangesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The forex pairs endpoint provides a comprehensive list of all available foreign exchange currency pairs. It returns an array of forex pairs, which is updated daily.
pub async fn get_forex_pairs(configuration: &configuration::Configuration, params: GetForexPairsParams) -> Result<models::GetForexPairs200Response, Error<GetForexPairsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_currency_base = params.currency_base;
    let p_query_currency_quote = params.currency_quote;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;

    let uri_str = format!("{}/forex_pairs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_currency_base {
        req_builder = req_builder.query(&[("currency_base", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_currency_quote {
        req_builder = req_builder.query(&[("currency_quote", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetForexPairs200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetForexPairs200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetForexPairsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The funds endpoint provides a daily updated list of available investment funds. It returns an array containing detailed information about each fund, including identifiers, names, and other relevant attributes.
pub async fn get_funds(configuration: &configuration::Configuration, params: GetFundsParams) -> Result<models::GetFunds200Response, Error<GetFundsError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_cik = params.cik;
    let p_query_exchange = params.exchange;
    let p_query_country = params.country;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_show_plan = params.show_plan;
    let p_query_page = params.page;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/funds", configuration.base_path);
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
    if let Some(ref param_value) = p_query_cik {
        req_builder = req_builder.query(&[("cik", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_show_plan {
        req_builder = req_builder.query(&[("show_plan", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetFunds200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetFunds200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFundsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The instrument type endpoint lists all available financial instrument types, such as stocks, ETFs, and cryptos. This information is essential for users to identify and categorize different financial instruments when accessing or analyzing market data.
pub async fn get_instrument_type(configuration: &configuration::Configuration) -> Result<models::GetInstrumentType200Response, Error<GetInstrumentTypeError>> {

    let uri_str = format!("{}/instrument_type", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetInstrumentType200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetInstrumentType200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInstrumentTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The intervals endpoint provides a list of supported time intervals that can be used for querying financial data.
pub async fn get_intervals(configuration: &configuration::Configuration) -> Result<models::GetIntervals200Response, Error<GetIntervalsError>> {

    let uri_str = format!("{}/intervals", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetIntervals200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetIntervals200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIntervalsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The market state endpoint provides real-time information on the operational status of all available stock exchanges. It returns data on whether each exchange is currently open or closed, along with the time remaining until the next opening or closing. This endpoint is useful for users who need to monitor exchange hours and plan their trading activities accordingly.
pub async fn get_market_state(configuration: &configuration::Configuration, params: GetMarketStateParams) -> Result<Vec<models::MarketStateResponseItem>, Error<GetMarketStateError>> {
    // Extract parameters from params struct
    let p_query_exchange = params.exchange;
    let p_query_code = params.code;
    let p_query_country = params.country;

    let uri_str = format!("{}/market_state", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_exchange {
        req_builder = req_builder.query(&[("exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_code {
        req_builder = req_builder.query(&[("code", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::MarketStateResponseItem&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::MarketStateResponseItem&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMarketStateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds family endpoint provides a comprehensive list of MF families, which are groups of mutual funds managed by the same investment company. This data is useful for users looking to explore or compare different fund families, understand the range of investment options offered by each, and identify potential investment opportunities within specific fund families.
pub async fn get_mutual_funds_family(configuration: &configuration::Configuration, params: GetMutualFundsFamilyParams) -> Result<models::GetMutualFundsFamily200Response, Error<GetMutualFundsFamilyError>> {
    // Extract parameters from params struct
    let p_query_fund_family = params.fund_family;
    let p_query_country = params.country;

    let uri_str = format!("{}/mutual_funds/family", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_fund_family {
        req_builder = req_builder.query(&[("fund_family", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsFamily200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsFamily200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsFamilyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The mutual funds directory endpoint provides a daily updated list of mutual funds, sorted in descending order by their total assets value. This endpoint is useful for retrieving an organized overview of available mutual funds.
pub async fn get_mutual_funds_list(configuration: &configuration::Configuration, params: GetMutualFundsListParams) -> Result<models::GetMutualFundsList200Response, Error<GetMutualFundsListError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_cik = params.cik;
    let p_query_country = params.country;
    let p_query_fund_family = params.fund_family;
    let p_query_fund_type = params.fund_type;
    let p_query_performance_rating = params.performance_rating;
    let p_query_risk_rating = params.risk_rating;
    let p_query_page = params.page;
    let p_query_outputsize = params.outputsize;

    let uri_str = format!("{}/mutual_funds/list", configuration.base_path);
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
    if let Some(ref param_value) = p_query_cik {
        req_builder = req_builder.query(&[("cik", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_country {
        req_builder = req_builder.query(&[("country", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_fund_family {
        req_builder = req_builder.query(&[("fund_family", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_fund_type {
        req_builder = req_builder.query(&[("fund_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_performance_rating {
        req_builder = req_builder.query(&[("performance_rating", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_risk_rating {
        req_builder = req_builder.query(&[("risk_rating", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsList200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsList200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// This endpoint provides detailed information on various types of mutual funds, such as equity, bond, and balanced funds, allowing users to understand the different investment options available.
pub async fn get_mutual_funds_type(configuration: &configuration::Configuration, params: GetMutualFundsTypeParams) -> Result<models::GetMutualFundsType200Response, Error<GetMutualFundsTypeError>> {
    // Extract parameters from params struct
    let p_query_fund_type = params.fund_type;
    let p_query_country = params.country;

    let uri_str = format!("{}/mutual_funds/type", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_fund_type {
        req_builder = req_builder.query(&[("fund_type", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetMutualFundsType200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetMutualFundsType200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMutualFundsTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The stocks endpoint provides a daily updated list of all available stock symbols. It returns an array containing the symbols, which can be used to identify and access specific stock data across various services. This endpoint is essential for users needing to retrieve the latest stock symbol information for further data requests or integration into financial applications.
pub async fn get_stocks(configuration: &configuration::Configuration, params: GetStocksParams) -> Result<models::GetStocks200Response, Error<GetStocksError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_figi = params.figi;
    let p_query_isin = params.isin;
    let p_query_cusip = params.cusip;
    let p_query_cik = params.cik;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_type = params.r#type;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_show_plan = params.show_plan;
    let p_query_include_delisted = params.include_delisted;

    let uri_str = format!("{}/stocks", configuration.base_path);
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
    if let Some(ref param_value) = p_query_cik {
        req_builder = req_builder.query(&[("cik", &param_value.to_string())]);
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
        req_builder = req_builder.query(&[("type", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_show_plan {
        req_builder = req_builder.query(&[("show_plan", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_include_delisted {
        req_builder = req_builder.query(&[("include_delisted", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetStocks200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetStocks200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetStocksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The symbol search endpoint allows users to find financial instruments by name or symbol. It returns a list of matching symbols, ordered by relevance, with the most relevant instrument first. This is useful for quickly locating specific stocks, ETFs, or other financial instruments when only partial information is available.
pub async fn get_symbol_search(configuration: &configuration::Configuration, params: GetSymbolSearchParams) -> Result<models::GetSymbolSearch200Response, Error<GetSymbolSearchError>> {
    // Extract parameters from params struct
    let p_query_symbol = params.symbol;
    let p_query_outputsize = params.outputsize;
    let p_query_show_plan = params.show_plan;

    let uri_str = format!("{}/symbol_search", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("symbol", &p_query_symbol.to_string())]);
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_show_plan {
        req_builder = req_builder.query(&[("show_plan", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetSymbolSearch200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetSymbolSearch200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSymbolSearchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The technical indicators endpoint provides a comprehensive list of available technical indicators, each represented as an object. This endpoint is useful for developers looking to integrate a variety of technical analysis tools into their applications, allowing for streamlined access to indicator data without needing to manually configure each one.
pub async fn get_technical_indicators(configuration: &configuration::Configuration) -> Result<models::GetTechnicalIndicators200Response, Error<GetTechnicalIndicatorsError>> {

    let uri_str = format!("{}/technical_indicators", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTechnicalIndicators200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTechnicalIndicators200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTechnicalIndicatorsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

