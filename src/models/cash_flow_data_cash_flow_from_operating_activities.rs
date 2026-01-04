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

/// CashFlowDataCashFlowFromOperatingActivities : Cash flow from operating activities
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CashFlowDataCashFlowFromOperatingActivities {
    /// Net income from continuing operations
    #[serde(
        rename = "net_income_from_continuing_operations",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_income_from_continuing_operations: Option<f64>,
    /// Operating cash flow
    #[serde(
        rename = "operating_cash_flow",
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_cash_flow: Option<f64>,
    /// Cash flow from continuing operating activities
    #[serde(
        rename = "cash_flow_from_continuing_operating_activities",
        skip_serializing_if = "Option::is_none"
    )]
    pub cash_flow_from_continuing_operating_activities: Option<f64>,
    /// Cash from discontinued operating activities
    #[serde(
        rename = "cash_from_discontinued_operating_activities",
        skip_serializing_if = "Option::is_none"
    )]
    pub cash_from_discontinued_operating_activities: Option<f64>,
    /// Cash flow from discontinued operation
    #[serde(
        rename = "cash_flow_from_discontinued_operation",
        skip_serializing_if = "Option::is_none"
    )]
    pub cash_flow_from_discontinued_operation: Option<f64>,
    /// Free cash flow
    #[serde(rename = "free_cash_flow", skip_serializing_if = "Option::is_none")]
    pub free_cash_flow: Option<f64>,
    /// Cash flows from used in operating activities direct
    #[serde(
        rename = "cash_flows_from_used_in_operating_activities_direct",
        skip_serializing_if = "Option::is_none"
    )]
    pub cash_flows_from_used_in_operating_activities_direct: Option<f64>,
    /// Taxes refund paid
    #[serde(rename = "taxes_refund_paid", skip_serializing_if = "Option::is_none")]
    pub taxes_refund_paid: Option<f64>,
    /// Taxes refund paid direct
    #[serde(
        rename = "taxes_refund_paid_direct",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxes_refund_paid_direct: Option<f64>,
    /// Interest received
    #[serde(rename = "interest_received", skip_serializing_if = "Option::is_none")]
    pub interest_received: Option<f64>,
    /// Interest received direct
    #[serde(
        rename = "interest_received_direct",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_received_direct: Option<f64>,
    /// Interest paid
    #[serde(rename = "interest_paid", skip_serializing_if = "Option::is_none")]
    pub interest_paid: Option<f64>,
    /// Interest paid direct
    #[serde(
        rename = "interest_paid_direct",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_paid_direct: Option<f64>,
    /// Dividend received
    #[serde(rename = "dividend_received", skip_serializing_if = "Option::is_none")]
    pub dividend_received: Option<f64>,
    /// Dividend received direct
    #[serde(
        rename = "dividend_received_direct",
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_received_direct: Option<f64>,
    /// Dividend paid
    #[serde(rename = "dividend_paid", skip_serializing_if = "Option::is_none")]
    pub dividend_paid: Option<f64>,
    /// Dividend paid direct
    #[serde(
        rename = "dividend_paid_direct",
        skip_serializing_if = "Option::is_none"
    )]
    pub dividend_paid_direct: Option<f64>,
    /// Change in working capital
    #[serde(
        rename = "change_in_working_capital",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_working_capital: Option<f64>,
    /// Change in other working capital
    #[serde(
        rename = "change_in_other_working_capital",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_other_working_capital: Option<f64>,
    /// Change in receivables
    #[serde(
        rename = "change_in_receivables",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_receivables: Option<f64>,
    /// Changes in account receivables
    #[serde(
        rename = "changes_in_account_receivables",
        skip_serializing_if = "Option::is_none"
    )]
    pub changes_in_account_receivables: Option<f64>,
    /// Change in payables and accrued expense
    #[serde(
        rename = "change_in_payables_and_accrued_expense",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_payables_and_accrued_expense: Option<f64>,
    /// Change in accrued expense
    #[serde(
        rename = "change_in_accrued_expense",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_accrued_expense: Option<f64>,
    /// Change in payable
    #[serde(rename = "change_in_payable", skip_serializing_if = "Option::is_none")]
    pub change_in_payable: Option<f64>,
    /// Change in dividend payable
    #[serde(
        rename = "change_in_dividend_payable",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_dividend_payable: Option<f64>,
    /// Change in account payable
    #[serde(
        rename = "change_in_account_payable",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_account_payable: Option<f64>,
    /// Change in tax payable
    #[serde(
        rename = "change_in_tax_payable",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_tax_payable: Option<f64>,
    /// Change in income tax payable
    #[serde(
        rename = "change_in_income_tax_payable",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_income_tax_payable: Option<f64>,
    /// Change in interest payable
    #[serde(
        rename = "change_in_interest_payable",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_interest_payable: Option<f64>,
    /// Change in other current liabilities
    #[serde(
        rename = "change_in_other_current_liabilities",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_other_current_liabilities: Option<f64>,
    /// Change in other current assets
    #[serde(
        rename = "change_in_other_current_assets",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_other_current_assets: Option<f64>,
    /// Change in inventory
    #[serde(
        rename = "change_in_inventory",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_inventory: Option<f64>,
    /// Change in prepaid assets
    #[serde(
        rename = "change_in_prepaid_assets",
        skip_serializing_if = "Option::is_none"
    )]
    pub change_in_prepaid_assets: Option<f64>,
    /// Other non cash items
    #[serde(
        rename = "other_non_cash_items",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_non_cash_items: Option<f64>,
    /// Excess tax benefit from stock based compensation
    #[serde(
        rename = "excess_tax_benefit_from_stock_based_compensation",
        skip_serializing_if = "Option::is_none"
    )]
    pub excess_tax_benefit_from_stock_based_compensation: Option<f64>,
    /// Stock based compensation
    #[serde(
        rename = "stock_based_compensation",
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_based_compensation: Option<f64>,
    /// Unrealized gain loss on investment securities
    #[serde(
        rename = "unrealized_gain_loss_on_investment_securities",
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_gain_loss_on_investment_securities: Option<f64>,
    /// Provision and write off of assets
    #[serde(
        rename = "provision_and_write_off_of_assets",
        skip_serializing_if = "Option::is_none"
    )]
    pub provision_and_write_off_of_assets: Option<f64>,
    /// Asset impairment charge
    #[serde(
        rename = "asset_impairment_charge",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_impairment_charge: Option<f64>,
    /// Amortization of securities
    #[serde(
        rename = "amortization_of_securities",
        skip_serializing_if = "Option::is_none"
    )]
    pub amortization_of_securities: Option<f64>,
    /// Deferred tax
    #[serde(rename = "deferred_tax", skip_serializing_if = "Option::is_none")]
    pub deferred_tax: Option<f64>,
    /// Deferred income tax
    #[serde(
        rename = "deferred_income_tax",
        skip_serializing_if = "Option::is_none"
    )]
    pub deferred_income_tax: Option<f64>,
    /// Depreciation amortization depletion
    #[serde(
        rename = "depreciation_amortization_depletion",
        skip_serializing_if = "Option::is_none"
    )]
    pub depreciation_amortization_depletion: Option<f64>,
    /// Depletion
    #[serde(rename = "depletion", skip_serializing_if = "Option::is_none")]
    pub depletion: Option<f64>,
    /// Depreciation and amortization
    #[serde(
        rename = "depreciation_and_amortization",
        skip_serializing_if = "Option::is_none"
    )]
    pub depreciation_and_amortization: Option<f64>,
    /// Amortization cash flow
    #[serde(
        rename = "amortization_cash_flow",
        skip_serializing_if = "Option::is_none"
    )]
    pub amortization_cash_flow: Option<f64>,
    /// Amortization of intangibles
    #[serde(
        rename = "amortization_of_intangibles",
        skip_serializing_if = "Option::is_none"
    )]
    pub amortization_of_intangibles: Option<f64>,
    /// Depreciation
    #[serde(rename = "depreciation", skip_serializing_if = "Option::is_none")]
    pub depreciation: Option<f64>,
    /// Operating gains losses
    #[serde(
        rename = "operating_gains_losses",
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_gains_losses: Option<f64>,
    /// Pension and employee benefit expense
    #[serde(
        rename = "pension_and_employee_benefit_expense",
        skip_serializing_if = "Option::is_none"
    )]
    pub pension_and_employee_benefit_expense: Option<f64>,
    /// Earnings losses from equity investments
    #[serde(
        rename = "earnings_losses_from_equity_investments",
        skip_serializing_if = "Option::is_none"
    )]
    pub earnings_losses_from_equity_investments: Option<f64>,
    /// Gain loss on investment securities
    #[serde(
        rename = "gain_loss_on_investment_securities",
        skip_serializing_if = "Option::is_none"
    )]
    pub gain_loss_on_investment_securities: Option<f64>,
    /// Net foreign currency exchange gain loss
    #[serde(
        rename = "net_foreign_currency_exchange_gain_loss",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_foreign_currency_exchange_gain_loss: Option<f64>,
    /// Gain loss on sale of ppe
    #[serde(
        rename = "gain_loss_on_sale_of_ppe",
        skip_serializing_if = "Option::is_none"
    )]
    pub gain_loss_on_sale_of_ppe: Option<f64>,
    /// Gain loss on sale of business
    #[serde(
        rename = "gain_loss_on_sale_of_business",
        skip_serializing_if = "Option::is_none"
    )]
    pub gain_loss_on_sale_of_business: Option<f64>,
}

