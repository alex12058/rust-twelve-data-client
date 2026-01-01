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

/// struct for passing parameters to the method [`get_time_series`]
#[derive(Clone, Debug, Default)]
pub struct GetTimeSeriesParams {
    /// Interval between two consecutive points in time series
    pub interval: String,
    /// Symbol ticker of the instrument. E.g. `AAPL`, `EUR/USD`, `ETH/BTC`, ...
    pub symbol: Option<String>,
    /// Filter by international securities identification number (ISIN)
    pub isin: Option<String>,
    /// The FIGI of an instrument for which data is requested
    pub figi: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    pub cusip: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum
    pub outputsize: Option<i64>,
    /// Exchange where instrument is traded
    pub exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    pub mic_code: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub country: Option<String>,
    /// The asset class to which the instrument belongs
    pub r#type: Option<String>,
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a></li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    pub timezone: Option<String>,
    /// Can be used separately and together with `end_date`. Format `2006-01-02` or `2006-01-02T15:04:05`  Default location: <ul> <li>Forex and Cryptocurrencies - <code>UTC</code></li> <li>Stocks - where exchange is located (e.g. for AAPL it will be <code>America/New_York</code>)</li> </ul> Both parameters take into account if <code>timezone</code> parameter is provided.<br/> If <code>timezone</code> is given then, <code>start_date</code> and <code>end_date</code> will be used in the specified location  Examples: <ul> <li>1. <code>&symbol=AAPL&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 New York time up to current date</li> <li>2. <code>&symbol=EUR/USD&timezone=Asia/Singapore&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 Singapore time up to current date</li> <li>3. <code>&symbol=ETH/BTC&timezone=Europe/Zurich&start_date=2019-08-09T15:50:00&end_date=2019-08-09T15:55:00&...</code><br/> Returns all records starting from 2019-08-09T15:50:00 Zurich time up to 2019-08-09T15:55:00</li> </ul>
    pub start_date: Option<String>,
    /// The ending date and time for data selection, see `start_date` description for details.
    pub end_date: Option<String>,
    /// Specifies the exact date to get the data for. Could be the exact date, e.g. `2021-10-27`, or in human language `today` or `yesterday`
    pub date: Option<String>,
    /// Sorting order of the output
    pub order: Option<String>,
    /// Returns quotes that include pre-market and post-market data. Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume
    pub prepost: Option<bool>,
    /// The format of the response data
    pub format: Option<String>,
    /// The separator used in the CSV response data
    pub delimiter: Option<String>,
    /// Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive. By default, the number of decimal places is automatically determined based on the values provided
    pub dp: Option<i64>,
    /// A boolean parameter to include the previous closing price in the time_series data. If true, adds previous bar close price value to the current object
    pub previous_close: Option<bool>,
    /// Adjusting mode for prices
    pub adjust: Option<String>
}

impl GetTimeSeriesParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetTimeSeriesParamsBuilder {
        GetTimeSeriesParamsBuilder::default()
    }
}

