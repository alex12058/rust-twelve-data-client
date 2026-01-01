/*
 * Twelve Data API
 *
 * ## Overview  Welcome to Twelve Data developer docs — your gateway to comprehensive financial market data through a powerful and easy-to-use API. Twelve Data provides access to financial markets across over 50 global countries, covering more than 1 million public instruments, including stocks, forex, ETFs, mutual funds, commodities, and cryptocurrencies.  ## Quickstart  To get started, you'll need to sign up for an API key. Once you have your API key, you can start making requests to the API.  ### Step 1: Create Twelve Data account  Sign up on the Twelve Data website to create your account [here](https://twelvedata.com/register). This gives you access to the API dashboard and your API key.  ### Step 2: Get your API key  After signing in, navigate to your [dashboard](https://twelvedata.com/account/api-keys) to find your unique API key. This key is required to authenticate all API and WebSocket requests.  ### Step 3: Make your first request  Try a simple API call with cURL to fetch the latest price for Apple (AAPL):  ``` curl \"https://api.twelvedata.com/price?symbol=AAPL&apikey=your_api_key\" ```  ### Step 4: Make a request from Python or Javascript  Use our client libraries or standard HTTP clients to make API calls programmatically. Here’s an example in [Python](https://github.com/twelvedata/twelvedata-python) and JavaScript:  #### Python (using official Twelve Data SDK):  ```python from twelvedata import TDClient  # Initialize client with your API key td = TDClient(apikey=\"your_api_key\")  # Get latest price for Apple price = td.price(symbol=\"AAPL\").as_json()  print(price) ```  #### JavaScript (Node.js):  ```javascript const fetch = require('node-fetch');  fetch('https://api.twelvedata.com/price?symbol=AAPL&apikey=your_api_key') &nbsp;&nbsp;.then(response => response.json()) &nbsp;&nbsp;.then(data => console.log(data)); ```  ### Step 5: Perform correlation analysis between Tesla and Microsoft prices  Fetch historical price data for Tesla (TSLA) and Microsoft (MSFT) and calculate the correlation of their closing prices:  ```python from twelvedata import TDClient import pandas as pd  # Initialize client with your API key td = TDClient(apikey=\"your_api_key\")  # Fetch historical price data for Tesla tsla_ts = td.time_series( &nbsp;&nbsp;&nbsp;&nbsp;symbol=\"TSLA\", &nbsp;&nbsp;&nbsp;&nbsp;interval=\"1day\", &nbsp;&nbsp;&nbsp;&nbsp;outputsize=100 ).as_pandas()  # Fetch historical price data for Microsoft msft_ts = td.time_series( &nbsp;&nbsp;&nbsp;&nbsp;symbol=\"MSFT\", &nbsp;&nbsp;&nbsp;&nbsp;interval=\"1day\", &nbsp;&nbsp;&nbsp;&nbsp;outputsize=100 ).as_pandas()  # Align data on datetime index combined = pd.concat( &nbsp;&nbsp;&nbsp;&nbsp;[tsla_ts['close'].astype(float), msft_ts['close'].astype(float)], &nbsp;&nbsp;&nbsp;&nbsp;axis=1, &nbsp;&nbsp;&nbsp;&nbsp;keys=[\"TSLA\", \"MSFT\"] ).dropna()  # Calculate correlation correlation = combined[\"TSLA\"].corr(combined[\"MSFT\"]) print(f\"Correlation of closing prices between TSLA and MSFT: {correlation:.2f}\") ```  ### Authentication  Authenticate your requests using one of these methods:  #### Query parameter method ``` GET https://api.twelvedata.com/endpoint?symbol=AAPL&apikey=your_api_key ```  #### HTTP header method (recommended) ``` Authorization: apikey your_api_key ```  ##### API key useful information <ul> <li> Demo API key (<code>apikey=demo</code>) available for demo requests</li> <li> Personal API key required for full access</li> <li> Premium endpoints and data require higher-tier plans (testable with <a href=\"https://twelvedata.com/exchanges\">trial symbols</a>)</li> </ul>  ### API endpoints   Service | Base URL | ---------|----------|  REST API | `https://api.twelvedata.com` |  WebSocket | `wss://ws.twelvedata.com` |  ### Parameter guidelines <ul> <li><b>Separator:</b> Use <code>&</code> to separate multiple parameters</li> <li><b>Case sensitivity:</b> Parameter names are case-insensitive</li>  <ul><li><code>symbol=AAPL</code> = <code>symbol=aapl</code></li></ul>  <li><b>Multiple values:</b> Separate with commas where supported</li> </ul>  ### Response handling  #### Default format All responses return JSON format by default unless otherwise specified.  #### Null values <b>Important:</b> Some response fields may contain `null` values when data is unavailable for specific metrics. This is expected behavior, not an error.  ##### Best Practices: <ul> <li>Always implement <code>null</code> value handling in your application</li> <li>Use defensive programming techniques for data processing</li> <li>Consider fallback values or error handling for critical metrics</li> </ul>  #### Error handling Structure your code to gracefully handle: <ul> <li>Network timeouts</li> <li>Rate limiting responses</li> <li>Invalid parameter errors</li> <li>Data unavailability periods</li> </ul>  ##### Best practices <ul> <li><b>Rate limits:</b> Adhere to your plan’s rate limits to avoid throttling. Check your dashboard for details.</li> <li><b>Error handling:</b> Implement retry logic for transient errors (e.g., <code>429 Too Many Requests</code>).</li> <li><b>Caching:</b> Cache responses for frequently accessed data to reduce API calls and improve performance.</li> <li><b>Secure storage:</b> Store your API key securely and never expose it in client-side code or public repositories.</li> </ul>  ## Errors  Twelve Data API employs a standardized error response format, delivering a JSON object with `code`, `message`, and `status` keys for clear and consistent error communication.  ### Codes  Below is a table of possible error codes, their HTTP status, meanings, and resolution steps:   Code | status | Meaning | Resolution |  --- | --- | --- | --- |  **400** | Bad Request | Invalid or incorrect parameter(s) provided. | Check the `message` in the response for details. Refer to the API Documenta­tion to correct the input. |  **401** | Unauthor­ized | Invalid or incorrect API key. | Verify your API key is correct. Sign up for a key <a href=\"https://twelvedata.com/account/api-keys\">here</a>. |  **403** | Forbidden | API key lacks permissions for the requested resource (upgrade required). | Upgrade your plan <a href=\"https://twelvedata.com/pricing\">here</a>. |  **404** | Not Found | Requested data could not be found. | Adjust parameters to be less strict as they may be too restrictive. |  **414** | Parameter Too Long | Input parameter array exceeds the allowed length. | Follow the `message` guidance to adjust the parameter length. |  **429** | Too Many Requests | API request limit reached for your key. | Wait briefly or upgrade your plan <a href=\"https://twelvedata.com/pricing\">here</a>. |  **500** | Internal Server Error | Server-side issue occurred; retry later. | Contact support <a href=\"https://twelvedata.com/contact\">here</a> for assistance. |  ### Example error response  Consider the following invalid request:  ``` https://api.twelvedata.com/time_series?symbol=AAPL&interval=0.99min&apikey=your_api_key ```  Due to the incorrect `interval` value, the API returns:  ```json { &nbsp;&nbsp;\"code\": 400, &nbsp;&nbsp;\"message\": \"Invalid **interval** provided: 0.99min. Supported intervals: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h, 8h, 1day, 1week, 1month\", &nbsp;&nbsp;\"status\": \"error\" } ```  Refer to the API Documentation for valid parameter values to resolve such errors.  ## Libraries  Twelve Data provides a growing ecosystem of libraries and integrations to help you build faster and smarter in your preferred environment. Official libraries are actively maintained by the Twelve Data team, while selected community-built libraries offer additional flexibility.  A full list is available on our [GitHub profile](https://github.com/search?q=twelvedata).  ### Official SDKs <ul> <li><b>Python:</b> <a href=\"https://github.com/twelvedata/twelvedata-python\">twelvedata-python</a></li> <li><b>R:</b> <a href=\"https://github.com/twelvedata/twelvedata-r-sdk\">twelvedata-r-sdk</a></li> </ul>  ### AI integrations <ul> <li><b>Twelve Data MCP Server:</b> <a href=\"https://github.com/twelvedata/mcp\">Repository</a> — Model Context Protocol (MCP) server that provides seamless integration with AI assistants and language models, enabling direct access to Twelve Data's financial market data within conversational interfaces and AI workflows.</li> </ul>  ### Spreadsheet add-ons <ul> <li><b>Excel:</b> <a href=\"https://twelvedata.com/excel\">Excel Add-in</a></li> <li><b>Google Sheets:</b> <a href=\"https://twelvedata.com/google-sheets\">Google Sheets Add-on</a></li> </ul>  ### Community libraries  The community has developed libraries in several popular languages. You can explore more community libraries on [GitHub](https://github.com/search?q=twelvedata). <ul> <li><b>C#:</b> <a href=\"https://github.com/pseudomarkets/TwelveDataSharp\">TwelveDataSharp</a></li> <li><b>JavaScript:</b> <a href=\"https://github.com/evzaboun/twelvedata\">twelvedata</a></li> <li><b>PHP:</b> <a href=\"https://github.com/ingelby/twelvedata\">twelvedata</a></li> <li><b>Go:</b> <a href=\"https://github.com/soulgarden/twelvedata\">twelvedata</a></li> <li><b>TypeScript:</b> <a href=\"https://github.com/Clyde-Goodall/twelve-data-wrapper\">twelve-data-wrapper</a></li> </ul>  ### Other Twelve Data repositories <ul> <li><b>searchindex</b> <i>(Go)</i>: <a href=\"https://github.com/twelvedata/searchindex\">Repository</a> — In-memory search index by strings</li> <li><b>ws-tools</b> <i>(Python)</i>: <a href=\"https://github.com/twelvedata/ws-tools\">Repository</a> — Utility tools for WebSocket stream handling</li> </ul>  ### API specification <ul> <li><b>OpenAPI / Swagger:</b> Access the <a href=\"https://api.twelvedata.com/doc/swagger/openapi.json\">complete API specification</a> in OpenAPI format. You can use this file to automatically generate client libraries in your preferred programming language, explore the API interactively via Swagger tools, or integrate Twelve Data seamlessly into your AI and LLM workflows.</li> </ul>
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AssetsInfoNonCurrentAssets : Non-current assets information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetsInfoNonCurrentAssets {
    /// Total non current assets
    #[serde(rename = "total_non_current_assets", skip_serializing_if = "Option::is_none")]
    pub total_non_current_assets: Option<f64>,
    /// Financial assets
    #[serde(rename = "financial_assets", skip_serializing_if = "Option::is_none")]
    pub financial_assets: Option<f64>,
    /// Investments and advances
    #[serde(rename = "investments_and_advances", skip_serializing_if = "Option::is_none")]
    pub investments_and_advances: Option<f64>,
    /// Other investments
    #[serde(rename = "other_investments", skip_serializing_if = "Option::is_none")]
    pub other_investments: Option<f64>,
    /// Investment in financial assets
    #[serde(rename = "investment_in_financial_assets", skip_serializing_if = "Option::is_none")]
    pub investment_in_financial_assets: Option<f64>,
    /// Held to maturity securities
    #[serde(rename = "held_to_maturity_securities", skip_serializing_if = "Option::is_none")]
    pub held_to_maturity_securities: Option<f64>,
    /// Available for sale securities
    #[serde(rename = "available_for_sale_securities", skip_serializing_if = "Option::is_none")]
    pub available_for_sale_securities: Option<f64>,
    /// Financial assets designated as fair value through profit or loss total
    #[serde(rename = "financial_assets_designated_as_fair_value_through_profit_or_loss_total", skip_serializing_if = "Option::is_none")]
    pub financial_assets_designated_as_fair_value_through_profit_or_loss_total: Option<f64>,
    /// Trading securities
    #[serde(rename = "trading_securities", skip_serializing_if = "Option::is_none")]
    pub trading_securities: Option<f64>,
    /// Long term equity investment
    #[serde(rename = "long_term_equity_investment", skip_serializing_if = "Option::is_none")]
    pub long_term_equity_investment: Option<f64>,
    /// Investments in joint ventures at cost
    #[serde(rename = "investments_in_joint_ventures_at_cost", skip_serializing_if = "Option::is_none")]
    pub investments_in_joint_ventures_at_cost: Option<f64>,
    /// Investments in other ventures under equity method
    #[serde(rename = "investments_in_other_ventures_under_equity_method", skip_serializing_if = "Option::is_none")]
    pub investments_in_other_ventures_under_equity_method: Option<f64>,
    /// Investments in associates at cost
    #[serde(rename = "investments_in_associates_at_cost", skip_serializing_if = "Option::is_none")]
    pub investments_in_associates_at_cost: Option<f64>,
    /// Investments in subsidiaries at cost
    #[serde(rename = "investments_in_subsidiaries_at_cost", skip_serializing_if = "Option::is_none")]
    pub investments_in_subsidiaries_at_cost: Option<f64>,
    /// Investment properties
    #[serde(rename = "investment_properties", skip_serializing_if = "Option::is_none")]
    pub investment_properties: Option<f64>,
    #[serde(rename = "goodwill_and_other_intangible_assets", skip_serializing_if = "Option::is_none")]
    pub goodwill_and_other_intangible_assets: Option<Box<models::AssetsInfoNonCurrentAssetsGoodwillAndOtherIntangibleAssets>>,
    /// Net ppe
    #[serde(rename = "net_ppe", skip_serializing_if = "Option::is_none")]
    pub net_ppe: Option<f64>,
    /// Gross ppe
    #[serde(rename = "gross_ppe", skip_serializing_if = "Option::is_none")]
    pub gross_ppe: Option<f64>,
    /// Accumulated depreciation
    #[serde(rename = "accumulated_depreciation", skip_serializing_if = "Option::is_none")]
    pub accumulated_depreciation: Option<f64>,
    /// Leases
    #[serde(rename = "leases", skip_serializing_if = "Option::is_none")]
    pub leases: Option<f64>,
    /// Construction in progress
    #[serde(rename = "construction_in_progress", skip_serializing_if = "Option::is_none")]
    pub construction_in_progress: Option<f64>,
    /// Other properties
    #[serde(rename = "other_properties", skip_serializing_if = "Option::is_none")]
    pub other_properties: Option<f64>,
    /// Machinery furniture equipment
    #[serde(rename = "machinery_furniture_equipment", skip_serializing_if = "Option::is_none")]
    pub machinery_furniture_equipment: Option<f64>,
    /// Buildings and improvements
    #[serde(rename = "buildings_and_improvements", skip_serializing_if = "Option::is_none")]
    pub buildings_and_improvements: Option<f64>,
    /// Land and improvements
    #[serde(rename = "land_and_improvements", skip_serializing_if = "Option::is_none")]
    pub land_and_improvements: Option<f64>,
    /// Properties
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<f64>,
    /// Non current accounts receivable
    #[serde(rename = "non_current_accounts_receivable", skip_serializing_if = "Option::is_none")]
    pub non_current_accounts_receivable: Option<f64>,
    /// Non current note receivables
    #[serde(rename = "non_current_note_receivables", skip_serializing_if = "Option::is_none")]
    pub non_current_note_receivables: Option<f64>,
    /// Due from related parties non current
    #[serde(rename = "due_from_related_parties_non_current", skip_serializing_if = "Option::is_none")]
    pub due_from_related_parties_non_current: Option<f64>,
    /// Non current prepaid assets
    #[serde(rename = "non_current_prepaid_assets", skip_serializing_if = "Option::is_none")]
    pub non_current_prepaid_assets: Option<f64>,
    /// Non current deferred assets
    #[serde(rename = "non_current_deferred_assets", skip_serializing_if = "Option::is_none")]
    pub non_current_deferred_assets: Option<f64>,
    /// Non current deferred taxes assets
    #[serde(rename = "non_current_deferred_taxes_assets", skip_serializing_if = "Option::is_none")]
    pub non_current_deferred_taxes_assets: Option<f64>,
    /// Defined pension benefit
    #[serde(rename = "defined_pension_benefit", skip_serializing_if = "Option::is_none")]
    pub defined_pension_benefit: Option<f64>,
    /// Other non current assets
    #[serde(rename = "other_non_current_assets", skip_serializing_if = "Option::is_none")]
    pub other_non_current_assets: Option<f64>,
}

impl AssetsInfoNonCurrentAssets {
    /// Non-current assets information
    pub fn new() -> AssetsInfoNonCurrentAssets {
        AssetsInfoNonCurrentAssets {
            total_non_current_assets: None,
            financial_assets: None,
            investments_and_advances: None,
            other_investments: None,
            investment_in_financial_assets: None,
            held_to_maturity_securities: None,
            available_for_sale_securities: None,
            financial_assets_designated_as_fair_value_through_profit_or_loss_total: None,
            trading_securities: None,
            long_term_equity_investment: None,
            investments_in_joint_ventures_at_cost: None,
            investments_in_other_ventures_under_equity_method: None,
            investments_in_associates_at_cost: None,
            investments_in_subsidiaries_at_cost: None,
            investment_properties: None,
            goodwill_and_other_intangible_assets: None,
            net_ppe: None,
            gross_ppe: None,
            accumulated_depreciation: None,
            leases: None,
            construction_in_progress: None,
            other_properties: None,
            machinery_furniture_equipment: None,
            buildings_and_improvements: None,
            land_and_improvements: None,
            properties: None,
            non_current_accounts_receivable: None,
            non_current_note_receivables: None,
            due_from_related_parties_non_current: None,
            non_current_prepaid_assets: None,
            non_current_deferred_assets: None,
            non_current_deferred_taxes_assets: None,
            defined_pension_benefit: None,
            other_non_current_assets: None,
        }
    }
}

