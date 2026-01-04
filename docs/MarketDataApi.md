# \MarketDataApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_currency_conversion**](MarketDataApi.md#get_currency_conversion) | **GET** /currency_conversion | Currency conversion
[**get_eod**](MarketDataApi.md#get_eod) | **GET** /eod | End of day price
[**get_exchange_rate**](MarketDataApi.md#get_exchange_rate) | **GET** /exchange_rate | Exchange rate
[**get_market_movers**](MarketDataApi.md#get_market_movers) | **GET** /market_movers/{market} | Market movers
[**get_price**](MarketDataApi.md#get_price) | **GET** /price | Latest price
[**get_quote**](MarketDataApi.md#get_quote) | **GET** /quote | Quote
[**get_time_series**](MarketDataApi.md#get_time_series) | **GET** /time_series | Time series
[**get_time_series_cross**](MarketDataApi.md#get_time_series_cross) | **GET** /time_series/cross | Time series cross



## get_currency_conversion

> models::GetCurrencyConversion200ResponseEnum get_currency_conversion(symbol, amount, date, format, delimiter, dp, timezone)
Currency conversion

The currency conversion endpoint provides real-time exchange rates and calculates the converted amount for specified currency pairs, including both forex and cryptocurrencies. This endpoint is useful for obtaining up-to-date conversion values between two currencies, facilitating tasks such as financial reporting, e-commerce transactions, and travel budgeting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | The currency pair you want to request can be either forex or cryptocurrency. Slash(`/`) delimiter is used. E.g. `EUR/USD` or `BTC/ETH` will be correct | [required] |
**amount** | **f64** | Amount of base currency to be converted into quote currency. Supports values in the range from `0` and above | [required] |
**date** | Option<**String**> | If not null, will use exchange rate from a specific date or time. Format `2006-01-02` or `2006-01-02T15:04:05`. Is set in the local exchange time zone, use timezone parameter to specify a specific time zone |  |
**format** | Option<**String**> | Value can be `JSON` or `CSV`. Default `JSON` |  |[default to JSON]
**delimiter** | Option<**String**> | Specify the delimiter used when downloading the `CSV` file. Default semicolon `;` |  |[default to ;]
**dp** | Option<**i64**> | The number of decimal places for the data |  |[default to 5]
**timezone** | Option<**String**> | Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i> |  |

### Return type

[**models::GetCurrencyConversion200ResponseEnum**](GetCurrencyConversion_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eod

> models::GetEod200ResponseEnum get_eod(symbol, figi, isin, cusip, exchange, mic_code, country, r#type, date, prepost, dp)
End of day price

The End of Day (EOD) Prices endpoint provides the closing price and other relevant metadata for a financial instrument at the end of a trading day. This endpoint is useful for retrieving daily historical data for stocks, ETFs, or other securities, allowing users to track performance over time and compare daily market movements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of the instrument |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**r#type** | Option<**String**> | The asset class to which the instrument belongs |  |
**date** | Option<**String**> | If not null, then return data from a specific date |  |
**prepost** | Option<**bool**> | Parameter is optional. Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume |  |[default to false]
**dp** | Option<**i64**> | Specifies the number of decimal places for floating values Should be in range [0,11] inclusive |  |[default to 5]

### Return type

[**models::GetEod200ResponseEnum**](GetEod_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_rate

> models::GetExchangeRate200ResponseEnum get_exchange_rate(symbol, date, format, delimiter, dp, timezone)
Exchange rate

The exchange rate endpoint provides real-time exchange rates for specified currency pairs, including both forex and cryptocurrency. It returns the current exchange rate value between two currencies, allowing users to quickly access up-to-date conversion rates for financial transactions or market analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | The currency pair you want to request can be either forex or cryptocurrency. Slash(`/`) delimiter is used. E.g. `EUR/USD` or `BTC/ETH` will be correct | [required] |
**date** | Option<**String**> | If not null, will use exchange rate from a specific date or time. Format `2006-01-02` or `2006-01-02T15:04:05`. Is set in the local exchange time zone, use timezone parameter to specify a specific time zone |  |
**format** | Option<**String**> | Value can be `JSON` or `CSV`. Default `JSON` |  |[default to JSON]
**delimiter** | Option<**String**> | Specify the delimiter used when downloading the `CSV` file. Default semicolon `;` |  |[default to ;]
**dp** | Option<**i64**> | The number of decimal places for the data |  |[default to 5]
**timezone** | Option<**String**> | Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i> |  |

### Return type

[**models::GetExchangeRate200ResponseEnum**](GetExchangeRate_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_movers

> models::GetMarketMovers200Response get_market_movers(market, direction, outputsize, country, price_greater_than, dp)
Market movers

The market movers endpoint provides a ranked list of the top-gaining and losing assets for the current trading day. It returns detailed data on the highest percentage price increases and decreases since the previous day's close. This endpoint supports international equities, forex, and cryptocurrencies, enabling users to quickly identify significant market movements across various asset classes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | **String** | Maket type | [required] |
**direction** | Option<**String**> | Specifies direction of the snapshot gainers or losers |  |[default to gainers]
**outputsize** | Option<**i64**> | Specifies the size of the snapshot. Can be in a range from `1` to `50` |  |[default to 30]
**country** | Option<**String**> | Country of the snapshot, applicable to non-currencies only. Takes country name or alpha code |  |[default to USA]
**price_greater_than** | Option<**String**> | Takes values with price grater than specified value |  |
**dp** | Option<**String**> | Specifies the number of decimal places for floating values. Should be in range [0,11] inclusive |  |[default to 5]

### Return type

[**models::GetMarketMovers200Response**](GetMarketMovers_200_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_price

> models::GetPrice200ResponseEnum get_price(symbol, figi, isin, cusip, exchange, mic_code, country, r#type, format, delimiter, prepost, dp)
Latest price

The latest price endpoint provides the latest market price for a specified financial instrument. It returns a single data point representing the current (or the most recently available) trading price.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of the instrument |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**r#type** | Option<**String**> | The asset class to which the instrument belongs |  |
**format** | Option<**String**> | Value can be JSON or CSV |  |[default to JSON]
**delimiter** | Option<**String**> | Specify the delimiter used when downloading the CSV file |  |[default to ;]
**prepost** | Option<**bool**> | Parameter is optional. Only for Pro and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume. |  |[default to false]
**dp** | Option<**i64**> | Specifies the number of decimal places for floating values. Should be in range [0,11] inclusive |  |[default to 5]

### Return type

[**models::GetPrice200ResponseEnum**](GetPrice_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quote

> models::GetQuote200ResponseEnum get_quote(symbol, figi, isin, cusip, interval, exchange, mic_code, country, volume_time_period, r#type, format, delimiter, prepost, eod, rolling_period, dp, timezone)
Quote

The quote endpoint provides real-time data for a selected financial instrument, returning essential information such as the latest price, open, high, low, close, volume, and price change. This endpoint is ideal for users needing up-to-date market data to track price movements and trading activity for specific stocks, ETFs, or other securities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of the instrument |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**interval** | Option<**String**> | Interval of the quote |  |[default to 1day]
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |
**volume_time_period** | Option<**i64**> | Number of periods for Average Volume |  |[default to 9]
**r#type** | Option<**String**> | The asset class to which the instrument belongs |  |
**format** | Option<**String**> | Value can be JSON or CSV Default JSON |  |[default to JSON]
**delimiter** | Option<**String**> | Specify the delimiter used when downloading the CSV file |  |[default to ;]
**prepost** | Option<**bool**> | Parameter is optional. Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume. |  |[default to false]
**eod** | Option<**bool**> | If true, then return data for closed day |  |[default to false]
**rolling_period** | Option<**i64**> | Number of hours for calculate rolling change at period. By default set to 24, it can be in range [1, 168]. |  |[default to 24]
**dp** | Option<**i64**> | Specifies the number of decimal places for floating values Should be in range [0,11] inclusive |  |[default to 5]
**timezone** | Option<**String**> | Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i> |  |[default to Exchange]

### Return type

[**models::GetQuote200ResponseEnum**](GetQuote_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_series

> models::GetTimeSeries200ResponseEnum get_time_series(interval, symbol, isin, figi, cusip, outputsize, exchange, mic_code, country, r#type, timezone, start_date, end_date, date, order, prepost, format, delimiter, dp, previous_close, adjust)
Time series

The time series endpoint provides detailed historical data for a specified financial instrument. It returns two main components: metadata, which includes essential information about the instrument, and a time series dataset. The time series consists of chronological entries with Open, High, Low, and Close prices, and for applicable instruments, it also includes trading volume. This endpoint is ideal for retrieving comprehensive historical price data for analysis or visualization purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | **String** | Interval between two consecutive points in time series | [required] |
**symbol** | Option<**String**> | Symbol ticker of the instrument. E.g. `AAPL`, `EUR/USD`, `ETH/BTC`, ... |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**figi** | Option<**String**> | The FIGI of an instrument for which data is requested |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**outputsize** | Option<**i64**> | Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum |  |[default to 30]
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | The country where the instrument is traded, e.g., `United States` or `US` |  |
**r#type** | Option<**String**> | The asset class to which the instrument belongs |  |
**timezone** | Option<**String**> | Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a></li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i> |  |[default to Exchange]
**start_date** | Option<**String**> | Can be used separately and together with `end_date`. Format `2006-01-02` or `2006-01-02T15:04:05`  Default location: <ul> <li>Forex and Cryptocurrencies - <code>UTC</code></li> <li>Stocks - where exchange is located (e.g. for AAPL it will be <code>America/New_York</code>)</li> </ul> Both parameters take into account if <code>timezone</code> parameter is provided.<br/> If <code>timezone</code> is given then, <code>start_date</code> and <code>end_date</code> will be used in the specified location  Examples: <ul> <li>1. <code>&symbol=AAPL&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 New York time up to current date</li> <li>2. <code>&symbol=EUR/USD&timezone=Asia/Singapore&start_date=2019-08-09T15:50:00&…</code><br/> Returns all records starting from 2019-08-09T15:50:00 Singapore time up to current date</li> <li>3. <code>&symbol=ETH/BTC&timezone=Europe/Zurich&start_date=2019-08-09T15:50:00&end_date=2019-08-09T15:55:00&...</code><br/> Returns all records starting from 2019-08-09T15:50:00 Zurich time up to 2019-08-09T15:55:00</li> </ul> |  |
**end_date** | Option<**String**> | The ending date and time for data selection, see `start_date` description for details. |  |
**date** | Option<**String**> | Specifies the exact date to get the data for. Could be the exact date, e.g. `2021-10-27`, or in human language `today` or `yesterday` |  |
**order** | Option<**String**> | Sorting order of the output |  |[default to desc]
**prepost** | Option<**bool**> | Returns quotes that include pre-market and post-market data. Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume |  |[default to false]
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]
**dp** | Option<**i64**> | Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive. By default, the number of decimal places is automatically determined based on the values provided |  |[default to -1]
**previous_close** | Option<**bool**> | A boolean parameter to include the previous closing price in the time_series data. If true, adds previous bar close price value to the current object |  |[default to false]
**adjust** | Option<**String**> | Adjusting mode for prices |  |[default to splits]

### Return type

[**models::GetTimeSeries200ResponseEnum**](GetTimeSeries_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_series_cross

> models::GetTimeSeriesCross200ResponseEnum get_time_series_cross(base, quote, interval, base_type, base_exchange, base_mic_code, quote_type, quote_exchange, quote_mic_code, outputsize, format, delimiter, prepost, start_date, end_date, adjust, dp, timezone)
Time series cross

The Time Series Cross endpoint calculates and returns historical cross-rate data for exotic forex pairs, cryptocurrencies, or stocks (e.g., Apple Inc. price in Indian Rupees) on the fly. It provides metadata about the requested symbol and a time series array with Open, High, Low, and Close prices, sorted descending by time, enabling analysis of price history and market trends.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base** | **String** | Base currency symbol | [required] |
**quote** | **String** | Quote currency symbol | [required] |
**interval** | **String** | Interval between two consecutive points in time series | [required] |
**base_type** | Option<**String**> | Base instrument type according to the `/instrument_type` endpoint |  |
**base_exchange** | Option<**String**> | Base exchange |  |
**base_mic_code** | Option<**String**> | Base MIC code |  |
**quote_type** | Option<**String**> | Quote instrument type according to the `/instrument_type` endpoint |  |
**quote_exchange** | Option<**String**> | Quote exchange |  |
**quote_mic_code** | Option<**String**> | Quote MIC code |  |
**outputsize** | Option<**i64**> | Number of data points to retrieve. Supports values in the range from `1` to `5000`. Default `30` when no date parameters are set, otherwise set to maximum |  |
**format** | Option<**String**> | Format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | Delimiter used in CSV file |  |[default to ;]
**prepost** | Option<**bool**> | Only for `Pro` and above plans. Available at the `1min`, `5min`, `15min`, and `30min` intervals for US equities. Open, high, low, close values are supplied without volume. |  |[default to false]
**start_date** | Option<**String**> | Start date for the time series data |  |
**end_date** | Option<**String**> | End date for the time series data |  |
**adjust** | Option<**bool**> | Specifies if there should be an adjustment |  |[default to true]
**dp** | Option<**i64**> | Specifies the number of decimal places for floating values. Should be in range [0, 11] inclusive. |  |[default to 5]
**timezone** | Option<**String**> | Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i> |  |

### Return type

[**models::GetTimeSeriesCross200ResponseEnum**](GetTimeSeriesCross_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