impl CashFlowDataCashFlowFromOperatingActivities {
    /// Cash flow from operating activities
    pub fn new() -> CashFlowDataCashFlowFromOperatingActivities {
        CashFlowDataCashFlowFromOperatingActivities {
            net_income_from_continuing_operations: None,
            operating_cash_flow: None,
            cash_flow_from_continuing_operating_activities: None,
            cash_from_discontinued_operating_activities: None,
            cash_flow_from_discontinued_operation: None,
            free_cash_flow: None,
            cash_flows_from_used_in_operating_activities_direct: None,
            taxes_refund_paid: None,
            taxes_refund_paid_direct: None,
            interest_received: None,
            interest_received_direct: None,
            interest_paid: None,
            interest_paid_direct: None,
            dividend_received: None,
            dividend_received_direct: None,
            dividend_paid: None,
            dividend_paid_direct: None,
            change_in_working_capital: None,
            change_in_other_working_capital: None,
            change_in_receivables: None,
            changes_in_account_receivables: None,
            change_in_payables_and_accrued_expense: None,
            change_in_accrued_expense: None,
            change_in_payable: None,
            change_in_dividend_payable: None,
            change_in_account_payable: None,
            change_in_tax_payable: None,
            change_in_income_tax_payable: None,
            change_in_interest_payable: None,
            change_in_other_current_liabilities: None,
            change_in_other_current_assets: None,
            change_in_inventory: None,
            change_in_prepaid_assets: None,
            other_non_cash_items: None,
            excess_tax_benefit_from_stock_based_compensation: None,
            stock_based_compensation: None,
            unrealized_gain_loss_on_investment_securities: None,
            provision_and_write_off_of_assets: None,
            asset_impairment_charge: None,
            amortization_of_securities: None,
            deferred_tax: None,
            deferred_income_tax: None,
            depreciation_amortization_depletion: None,
            depletion: None,
            depreciation_and_amortization: None,
            amortization_cash_flow: None,
            amortization_of_intangibles: None,
            depreciation: None,
            operating_gains_losses: None,
            pension_and_employee_benefit_expense: None,
            earnings_losses_from_equity_investments: None,
            gain_loss_on_investment_securities: None,
            net_foreign_currency_exchange_gain_loss: None,
            gain_loss_on_sale_of_ppe: None,
            gain_loss_on_sale_of_business: None,
        }
    }
}
