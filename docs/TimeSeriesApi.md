# \TimeSeriesApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_time_series**](TimeSeriesApi.md#get_time_series) | **GET** /time_series | Time series
[**get_time_series_cross**](TimeSeriesApi.md#get_time_series_cross) | **GET** /time_series/cross | Time series cross



## get_time_series

> models::GetTimeSeriesResponse get_time_series(interval, symbol, isin, figi, cusip, outputsize, exchange, mic_code, country, r#type, timezone, start_date, end_date, date, order, prepost, format, delimiter, dp, previous_close, adjust)
Time series

The time series endpoint provides detailed historical data for a specified financial instrument. It returns two main components: metadata, which includes essential information about the instrument, and a time series dataset. The time series consists of chronological entries with Open, High, Low, and Close prices, and for applicable instruments, it also includes trading volume. This endpoint is ideal for retrieving comprehensive historical price data for analysis or visualization purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | **String** | Interval between two consecutive points in time series | [required] |
**symbol** | Option<**String**> | Symbol ticker of the instrument. E.g. `AAPL`, `EUR/USD`, `ETH/BTC`, ... |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**figi** | Option<**String**> | The FIGI of an instrument for which data is requested. This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
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

[**models::GetTimeSeriesResponse**](GetTimeSeries_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_series_cross

> models::GetTimeSeriesCrossResponse get_time_series_cross(base, quote, interval, base_type, base_exchange, base_mic_code, quote_type, quote_exchange, quote_mic_code, outputsize, format, delimiter, prepost, start_date, end_date, adjust, dp, timezone)
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

[**models::GetTimeSeriesCrossResponse**](GetTimeSeriesCross_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