/// Builder for [`GetTimeSeriesParams`]
#[derive(Clone, Debug, Default)]
pub struct GetTimeSeriesParamsBuilder {
    /// Interval between two consecutive points in time series
    interval: String,
    /// Symbol ticker of the instrument. E.g. `AAPL`, `EUR/USD`, `ETH/BTC`, ...
    symbol: Option<String>,
    /// Filter by international securities identification number (ISIN)
    isin: Option<String>,
    /// The FIGI of an instrument for which data is requested
    figi: Option<String>,
    /// The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section
    cusip: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum
    outputsize: Option<i64>,
    /// Exchange where instrument is traded
    exchange: Option<String>,
    /// Market Identifier Code (MIC) under ISO 10383 standard
    mic_code: Option<String>,
    /// The country where the instrument is traded, e.g., `United States` or `US`
    country: Option<String>,
    /// The asset class to which the instrument belongs
    r#type: Option<String>,
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a></li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    timezone: Option<String>,
    /// Can be used separately and together with `end_date`. Format `2006-01-02` or `2006-01-02T15:04:05`  Default location: <ul> <li>Forex and Cryptocurrencies - <code>UTC</code></li> <li>Stocks - where exchange is located (e.g. for AAPL it will be <code>America/New_York</code>)</li> </ul> Both parameters take into account if <code>timezone</code> parameter is provided.<br/> If <code>timezone</code> is given then, <code>start_date</code> and <code>end_date</code> will be used in the specified location  Examples: <ul> <li>1. <code>&symbol=AAPL&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 New York time up to current date</li> <li>2. <code>&symbol=EUR/USD&timezone=Asia/Singapore&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 Singapore time up to current date</li> <li>3. <code>&symbol=ETH/BTC&timezone=Europe/Zurich&start_date=2019-08-09T15:50:00&end_date=2019-08-09T15:55:00&...</code><br/> Returns all records starting from 2019-08-09T15:50:00 Zurich time up to 2019-08-09T15:55:00</li> </ul>
    start_date: Option<String>,
    /// The ending date and time for data selection, see `start_date` description for details.
    end_date: Option<String>,
    /// Specifies the exact date to get the data for. Could be the exact date, e.g. `2021-10-27`, or in human language `today` or `yesterday`
    date: Option<String>,
    /// Sorting order of the output
    order: Option<String>,
    /// Returns quotes that include pre-market and post-market data. Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume
    prepost: Option<bool>,
    /// The format of the response data
    format: Option<String>,
    /// The separator used in the CSV response data
    delimiter: Option<String>,
    /// Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive. By default, the number of decimal places is automatically determined based on the values provided
    dp: Option<i64>,
    /// A boolean parameter to include the previous closing price in the time_series data. If true, adds previous bar close price value to the current object
    previous_close: Option<bool>,
    /// Adjusting mode for prices
    adjust: Option<String>
}

