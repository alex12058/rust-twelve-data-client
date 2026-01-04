# \ReferenceDataApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bonds**](ReferenceDataApi.md#get_bonds) | **GET** /bonds | Fixed income
[**get_commodities**](ReferenceDataApi.md#get_commodities) | **GET** /commodities | Commodities
[**get_countries**](ReferenceDataApi.md#get_countries) | **GET** /countries | Countries
[**get_cross_listings**](ReferenceDataApi.md#get_cross_listings) | **GET** /cross_listings | Cross listings
[**get_cryptocurrencies**](ReferenceDataApi.md#get_cryptocurrencies) | **GET** /cryptocurrencies | Cryptocurrency pairs
[**get_cryptocurrency_exchanges**](ReferenceDataApi.md#get_cryptocurrency_exchanges) | **GET** /cryptocurrency_exchanges | Cryptocurrency exchanges
[**get_earliest_timestamp**](ReferenceDataApi.md#get_earliest_timestamp) | **GET** /earliest_timestamp | Earliest timestamp
[**get_etf**](ReferenceDataApi.md#get_etf) | **GET** /etfs | ETFs
[**get_etfs_family**](ReferenceDataApi.md#get_etfs_family) | **GET** /etfs/family | ETFs families
[**get_etfs_list**](ReferenceDataApi.md#get_etfs_list) | **GET** /etfs/list | ETFs directory
[**get_etfs_type**](ReferenceDataApi.md#get_etfs_type) | **GET** /etfs/type | ETFs types
[**get_exchange_schedule**](ReferenceDataApi.md#get_exchange_schedule) | **GET** /exchange_schedule | Exchanges schedule
[**get_exchanges**](ReferenceDataApi.md#get_exchanges) | **GET** /exchanges | Exchanges
[**get_forex_pairs**](ReferenceDataApi.md#get_forex_pairs) | **GET** /forex_pairs | Forex pairs
[**get_funds**](ReferenceDataApi.md#get_funds) | **GET** /funds | Funds
[**get_instrument_type**](ReferenceDataApi.md#get_instrument_type) | **GET** /instrument_type | Instrument type
[**get_intervals**](ReferenceDataApi.md#get_intervals) | **GET** /intervals | Intervals List
[**get_market_state**](ReferenceDataApi.md#get_market_state) | **GET** /market_state | Market state
[**get_mutual_funds_family**](ReferenceDataApi.md#get_mutual_funds_family) | **GET** /mutual_funds/family | MFs families
[**get_mutual_funds_list**](ReferenceDataApi.md#get_mutual_funds_list) | **GET** /mutual_funds/list | MFs directory
[**get_mutual_funds_type**](ReferenceDataApi.md#get_mutual_funds_type) | **GET** /mutual_funds/type | MFs types
[**get_stocks**](ReferenceDataApi.md#get_stocks) | **GET** /stocks | Stocks
[**get_symbol_search**](ReferenceDataApi.md#get_symbol_search) | **GET** /symbol_search | Symbol search
[**get_technical_indicators**](ReferenceDataApi.md#get_technical_indicators) | **GET** /technical_indicators | Technical indicators



## get_bonds

> models::GetBondsResponse get_bonds(symbol, exchange, country, format, delimiter, show_plan, page, outputsize)
Fixed income

The fixed income endpoint provides a daily updated list of available bonds. It returns an array containing detailed information about each bond, including identifiers, names, and other relevant attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]
**show_plan** | Option<**bool**> | Adds info on which plan symbol is available |  |[default to false]
**page** | Option<**i64**> | Page number of the results to fetch |  |[default to 1]
**outputsize** | Option<**i64**> | Determines the number of data points returned in the output |  |[default to 5000]

### Return type

[**models::GetBondsResponse**](GetBonds_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_commodities

> models::GetCommoditiesResponse get_commodities(symbol, category, format, delimiter)
Commodities

The commodities endpoint provides a daily updated list of available commodity pairs, across precious metals, livestock, softs, grains, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**category** | Option<**String**> | Filter by category of commodity |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]

### Return type

[**models::GetCommoditiesResponse**](GetCommodities_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_countries

> models::GetCountriesResponse get_countries()
Countries

The countries endpoint provides a comprehensive list of countries, including their ISO codes, official names, capitals, and currencies. This data is essential for applications requiring accurate country information for tasks such as localization, currency conversion, or geographic analysis.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetCountriesResponse**](GetCountries_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cross_listings

> models::GetCrossListingsResponse get_cross_listings(symbol, exchange, mic_code, country)
Cross listings

The cross_listings endpoint provides a daily updated list of cross-listed symbols for a specified financial instrument. Cross-listed symbols represent the same security available on multiple exchanges. This endpoint is useful for identifying all the exchanges where a particular security is traded, allowing users to access comprehensive trading information across different markets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | The ticker symbol of an instrument for which data is requested | [required] |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country to which stock exchange belongs to |  |

### Return type

[**models::GetCrossListingsResponse**](GetCrossListings_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cryptocurrencies

> models::GetCryptocurrenciesResponse get_cryptocurrencies(symbol, exchange, currency_base, currency_quote, format, delimiter)
Cryptocurrency pairs

The cryptocurrencies endpoint provides a daily updated list of all available cryptos. It returns an array containing detailed information about each cryptocurrency, including its symbol, name, and other relevant identifiers. This endpoint is useful for retrieving a comprehensive catalog of cryptocurrencies for applications that require up-to-date market listings or need to display available crypto assets to users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**exchange** | Option<**String**> | Filter by exchange name. E.g. `Binance`, `Coinbase`, etc. |  |
**currency_base** | Option<**String**> | Filter by currency base |  |
**currency_quote** | Option<**String**> | Filter by currency quote |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]

### Return type

[**models::GetCryptocurrenciesResponse**](GetCryptocurrencies_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cryptocurrency_exchanges

> models::GetCryptocurrencyExchangesResponse get_cryptocurrency_exchanges(format, delimiter)
Cryptocurrency exchanges

The cryptocurrency exchanges endpoint provides a daily updated list of available cryptocurrency exchanges. It returns an array containing details about each exchange, such as exchange names and identifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | Specify the delimiter used when downloading the CSV file |  |[default to ;]

### Return type

[**models::GetCryptocurrencyExchangesResponse**](GetCryptocurrencyExchanges_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_earliest_timestamp

> models::GetEarliestTimestampResponse get_earliest_timestamp(interval, symbol, figi, isin, cusip, exchange, mic_code, timezone)
Earliest timestamp

The earliest_timestamp endpoint provides the earliest available date and time for a specified financial instrument at a given data interval. This endpoint is useful for determining the starting point of historical data availability for various assets, such as stocks or currencies, allowing users to understand the time range covered by the data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | **String** | Interval between two consecutive points in time series. | [required] |
**symbol** | Option<**String**> | Symbol ticker of the instrument. |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded. |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard. |  |
**timezone** | Option<**String**> | Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>Exchange</code> for local exchange time</li> <li>2. <code>UTC</code> for datetime at universal UTC standard</li> <li>3. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i> |  |[default to Exchange]

### Return type

[**models::GetEarliestTimestampResponse**](GetEarliestTimestamp_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etf

> models::GetEtfResponse get_etf(symbol, figi, isin, cusip, cik, exchange, mic_code, country, format, delimiter, show_plan, include_delisted)
ETFs

The ETFs endpoint provides a daily updated list of all available Exchange-Traded Funds. It returns an array containing detailed information about each ETF, including its symbol, name, and other relevant identifiers. This endpoint is useful for retrieving a comprehensive catalog of ETFs for portfolio management, investment tracking, or financial analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**cik** | Option<**String**> | The CIK of an instrument for which data is requested |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**mic_code** | Option<**String**> | Filter by market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]
**show_plan** | Option<**bool**> | Adds info on which plan symbol is available |  |[default to false]
**include_delisted** | Option<**bool**> | Include delisted identifiers |  |[default to false]

### Return type

[**models::GetEtfResponse**](GetEtf_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etfs_family

> models::GetEtfsFamilyResponse get_etfs_family(country, fund_family)
ETFs families

Retrieve a comprehensive list of exchange-traded fund (ETF) families, providing users with detailed information on various ETF groups available in the market. This endpoint is ideal for users looking to explore different ETF categories, compare offerings, or integrate ETF family data into their financial applications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**fund_family** | Option<**String**> | Filter by investment company that manages the fund |  |

### Return type

[**models::GetEtfsFamilyResponse**](GetETFsFamily_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etfs_list

> models::GetEtfsListResponse get_etfs_list(symbol, figi, isin, cusip, cik, country, fund_family, fund_type, page, outputsize)
ETFs directory

The ETFs directory endpoint provides a daily updated list of exchange-traded funds, sorted by total assets in descending order. This endpoint is useful for retrieving comprehensive ETF data, including fund names and asset values, to assist users in quickly identifying the ETFs available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**cik** | Option<**String**> | The CIK of an instrument for which data is requested |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**fund_family** | Option<**String**> | Filter by investment company that manages the fund |  |
**fund_type** | Option<**String**> | Filter by the type of fund |  |
**page** | Option<**i64**> | Page number |  |[default to 1]
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 50]

### Return type

[**models::GetEtfsListResponse**](GetETFsList_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etfs_type

> models::GetEtfsTypeResponse get_etfs_type(country, fund_type)
ETFs types

The ETFs Types endpoint provides a concise list of ETF categories by market (e.g., Singapore, United States), including types like \"Equity Precious Metals\" and \"Large Blend.\" It supports targeted investment research and portfolio diversification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**fund_type** | Option<**String**> | Filter by the type of fund |  |

### Return type

[**models::GetEtfsTypeResponse**](GetETFsType_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_schedule

> models::GetExchangeScheduleResponse get_exchange_schedule(mic_name, mic_code, country, date)
Exchanges schedule

The exchanges schedule endpoint provides detailed information about various stock exchanges, including their trading hours and operational days. This data is essential for users who need to know when specific exchanges are open for trading, allowing them to plan their activities around the availability of these markets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mic_name** | Option<**String**> | Filter by exchange name |  |
**mic_code** | Option<**String**> | Filter by market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**date** | Option<**String**> | <p> If a date is provided, the API returns the schedule for the specified date; otherwise, it returns the default (common) schedule. </p> The date can be specified in one of the following formats: <ul> <li>An exact date (e.g., <code>2021-10-27</code>)</li> <li>A human-readable keyword: <code>today</code> or <code>yesterday</code></li> <li>A full datetime string in UTC (e.g., <code>2025-04-11T20:00:00</code>) to retrieve the schedule corresponding to the day in the specified time.</li> </ul> When using a datetime value, the resulting schedule will correspond to the local calendar day at the specified time. For example, <code>2025-04-11T20:00:00 UTC</code> corresponds to: <ul> <li><code>2025-04-11</code> in the <code>America/New_York</code> timezone</li> <li><code>2025-04-12</code> in the <code>Australia/Sydney</code> timezone</li> </ul> |  |

### Return type

[**models::GetExchangeScheduleResponse**](GetExchangeSchedule_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchanges

> models::GetExchangesResponse get_exchanges(r#type, name, code, country, format, delimiter, show_plan)
Exchanges

The exchanges endpoint provides a comprehensive list of all available equity exchanges. It returns an array containing detailed information about each exchange, such as exchange code, name, country, and timezone. This data is updated daily.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The asset class to which the instrument belongs |  |
**name** | Option<**String**> | Filter by exchange name |  |
**code** | Option<**String**> | Filter by market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]
**show_plan** | Option<**bool**> | Adds info on which plan symbol is available |  |[default to false]

### Return type

[**models::GetExchangesResponse**](GetExchanges_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_forex_pairs

> models::GetForexPairsResponse get_forex_pairs(symbol, currency_base, currency_quote, format, delimiter)
Forex pairs

The forex pairs endpoint provides a comprehensive list of all available foreign exchange currency pairs. It returns an array of forex pairs, which is updated daily.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**currency_base** | Option<**String**> | Filter by currency base |  |
**currency_quote** | Option<**String**> | Filter by currency quote |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]

### Return type

[**models::GetForexPairsResponse**](GetForexPairs_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_funds

> models::GetFundsResponse get_funds(symbol, figi, isin, cusip, cik, exchange, country, format, delimiter, show_plan, page, outputsize)
Funds

The funds endpoint provides a daily updated list of available investment funds. It returns an array containing detailed information about each fund, including identifiers, names, and other relevant attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**cik** | Option<**String**> | The CIK of an instrument for which data is requested |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]
**show_plan** | Option<**bool**> | Adds info on which plan symbol is available |  |[default to false]
**page** | Option<**i64**> | Page number of the results to fetch |  |[default to 1]
**outputsize** | Option<**i64**> | Determines the number of data points returned in the output |  |[default to 5000]

### Return type

[**models::GetFundsResponse**](GetFunds_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instrument_type

> models::GetInstrumentTypeResponse get_instrument_type()
Instrument type

The instrument type endpoint lists all available financial instrument types, such as stocks, ETFs, and cryptos. This information is essential for users to identify and categorize different financial instruments when accessing or analyzing market data.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetInstrumentTypeResponse**](GetInstrumentType_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intervals

> models::GetIntervalsResponse get_intervals()
Intervals List

The intervals endpoint provides a list of supported time intervals that can be used for querying financial data.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetIntervalsResponse**](GetIntervals_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_market_state

> models::GetMarketStateResponse get_market_state(exchange, code, country)
Market state

The market state endpoint provides real-time information on the operational status of all available stock exchanges. It returns data on whether each exchange is currently open or closed, along with the time remaining until the next opening or closing. This endpoint is useful for users who need to monitor exchange hours and plan their trading activities accordingly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange** | Option<**String**> | The exchange name where the instrument is traded. |  |
**code** | Option<**String**> | The Market Identifier Code (MIC) of the exchange where the instrument is traded. |  |
**country** | Option<**String**> | The country where the exchange is located. Takes country name or alpha code. |  |

### Return type

[**models::GetMarketStateResponse**](GetMarketState_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_family

> models::GetMutualFundsFamilyResponse get_mutual_funds_family(fund_family, country)
MFs families

The mutual funds family endpoint provides a comprehensive list of MF families, which are groups of mutual funds managed by the same investment company. This data is useful for users looking to explore or compare different fund families, understand the range of investment options offered by each, and identify potential investment opportunities within specific fund families.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fund_family** | Option<**String**> | Filter by investment company that manages the fund |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |

### Return type

[**models::GetMutualFundsFamilyResponse**](GetMutualFundsFamily_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_list

> models::GetMutualFundsListResponse get_mutual_funds_list(symbol, figi, isin, cusip, cik, country, fund_family, fund_type, performance_rating, risk_rating, page, outputsize)
MFs directory

The mutual funds directory endpoint provides a daily updated list of mutual funds, sorted in descending order by their total assets value. This endpoint is useful for retrieving an organized overview of available mutual funds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**cik** | Option<**String**> | The CIK of an instrument for which data is requested |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**fund_family** | Option<**String**> | Filter by investment company that manages the fund |  |
**fund_type** | Option<**String**> | Filter by the type of fund |  |
**performance_rating** | Option<**i64**> | Filter by performance rating from `0` to `5` |  |
**risk_rating** | Option<**i64**> | Filter by risk rating from `0` to `5` |  |
**page** | Option<**i64**> | Page number |  |[default to 1]
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 100]

### Return type

[**models::GetMutualFundsListResponse**](GetMutualFundsList_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_type

> models::GetMutualFundsTypeResponse get_mutual_funds_type(fund_type, country)
MFs types

This endpoint provides detailed information on various types of mutual funds, such as equity, bond, and balanced funds, allowing users to understand the different investment options available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fund_type** | Option<**String**> | Filter by the type of fund |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |

### Return type

[**models::GetMutualFundsTypeResponse**](GetMutualFundsType_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stocks

> models::GetStocksResponse get_stocks(symbol, figi, isin, cusip, cik, exchange, mic_code, country, r#type, format, delimiter, show_plan, include_delisted)
Stocks

The stocks endpoint provides a daily updated list of all available stock symbols. It returns an array containing the symbols, which can be used to identify and access specific stock data across various services. This endpoint is essential for users needing to retrieve the latest stock symbol information for further data requests or integration into financial applications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**cik** | Option<**String**> | The CIK of an instrument for which data is requested |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**mic_code** | Option<**String**> | Filter by market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**r#type** | Option<**String**> | The asset class to which the instrument belongs |  |
**format** | Option<**String**> | The format of the response data |  |[default to JSON]
**delimiter** | Option<**String**> | The separator used in the CSV response data |  |[default to ;]
**show_plan** | Option<**bool**> | Adds info on which plan symbol is available |  |[default to false]
**include_delisted** | Option<**bool**> | Include delisted identifiers |  |[default to false]

### Return type

[**models::GetStocksResponse**](GetStocks_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_symbol_search

> models::GetSymbolSearchResponse get_symbol_search(symbol, outputsize, show_plan)
Symbol search

The symbol search endpoint allows users to find financial instruments by name or symbol. It returns a list of matching symbols, ordered by relevance, with the most relevant instrument first. This is useful for quickly locating specific stocks, ETFs, or other financial instruments when only partial information is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** | Symbol to search. Supports: <ul> <li>Ticker symbol of instrument.</li> <li>International securities identification number (ISIN). <li>Financial instrument global identifier (FIGI). <li>Composite FIGI.</li> <li>Share Class FIGI.</li> </ul> | [required] |
**outputsize** | Option<**i64**> | Number of matches in response. Max <code>120</code> |  |[default to 30]
**show_plan** | Option<**bool**> | Adds info on which plan symbol is available. |  |[default to false]

### Return type

[**models::GetSymbolSearchResponse**](GetSymbolSearch_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_technical_indicators

> models::GetTechnicalIndicatorsResponse get_technical_indicators()
Technical indicators

The technical indicators endpoint provides a comprehensive list of available technical indicators, each represented as an object. This endpoint is useful for developers looking to integrate a variety of technical analysis tools into their applications, allowing for streamlined access to indicator data without needing to manually configure each one.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetTechnicalIndicatorsResponse**](GetTechnicalIndicators_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

