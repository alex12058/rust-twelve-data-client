# \FundamentalsApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_balance_sheet**](FundamentalsApi.md#get_balance_sheet) | **GET** /balance_sheet | Balance sheet
[**get_balance_sheet_consolidated**](FundamentalsApi.md#get_balance_sheet_consolidated) | **GET** /balance_sheet/consolidated | Balance sheet consolidated
[**get_cash_flow**](FundamentalsApi.md#get_cash_flow) | **GET** /cash_flow | Cash flow
[**get_cash_flow_consolidated**](FundamentalsApi.md#get_cash_flow_consolidated) | **GET** /cash_flow/consolidated | Cash flow consolidated
[**get_dividends**](FundamentalsApi.md#get_dividends) | **GET** /dividends | Dividends
[**get_dividends_calendar**](FundamentalsApi.md#get_dividends_calendar) | **GET** /dividends_calendar | Dividends calendar
[**get_earnings**](FundamentalsApi.md#get_earnings) | **GET** /earnings | Earnings
[**get_earnings_calendar**](FundamentalsApi.md#get_earnings_calendar) | **GET** /earnings_calendar | Earnings calendar
[**get_income_statement**](FundamentalsApi.md#get_income_statement) | **GET** /income_statement | Income statement
[**get_income_statement_consolidated**](FundamentalsApi.md#get_income_statement_consolidated) | **GET** /income_statement/consolidated | Income statement consolidated
[**get_ipo_calendar**](FundamentalsApi.md#get_ipo_calendar) | **GET** /ipo_calendar | IPO calendar
[**get_key_executives**](FundamentalsApi.md#get_key_executives) | **GET** /key_executives | Key executives
[**get_last_changes**](FundamentalsApi.md#get_last_changes) | **GET** /last_change/{endpoint} | Last changes
[**get_logo**](FundamentalsApi.md#get_logo) | **GET** /logo | Logo
[**get_market_cap**](FundamentalsApi.md#get_market_cap) | **GET** /market_cap | Market capitalization
[**get_profile**](FundamentalsApi.md#get_profile) | **GET** /profile | Profile
[**get_splits**](FundamentalsApi.md#get_splits) | **GET** /splits | Splits
[**get_splits_calendar**](FundamentalsApi.md#get_splits_calendar) | **GET** /splits_calendar | Splits calendar
[**get_statistics**](FundamentalsApi.md#get_statistics) | **GET** /statistics | Statistics



## get_balance_sheet

> models::GetBalanceSheetResponse get_balance_sheet(symbol, figi, isin, cusip, exchange, mic_code, country, period, start_date, end_date, outputsize)
Balance sheet

The balance sheet endpoint provides a detailed financial statement for a company, outlining its assets, liabilities, and shareholders' equity. This endpoint returns structured data that includes current and non-current assets, total liabilities, and equity figures, enabling users to assess a company's financial health and stability.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**period** | Option<**String**> | The reporting period for the balane sheet data |  |[default to annual]
**start_date** | Option<**String**> | Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 6]

### Return type

[**models::GetBalanceSheetResponse**](GetBalanceSheet_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_balance_sheet_consolidated

> models::GetBalanceSheetConsolidatedResponse get_balance_sheet_consolidated(symbol, figi, isin, cusip, exchange, mic_code, country, period, start_date, end_date, outputsize)
Balance sheet consolidated

The balance sheet consolidated endpoint provides a detailed overview of a company's raw balance sheet, including a comprehensive summary of its assets, liabilities, and shareholders' equity. This endpoint is useful for retrieving financial data that reflects the overall financial position of a company, allowing users to access critical information about its financial health and structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**period** | Option<**String**> | The reporting period for the balance sheet data. |  |[default to annual]
**start_date** | Option<**String**> | Begin date for filtering items by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for filtering items by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 6]

### Return type

[**models::GetBalanceSheetConsolidatedResponse**](GetBalanceSheetConsolidated_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cash_flow

> models::GetCashFlowResponse get_cash_flow(symbol, figi, isin, cusip, exchange, mic_code, country, period, start_date, end_date, outputsize)
Cash flow

The cash flow endpoint provides detailed information on a company's cash flow activities, including the net cash and cash equivalents moving in and out of the business. This data includes operating, investing, and financing cash flows, offering a comprehensive view of the company's liquidity and financial health.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**period** | Option<**String**> | The reporting period for the cash flow statements |  |[default to annual]
**start_date** | Option<**String**> | Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 6]

### Return type

[**models::GetCashFlowResponse**](GetCashFlow_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cash_flow_consolidated

> models::GetCashFlowConsolidatedResponse get_cash_flow_consolidated(symbol, figi, isin, cusip, exchange, mic_code, country, period, start_date, end_date, outputsize)
Cash flow consolidated

The cash flow consolidated endpoint provides raw data on a company's consolidated cash flow, including the net cash and cash equivalents moving in and out of the business. It returns information on operating, investing, and financing activities, helping users track liquidity and financial health over a specified period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**period** | Option<**String**> | The reporting period for the cash flow statements |  |[default to annual]
**start_date** | Option<**String**> | Start date for filtering cash flow statements. Only cash flow statements with fiscal dates on or after this date will be included. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for filtering cash flow statements. Only cash flow statements with fiscal dates on or before this date will be included. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 6]

### Return type

[**models::GetCashFlowConsolidatedResponse**](GetCashFlowConsolidated_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dividends

> models::GetDividendsResponse get_dividends(symbol, figi, isin, cusip, exchange, mic_code, country, range, start_date, end_date, adjust)
Dividends

The dividends endpoint provides historical dividend data for a specified stock, in many cases covering over a decade. It returns information on dividend payouts, including the amount, payment date, and frequency. This endpoint is ideal for users tracking dividend histories or evaluating the income potential of stocks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**range** | Option<**String**> | Specifies the time range for which to retrieve dividend data. Accepts values such as `last` (most recent dividend), `next` (upcoming dividend), `1m` - `5y` for respective periods, or `full` for all available data. If provided together with `start_date` and/or `end_date`, this parameter takes precedence. |  |[default to last]
**start_date** | Option<**String**> | Start date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence. |  |
**end_date** | Option<**String**> | End date for the dividend data query. Only dividends with dates on or after this date will be returned. Format `2006-01-02`. If provided together with `range` parameter, `range` will take precedence. |  |
**adjust** | Option<**bool**> | Specifies if there should be an adjustment |  |[default to true]

### Return type

[**models::GetDividendsResponse**](GetDividends_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dividends_calendar

> models::GetDividendsCalendarResponse get_dividends_calendar(symbol, figi, isin, cusip, exchange, mic_code, country, start_date, end_date, outputsize, page)
Dividends calendar

The dividends calendar endpoint provides a detailed schedule of upcoming and past dividend events for specified date ranges. By using the `start_date` and `end_date` parameters, users can retrieve a list of companies issuing dividends, including the ex-dividend date, payment date, and dividend amount. This endpoint is ideal for tracking dividend payouts and planning investment strategies based on dividend schedules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**start_date** | Option<**String**> | Start date for the dividends calendar query. Only dividends with ex-dates on or after this date will be returned. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for the dividends calendar query. Only dividends with ex-dates on or before this date will be returned. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum |  |[default to 100]
**page** | Option<**i64**> | Page number |  |[default to 1]

### Return type

[**models::GetDividendsCalendarResponse**](GetDividendsCalendar_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_earnings

> models::GetEarningsResponse get_earnings(symbol, figi, isin, cusip, exchange, mic_code, country, r#type, period, outputsize, format, delimiter, start_date, end_date, dp)
Earnings

The earnings endpoint provides comprehensive earnings data for a specified company, including both the estimated and actual Earnings Per Share (EPS) figures. This endpoint delivers historical earnings information, allowing users to track a company's financial performance over time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**r#type** | Option<**String**> | The asset class to which the instrument belongs |  |
**period** | Option<**String**> | Type of earning, returns only 1 record. When is not empty, dates and outputsize parameters are ignored |  |
**outputsize** | Option<**i64**> | Number of data points to retrieve. Supports values in the range from `1` to `1000`. Default `10` when no date parameters are set, otherwise set to maximum |  |[default to 10]
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]
**start_date** | Option<**String**> | The date from which the data is requested. The date format is `YYYY-MM-DD`. |  |
**end_date** | Option<**String**> | The date to which the data is requested. The date format is `YYYY-MM-DD`. |  |
**dp** | Option<**i64**> | The number of decimal places in the response data. Should be in range [0,11] inclusive |  |[default to 2]

### Return type

[**models::GetEarningsResponse**](GetEarnings_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_earnings_calendar

> models::GetEarningsCalendarResponse get_earnings_calendar(exchange, mic_code, country, format, delimiter, start_date, end_date, dp)
Earnings calendar

The earnings calendar endpoint provides a schedule of company earnings announcements for a specified date range. By default, it returns earnings data for the current day. Users can customize the date range using the `start_date` and `end_date` parameters to retrieve earnings information for specific periods. This endpoint is useful for tracking upcoming earnings reports and planning around key financial announcements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**format** | Option<**String**> | Value can be JSON or CSV |  |[default to JSON]
**delimiter** | Option<**String**> | Specify the delimiter used when downloading the CSV file |  |[default to ;]
**start_date** | Option<**String**> | Can be used separately and together with end_date. Format `2006-01-02` or `2006-01-02T15:04:05` |  |
**end_date** | Option<**String**> | Can be used separately and together with start_date. Format `2006-01-02` or `2006-01-02T15:04:05` |  |
**dp** | Option<**i64**> | Specifies the number of decimal places for floating values. Should be in range [0,11] inclusive |  |[default to 2]

### Return type

[**models::GetEarningsCalendarResponse**](GetEarningsCalendar_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_income_statement

> models::GetIncomeStatementResponse get_income_statement(symbol, figi, isin, cusip, exchange, mic_code, country, period, start_date, end_date, outputsize)
Income statement

The income statement endpoint provides detailed financial data on a company's income statement, including revenues, expenses, and net income for specified periods, either annually or quarterly. This endpoint is essential for retrieving comprehensive financial performance metrics of a company, allowing users to access historical and current financial results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**period** | Option<**String**> | The reporting period for the income statement data |  |
**start_date** | Option<**String**> | Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 6]

### Return type

[**models::GetIncomeStatementResponse**](GetIncomeStatement_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_income_statement_consolidated

> models::GetIncomeStatementConsolidatedResponse get_income_statement_consolidated(symbol, figi, isin, cusip, exchange, mic_code, country, period, start_date, end_date, outputsize)
Income statement consolidated

The income statement consolidated endpoint provides a company's raw income statement, detailing revenue, expenses, and net income for specified periods, either annually or quarterly. This data is essential for evaluating a company's financial performance over time, allowing users to access comprehensive financial results in a structured format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**period** | Option<**String**> | The reporting period for the income statement data |  |
**start_date** | Option<**String**> | Begin date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or after this date. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for filtering income statements by fiscal date. Returns income statements with fiscal dates on or before this date. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 6]

### Return type

[**models::GetIncomeStatementConsolidatedResponse**](GetIncomeStatementConsolidated_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ipo_calendar

> models::GetIpoCalendarResponse get_ipo_calendar(exchange, mic_code, country, start_date, end_date)
IPO calendar

The IPO Calendar endpoint provides detailed information on initial public offerings (IPOs), including those that have occurred in the past, are happening today, or are scheduled for the future. Users can access data such as company names, IPO dates, and offering details, allowing them to track and monitor IPO activity efficiently.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**start_date** | Option<**String**> | The earliest IPO date to include in the results. Format: `2006-01-02` |  |
**end_date** | Option<**String**> | The latest IPO date to include in the results. Format: `2006-01-02` |  |

### Return type

[**models::GetIpoCalendarResponse**](GetIpoCalendar_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_key_executives

> models::GetKeyExecutivesResponse get_key_executives(symbol, figi, isin, cusip, exchange, mic_code, country)
Key executives

The key executives endpoint provides detailed information about a company's key executives identified by a specific stock symbol. It returns data such as names, titles, and roles of the executives, which can be useful for understanding the leadership structure of the company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |

### Return type

[**models::GetKeyExecutivesResponse**](GetKeyExecutives_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_last_changes

> models::GetLastChangesResponse get_last_changes(endpoint, start_date, symbol, exchange, mic_code, country, page, outputsize)
Last changes

The last change endpoint provides the most recent updates to fundamental data for a specified dataset. It returns a timestamp indicating when the data was last modified, allowing users to efficiently manage API requests by only fetching new data when changes occur. This helps optimize data retrieval and reduce unnecessary API credit usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | **String** | Endpoint name | [required] |
**start_date** | Option<**String**> | The starting date and time for data selection, in `2006-01-02T15:04:05` format |  |
**symbol** | Option<**String**> | Filter by symbol |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**mic_code** | Option<**String**> | Filter by market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**page** | Option<**i64**> | Page number |  |[default to 1]
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 30]

### Return type

[**models::GetLastChangesResponse**](GetLastChanges_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logo

> models::GetLogoResponse get_logo(symbol, exchange, mic_code, country)
Logo

The logo endpoint provides the official logo image for a specified company, cryptocurrency, or forex pair. This endpoint is useful for integrating visual branding elements into financial applications, websites, or reports, ensuring that users can easily identify and associate the correct logo with the respective financial asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `BTC/USD`, `EUR/USD`. | [required] |
**exchange** | Option<**String**> | The exchange name where the instrument is traded, e.g., `NASDAQ`, `NSE` |  |
**mic_code** | Option<**String**> | The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON` |  |
**country** | Option<**String**> | The country where the instrument is traded, e.g., `United States` or `US` |  |

### Return type

[**models::GetLogoResponse**](GetLogo_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_cap

> models::GetMarketCapResponse get_market_cap(symbol, figi, isin, cusip, exchange, mic_code, country, start_date, end_date, page, outputsize)
Market capitalization

The Market Capitalization History endpoint provides historical data on a company's market capitalization over a specified time period. It returns a time series of market cap values, allowing users to track changes in a company's market value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**mic_code** | Option<**String**> | Filter by market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**start_date** | Option<**String**> | Start date for market capitalization data retrieval. Data will be returned from this date onwards. Format `2006-01-02` |  |
**end_date** | Option<**String**> | End date for market capitalization data retrieval. Data will be returned up to and including this date. Format `2006-01-02` |  |
**page** | Option<**i64**> | Page number |  |[default to 1]
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 10]

### Return type

[**models::GetMarketCapResponse**](GetMarketCap_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile

> models::GetProfileResponse get_profile(symbol, figi, isin, cusip, exchange, mic_code, country)
Profile

The profile endpoint provides detailed company information, including the company's name, industry, sector, CEO, headquarters location, and market capitalization. This data is useful for obtaining a comprehensive overview of a company's business and financial standing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |

### Return type

[**models::GetProfileResponse**](GetProfile_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_splits

> models::GetSplitsResponse get_splits(symbol, figi, isin, cusip, exchange, mic_code, country, range, start_date, end_date)
Splits

The splits endpoint provides historical data on stock split events for a specified company. It returns details including the date of each split and the corresponding split factor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**range** | Option<**String**> | Range of data to be returned |  |[default to last]
**start_date** | Option<**String**> | The starting date for data selection. Format `2006-01-02` |  |
**end_date** | Option<**String**> | The ending date for data selection. Format `2006-01-02` |  |

### Return type

[**models::GetSplitsResponse**](GetSplits_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_splits_calendar

> models::GetSplitsCalendarResponse get_splits_calendar(symbol, figi, isin, cusip, exchange, mic_code, country, start_date, end_date, outputsize, page)
Splits calendar

The splits calendar endpoint provides a detailed calendar of stock split events within a specified date range. By setting the `start_date` and `end_date` parameters, users can retrieve a list of upcoming or past stock splits, including the company name, split ratio, and effective date. This endpoint is useful for tracking changes in stock structure and planning investment strategies around these events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**start_date** | Option<**String**> | The starting date (inclusive) for filtering split events in the calendar. Format `2006-01-02` |  |
**end_date** | Option<**String**> | The ending date (inclusive) for filtering split events in the calendar. Format `2006-01-02` |  |
**outputsize** | Option<**i64**> | Number of data points to retrieve. Supports values in the range from `1` to `500`. Default `100` when no date parameters are set, otherwise set to maximum |  |[default to 100]
**page** | Option<**String**> | Page number |  |[default to 1]

### Return type

[**models::GetSplitsCalendarResponse**](GetSplitsCalendar_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_statistics

> models::GetStatisticsResponse get_statistics(symbol, figi, isin, cusip, exchange, mic_code, country)
Statistics

The statistics endpoint provides a comprehensive snapshot of a company's key financial statistics, including valuation metrics, revenue figures, profit margins, and other essential financial data. This endpoint is ideal for users seeking detailed insights into a company's financial health and performance metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |

### Return type

[**models::GetStatisticsResponse**](GetStatistics_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

