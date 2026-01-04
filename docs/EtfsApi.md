# \EtfsApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_etfs_world**](EtfsApi.md#get_etfs_world) | **GET** /etfs/world | ETF full data
[**get_etfs_world_composition**](EtfsApi.md#get_etfs_world_composition) | **GET** /etfs/world/composition | Composition
[**get_etfs_world_performance**](EtfsApi.md#get_etfs_world_performance) | **GET** /etfs/world/performance | Performance
[**get_etfs_world_risk**](EtfsApi.md#get_etfs_world_risk) | **GET** /etfs/world/risk | Risk
[**get_etfs_world_summary**](EtfsApi.md#get_etfs_world_summary) | **GET** /etfs/world/summary | Summary



## get_etfs_world

> models::GetEtfsWorld200ResponseEnum get_etfs_world(symbol, figi, isin, cusip, country, dp)
ETF full data

The ETF full data endpoint provides detailed information about global Exchange-Traded Funds. It returns comprehensive data, including a summary, performance metrics, risk assessment, and composition details. This endpoint is ideal for users seeking an in-depth analysis of worldwide ETFs, enabling them to access key financial metrics and portfolio breakdowns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of etf |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetEtfsWorld200ResponseEnum**](GetETFsWorld_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etfs_world_composition

> models::GetEtfsWorldComposition200ResponseEnum get_etfs_world_composition(symbol, figi, isin, cusip, country, dp)
Composition

The ETFs composition endpoint provides detailed information about the composition of global Exchange-Traded Funds. It returns data on the sectors included in the ETF, specific holding details, and the weighted exposure of each component. This endpoint is useful for users who need to understand the specific makeup and sector distribution of an ETF portfolio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of etf |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetEtfsWorldComposition200ResponseEnum**](GetETFsWorldComposition_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etfs_world_performance

> models::GetEtfsWorldPerformance200ResponseEnum get_etfs_world_performance(symbol, figi, isin, cusip, country, dp)
Performance

The ETFs performance endpoint provides comprehensive performance data for exchange-traded funds globally. It returns detailed metrics such as trailing returns and annual returns, enabling users to evaluate the historical performance of various ETFs. This endpoint is ideal for users looking to compare ETF performance over different time periods and assess their investment potential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of etf |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetEtfsWorldPerformance200ResponseEnum**](GetETFsWorldPerformance_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etfs_world_risk

> models::GetEtfsWorldRisk200ResponseEnum get_etfs_world_risk(symbol, figi, isin, cusip, country, dp)
Risk

The ETFs risk endpoint provides essential risk metrics for global Exchange Traded Funds. It returns data such as volatility, beta, and other risk-related indicators, enabling users to assess the potential risk associated with investing in various ETFs worldwide.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of etf |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetEtfsWorldRisk200ResponseEnum**](GetETFsWorldRisk_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_etfs_world_summary

> models::GetEtfsWorldSummary200ResponseEnum get_etfs_world_summary(symbol, figi, isin, cusip, country, dp)
Summary

The ETFs summary endpoint provides a concise overview of global Exchange-Traded Funds. It returns key data points such as ETF names, symbols, and current market values, enabling users to quickly assess the performance and status of various international ETFs. This summary is ideal for users who need a snapshot of the global ETF landscape without delving into detailed analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of etf |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetEtfsWorldSummary200ResponseEnum**](GetETFsWorldSummary_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