impl GetTimeSeriesParamsBuilder {
    /// Interval between two consecutive points in time series
    pub fn interval(mut self, interval: impl Into<String>) -> Self {
        self.interval = interval.into();
        self
    }
    /// Symbol ticker of the instrument. E.g. `AAPL`, `EUR/USD`, `ETH/BTC`, ...
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    /// Filter by international securities identification number (ISIN)
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
    /// Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
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
    /// The country where the instrument is traded, e.g., `United States` or `US`
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
    /// The asset class to which the instrument belongs
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a></li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }
    /// Can be used separately and together with `end_date`. Format `2006-01-02` or `2006-01-02T15:04:05`  Default location: <ul> <li>Forex and Cryptocurrencies - <code>UTC</code></li> <li>Stocks - where exchange is located (e.g. for AAPL it will be <code>America/New_York</code>)</li> </ul> Both parameters take into account if <code>timezone</code> parameter is provided.<br/> If <code>timezone</code> is given then, <code>start_date</code> and <code>end_date</code> will be used in the specified location  Examples: <ul> <li>1. <code>&symbol=AAPL&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 New York time up to current date</li> <li>2. <code>&symbol=EUR/USD&timezone=Asia/Singapore&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 Singapore time up to current date</li> <li>3. <code>&symbol=ETH/BTC&timezone=Europe/Zurich&start_date=2019-08-09T15:50:00&end_date=2019-08-09T15:55:00&...</code><br/> Returns all records starting from 2019-08-09T15:50:00 Zurich time up to 2019-08-09T15:55:00</li> </ul>
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// The ending date and time for data selection, see `start_date` description for details.
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Specifies the exact date to get the data for. Could be the exact date, e.g. `2021-10-27`, or in human language `today` or `yesterday`
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
    /// Sorting order of the output
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }
    /// Returns quotes that include pre-market and post-market data. Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume
    pub fn prepost(mut self, prepost: bool) -> Self {
        self.prepost = Some(prepost);
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
    /// Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive. By default, the number of decimal places is automatically determined based on the values provided
    pub fn dp(mut self, dp: i64) -> Self {
        self.dp = Some(dp);
        self
    }
    /// A boolean parameter to include the previous closing price in the time_series data. If true, adds previous bar close price value to the current object
    pub fn previous_close(mut self, previous_close: bool) -> Self {
        self.previous_close = Some(previous_close);
        self
    }
    /// Adjusting mode for prices
    pub fn adjust(mut self, adjust: impl Into<String>) -> Self {
        self.adjust = Some(adjust.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetTimeSeriesParams {
        GetTimeSeriesParams {
            interval: self.interval,
            symbol: self.symbol,
            isin: self.isin,
            figi: self.figi,
            cusip: self.cusip,
            outputsize: self.outputsize,
            exchange: self.exchange,
            mic_code: self.mic_code,
            country: self.country,
            r#type: self.r#type,
            timezone: self.timezone,
            start_date: self.start_date,
            end_date: self.end_date,
            date: self.date,
            order: self.order,
            prepost: self.prepost,
            format: self.format,
            delimiter: self.delimiter,
            dp: self.dp,
            previous_close: self.previous_close,
            adjust: self.adjust
        }
    }
}

/// struct for passing parameters to the method [`get_time_series_cross`]
#[derive(Clone, Debug, Default)]
pub struct GetTimeSeriesCrossParams {
    /// Base currency symbol
    pub base: String,
    /// Quote currency symbol
    pub quote: String,
    /// Interval between two consecutive points in time series
    pub interval: String,
    /// Base instrument type according to the `/instrument_type` endpoint
    pub base_type: Option<String>,
    /// Base exchange
    pub base_exchange: Option<String>,
    /// Base MIC code
    pub base_mic_code: Option<String>,
    /// Quote instrument type according to the `/instrument_type` endpoint
    pub quote_type: Option<String>,
    /// Quote exchange
    pub quote_exchange: Option<String>,
    /// Quote MIC code
    pub quote_mic_code: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum
    pub outputsize: Option<i64>,
    /// Format of the response data
    pub format: Option<String>,
    /// Delimiter used in CSV file
    pub delimiter: Option<String>,
    /// Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume.
    pub prepost: Option<bool>,
    /// Start date for the time series data
    pub start_date: Option<String>,
    /// End date for the time series data
    pub end_date: Option<String>,
    /// Specifies if there should be an adjustment
    pub adjust: Option<bool>,
    /// Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive.
    pub dp: Option<i64>,
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    pub timezone: Option<String>
}

impl GetTimeSeriesCrossParams {
    /// Create a new builder for this parameter struct
    pub fn builder() -> GetTimeSeriesCrossParamsBuilder {
        GetTimeSeriesCrossParamsBuilder::default()
    }
}

/// Builder for [`GetTimeSeriesCrossParams`]
#[derive(Clone, Debug, Default)]
pub struct GetTimeSeriesCrossParamsBuilder {
    /// Base currency symbol
    base: String,
    /// Quote currency symbol
    quote: String,
    /// Interval between two consecutive points in time series
    interval: String,
    /// Base instrument type according to the `/instrument_type` endpoint
    base_type: Option<String>,
    /// Base exchange
    base_exchange: Option<String>,
    /// Base MIC code
    base_mic_code: Option<String>,
    /// Quote instrument type according to the `/instrument_type` endpoint
    quote_type: Option<String>,
    /// Quote exchange
    quote_exchange: Option<String>,
    /// Quote MIC code
    quote_mic_code: Option<String>,
    /// Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum
    outputsize: Option<i64>,
    /// Format of the response data
    format: Option<String>,
    /// Delimiter used in CSV file
    delimiter: Option<String>,
    /// Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume.
    prepost: Option<bool>,
    /// Start date for the time series data
    start_date: Option<String>,
    /// End date for the time series data
    end_date: Option<String>,
    /// Specifies if there should be an adjustment
    adjust: Option<bool>,
    /// Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive.
    dp: Option<i64>,
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    timezone: Option<String>
}

impl GetTimeSeriesCrossParamsBuilder {
    /// Base currency symbol
    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = base.into();
        self
    }
    /// Quote currency symbol
    pub fn quote(mut self, quote: impl Into<String>) -> Self {
        self.quote = quote.into();
        self
    }
    /// Interval between two consecutive points in time series
    pub fn interval(mut self, interval: impl Into<String>) -> Self {
        self.interval = interval.into();
        self
    }
    /// Base instrument type according to the `/instrument_type` endpoint
    pub fn base_type(mut self, base_type: impl Into<String>) -> Self {
        self.base_type = Some(base_type.into());
        self
    }
    /// Base exchange
    pub fn base_exchange(mut self, base_exchange: impl Into<String>) -> Self {
        self.base_exchange = Some(base_exchange.into());
        self
    }
    /// Base MIC code
    pub fn base_mic_code(mut self, base_mic_code: impl Into<String>) -> Self {
        self.base_mic_code = Some(base_mic_code.into());
        self
    }
    /// Quote instrument type according to the `/instrument_type` endpoint
    pub fn quote_type(mut self, quote_type: impl Into<String>) -> Self {
        self.quote_type = Some(quote_type.into());
        self
    }
    /// Quote exchange
    pub fn quote_exchange(mut self, quote_exchange: impl Into<String>) -> Self {
        self.quote_exchange = Some(quote_exchange.into());
        self
    }
    /// Quote MIC code
    pub fn quote_mic_code(mut self, quote_mic_code: impl Into<String>) -> Self {
        self.quote_mic_code = Some(quote_mic_code.into());
        self
    }
    /// Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum
    pub fn outputsize(mut self, outputsize: i64) -> Self {
        self.outputsize = Some(outputsize);
        self
    }
    /// Format of the response data
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
    /// Delimiter used in CSV file
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }
    /// Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume.
    pub fn prepost(mut self, prepost: bool) -> Self {
        self.prepost = Some(prepost);
        self
    }
    /// Start date for the time series data
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
    /// End date for the time series data
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }
    /// Specifies if there should be an adjustment
    pub fn adjust(mut self, adjust: bool) -> Self {
        self.adjust = Some(adjust);
        self
    }
    /// Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive.
    pub fn dp(mut self, dp: i64) -> Self {
        self.dp = Some(dp);
        self
    }
    /// Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i>
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    /// Build the parameter struct
    pub fn build(self) -> GetTimeSeriesCrossParams {
        GetTimeSeriesCrossParams {
            base: self.base,
            quote: self.quote,
            interval: self.interval,
            base_type: self.base_type,
            base_exchange: self.base_exchange,
            base_mic_code: self.base_mic_code,
            quote_type: self.quote_type,
            quote_exchange: self.quote_exchange,
            quote_mic_code: self.quote_mic_code,
            outputsize: self.outputsize,
            format: self.format,
            delimiter: self.delimiter,
            prepost: self.prepost,
            start_date: self.start_date,
            end_date: self.end_date,
            adjust: self.adjust,
            dp: self.dp,
            timezone: self.timezone
        }
    }
}


