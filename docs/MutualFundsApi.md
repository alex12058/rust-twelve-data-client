# \MutualFundsApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_mutual_funds_world**](MutualFundsApi.md#get_mutual_funds_world) | **GET** /mutual_funds/world | MF full data
[**get_mutual_funds_world_composition**](MutualFundsApi.md#get_mutual_funds_world_composition) | **GET** /mutual_funds/world/composition | Composition
[**get_mutual_funds_world_performance**](MutualFundsApi.md#get_mutual_funds_world_performance) | **GET** /mutual_funds/world/performance | Performance
[**get_mutual_funds_world_purchase_info**](MutualFundsApi.md#get_mutual_funds_world_purchase_info) | **GET** /mutual_funds/world/purchase_info | Purchase info
[**get_mutual_funds_world_ratings**](MutualFundsApi.md#get_mutual_funds_world_ratings) | **GET** /mutual_funds/world/ratings | Ratings
[**get_mutual_funds_world_risk**](MutualFundsApi.md#get_mutual_funds_world_risk) | **GET** /mutual_funds/world/risk | Risk
[**get_mutual_funds_world_summary**](MutualFundsApi.md#get_mutual_funds_world_summary) | **GET** /mutual_funds/world/summary | Summary
[**get_mutual_funds_world_sustainability**](MutualFundsApi.md#get_mutual_funds_world_sustainability) | **GET** /mutual_funds/world/sustainability | Sustainability



## get_mutual_funds_world

> models::GetMutualFundsWorldResponse get_mutual_funds_world(symbol, figi, isin, cusip, country, dp)
MF full data

The mutual full data endpoint provides detailed information about global mutual funds. It returns a comprehensive dataset that includes a summary of the fund, its performance metrics, risk assessment, ratings, asset composition, purchase details, and sustainability factors. This endpoint is essential for users seeking in-depth insights into mutual funds on a global scale, allowing them to evaluate various aspects such as investment performance, risk levels, and environmental impact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldResponse**](GetMutualFundsWorld_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_world_composition

> models::GetMutualFundsWorldCompositionResponse get_mutual_funds_world_composition(symbol, figi, isin, cusip, country, dp)
Composition

The mutual funds compositions endpoint provides detailed information about the portfolio composition of a specified mutual fund. It returns data on sector allocations, individual holdings, and their respective weighted exposures. This endpoint is useful for users seeking to understand the investment distribution and risk profile of a mutual fund.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldCompositionResponse**](GetMutualFundsWorldComposition_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_world_performance

> models::GetMutualFundsWorldPerformanceResponse get_mutual_funds_world_performance(symbol, figi, isin, cusip, country, dp)
Performance

The mutual funds performances endpoint provides comprehensive performance data for mutual funds globally. It returns metrics such as trailing returns, annual returns, quarterly returns, and load-adjusted returns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldPerformanceResponse**](GetMutualFundsWorldPerformance_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_world_purchase_info

> models::GetMutualFundsWorldPurchaseInfoResponse get_mutual_funds_world_purchase_info(symbol, figi, isin, cusip, country, dp)
Purchase info

The mutual funds purchase information endpoint provides detailed purchasing details for global mutual funds. It returns data on minimum investment requirements, current pricing, and a list of brokerages where the mutual fund can be purchased. This endpoint is useful for users looking to understand the entry requirements and options available for investing in specific mutual funds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldPurchaseInfoResponse**](GetMutualFundsWorldPurchaseInfo_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_world_ratings

> models::GetMutualFundsWorldRatingsResponse get_mutual_funds_world_ratings(symbol, figi, isin, cusip, country, dp)
Ratings

The mutual funds ratings endpoint provides detailed ratings for mutual funds across global markets. It returns data on the performance and quality of mutual funds, including ratings calculated in-house by Twelve Data and from various financial institutions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldRatingsResponse**](GetMutualFundsWorldRatings_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_world_risk

> models::GetMutualFundsWorldRiskResponse get_mutual_funds_world_risk(symbol, figi, isin, cusip, country, dp)
Risk

The mutual funds risk endpoint provides detailed risk metrics for global mutual funds. It returns data such as standard deviation, beta, and Sharpe ratio, which help assess the volatility and risk profile of mutual funds across different markets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldRiskResponse**](GetMutualFundsWorldRisk_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_world_summary

> models::GetMutualFundsWorldSummaryResponse get_mutual_funds_world_summary(symbol, figi, isin, cusip, country, dp)
Summary

The mutual funds summary endpoint provides a concise overview of global mutual funds, including key details such as fund name, symbol, asset class, and region. This endpoint is useful for quickly obtaining essential information about various mutual funds worldwide, aiding in the comparison and selection of funds for investment portfolios.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldSummaryResponse**](GetMutualFundsWorldSummary_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mutual_funds_world_sustainability

> models::GetMutualFundsWorldSustainabilityResponse get_mutual_funds_world_sustainability(symbol, figi, isin, cusip, country, dp)
Sustainability

The mutual funds sustainability endpoint provides detailed information on the sustainability and Environmental, Social, and Governance (ESG) ratings of global mutual funds. It returns data such as ESG scores, sustainability metrics, and fund identifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of mutual fund |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Accepts value in range [0,11] |  |[default to 5]

### Return type

[**models::GetMutualFundsWorldSustainabilityResponse**](GetMutualFundsWorldSustainability_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