/// struct for typed errors of method [`get_time_series`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTimeSeriesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_time_series_cross`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTimeSeriesCrossError {
    UnknownValue(serde_json::Value),
}


/// The time series endpoint provides detailed historical data for a specified financial instrument. It returns two main components: metadata, which includes essential information about the instrument, and a time series dataset. The time series consists of chronological entries with Open, High, Low, and Close prices, and for applicable instruments, it also includes trading volume. This endpoint is ideal for retrieving comprehensive historical price data for analysis or visualization purposes.
pub async fn get_time_series(configuration: &configuration::Configuration, params: GetTimeSeriesParams) -> Result<models::GetTimeSeries200Response, Error<GetTimeSeriesError>> {
    // Extract parameters from params struct
    let p_query_interval = params.interval;
    let p_query_symbol = params.symbol;
    let p_query_isin = params.isin;
    let p_query_figi = params.figi;
    let p_query_cusip = params.cusip;
    let p_query_outputsize = params.outputsize;
    let p_query_exchange = params.exchange;
    let p_query_mic_code = params.mic_code;
    let p_query_country = params.country;
    let p_query_type = params.r#type;
    let p_query_timezone = params.timezone;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_date = params.date;
    let p_query_order = params.order;
    let p_query_prepost = params.prepost;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_dp = params.dp;
    let p_query_previous_close = params.previous_close;
    let p_query_adjust = params.adjust;

    let uri_str = format!("{}/time_series", configuration.base_path);
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
    req_builder = req_builder.query(&[("interval", &p_query_interval.to_string())]);
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
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
    if let Some(ref param_value) = p_query_timezone {
        req_builder = req_builder.query(&[("timezone", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_date {
        req_builder = req_builder.query(&[("date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_order {
        req_builder = req_builder.query(&[("order", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_prepost {
        req_builder = req_builder.query(&[("prepost", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_previous_close {
        req_builder = req_builder.query(&[("previous_close", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_adjust {
        req_builder = req_builder.query(&[("adjust", &serde_json::to_string(param_value)?)]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTimeSeries200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTimeSeries200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTimeSeriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The Time Series Cross endpoint calculates and returns historical cross-rate data for exotic forex pairs, cryptocurrencies, or stocks (e.g., Apple Inc. price in Indian Rupees) on the fly. It provides metadata about the requested symbol and a time series array with Open, High, Low, and Close prices, sorted descending by time, enabling analysis of price history and market trends.
pub async fn get_time_series_cross(configuration: &configuration::Configuration, params: GetTimeSeriesCrossParams) -> Result<models::GetTimeSeriesCross200Response, Error<GetTimeSeriesCrossError>> {
    // Extract parameters from params struct
    let p_query_base = params.base;
    let p_query_quote = params.quote;
    let p_query_interval = params.interval;
    let p_query_base_type = params.base_type;
    let p_query_base_exchange = params.base_exchange;
    let p_query_base_mic_code = params.base_mic_code;
    let p_query_quote_type = params.quote_type;
    let p_query_quote_exchange = params.quote_exchange;
    let p_query_quote_mic_code = params.quote_mic_code;
    let p_query_outputsize = params.outputsize;
    let p_query_format = params.format;
    let p_query_delimiter = params.delimiter;
    let p_query_prepost = params.prepost;
    let p_query_start_date = params.start_date;
    let p_query_end_date = params.end_date;
    let p_query_adjust = params.adjust;
    let p_query_dp = params.dp;
    let p_query_timezone = params.timezone;

    let uri_str = format!("{}/time_series/cross", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("base", &p_query_base.to_string())]);
    if let Some(ref param_value) = p_query_base_type {
        req_builder = req_builder.query(&[("base_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_base_exchange {
        req_builder = req_builder.query(&[("base_exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_base_mic_code {
        req_builder = req_builder.query(&[("base_mic_code", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("quote", &p_query_quote.to_string())]);
    if let Some(ref param_value) = p_query_quote_type {
        req_builder = req_builder.query(&[("quote_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_quote_exchange {
        req_builder = req_builder.query(&[("quote_exchange", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_quote_mic_code {
        req_builder = req_builder.query(&[("quote_mic_code", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("interval", &p_query_interval.to_string())]);
    if let Some(ref param_value) = p_query_outputsize {
        req_builder = req_builder.query(&[("outputsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_format {
        req_builder = req_builder.query(&[("format", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_delimiter {
        req_builder = req_builder.query(&[("delimiter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_prepost {
        req_builder = req_builder.query(&[("prepost", &param_value.to_string())]);
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
    if let Some(ref param_value) = p_query_dp {
        req_builder = req_builder.query(&[("dp", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTimeSeriesCross200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTimeSeriesCross200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTimeSeriesCrossError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

