# \AnalysisApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_analyst_ratings_light**](AnalysisApi.md#get_analyst_ratings_light) | **GET** /analyst_ratings/light | Analyst ratings snapshot
[**get_analyst_ratings_us_equities**](AnalysisApi.md#get_analyst_ratings_us_equities) | **GET** /analyst_ratings/us_equities | Analyst ratings US equities
[**get_earnings_estimate**](AnalysisApi.md#get_earnings_estimate) | **GET** /earnings_estimate | Earnings estimate
[**get_edgar_filings_archive**](AnalysisApi.md#get_edgar_filings_archive) | **GET** /edgar_filings/archive | EDGAR fillings
[**get_eps_revisions**](AnalysisApi.md#get_eps_revisions) | **GET** /eps_revisions | EPS revisions
[**get_eps_trend**](AnalysisApi.md#get_eps_trend) | **GET** /eps_trend | EPS trend
[**get_growth_estimates**](AnalysisApi.md#get_growth_estimates) | **GET** /growth_estimates | Growth estimates
[**get_price_target**](AnalysisApi.md#get_price_target) | **GET** /price_target | Price target
[**get_recommendations**](AnalysisApi.md#get_recommendations) | **GET** /recommendations | Recommendations
[**get_revenue_estimate**](AnalysisApi.md#get_revenue_estimate) | **GET** /revenue_estimate | Revenue estimate



## get_analyst_ratings_light

> models::GetAnalystRatingsLightResponse get_analyst_ratings_light(symbol, figi, isin, cusip, exchange, rating_change, outputsize, country)
Analyst ratings snapshot

The analyst ratings snapshot endpoint provides a streamlined summary of ratings from analyst firms for both US and international markets. It delivers essential data on analyst recommendations, including buy, hold, and sell ratings, allowing users to quickly assess the general sentiment of analysts towards a particular stock.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**rating_change** | Option<**String**> | Filter by rating change action |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 30]
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |

### Return type

[**models::GetAnalystRatingsLightResponse**](GetAnalystRatingsLight_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analyst_ratings_us_equities

> models::GetAnalystRatingsUsEquitiesResponse get_analyst_ratings_us_equities(symbol, figi, isin, cusip, exchange, rating_change, outputsize)
Analyst ratings US equities

The analyst ratings US equities endpoint provides detailed information on analyst ratings for U.S. stocks. It returns data on the latest ratings issued by various analyst firms, including the rating itself, the firm issuing the rating, and any changes in the rating. This endpoint is useful for users tracking analyst opinions on U.S. equities, allowing them to see how professional analysts view the potential performance of specific stocks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**rating_change** | Option<**String**> | Filter by rating change action |  |
**outputsize** | Option<**i64**> | Number of records in response |  |[default to 30]

### Return type

[**models::GetAnalystRatingsUsEquitiesResponse**](GetAnalystRatingsUsEquities_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_earnings_estimate

> models::GetEarningsEstimateResponse get_earnings_estimate(symbol, figi, isin, cusip, country, exchange)
Earnings estimate

The earnings estimate endpoint provides access to analysts' projected earnings per share (EPS) for a specific company, covering both upcoming quarterly and annual periods. This data is crucial for users who need to track and compare expected financial performance across different timeframes, aiding in the evaluation of a company's future profitability.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | The FIGI of an instrument for which data is requested. This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**country** | Option<**String**> | The country where the instrument is traded, e.g., `United States` or `US` |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |

### Return type

[**models::GetEarningsEstimateResponse**](GetEarningsEstimate_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edgar_filings_archive

> models::GetEdgarFilingsArchiveResponse get_edgar_filings_archive(symbol, figi, isin, cusip, exchange, mic_code, country, form_type, filled_from, filled_to, page, page_size)
EDGAR fillings

The EDGAR fillings endpoint provides access to a comprehensive collection of financial documents submitted to the SEC, including real-time and historical forms, filings, and exhibits. Users can retrieve detailed information about company disclosures, financial statements, and regulatory submissions, enabling them to access essential compliance and financial data directly from the SEC's EDGAR system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**mic_code** | Option<**String**> | Filter by market identifier code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**form_type** | Option<**String**> | Filter by form types, example `8-K`, `EX-1.1` |  |
**filled_from** | Option<**String**> | Filter by filled time from |  |
**filled_to** | Option<**String**> | Filter by filled time to |  |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Number of records in response |  |[default to 10]

### Return type

[**models::GetEdgarFilingsArchiveResponse**](GetEdgarFilingsArchive_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eps_revisions

> models::GetEpsRevisionsResponse get_eps_revisions(symbol, figi, isin, cusip, country, exchange)
EPS revisions

The EPS revisions endpoint provides updated analyst forecasts for a company's earnings per share (EPS) on both a quarterly and annual basis. It delivers data on how these EPS predictions have changed over the past week and month, allowing users to track recent adjustments in analyst expectations. This endpoint is useful for monitoring shifts in market sentiment regarding a company's financial performance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**exchange** | Option<**String**> | Filter by exchange name |  |

### Return type

[**models::GetEpsRevisionsResponse**](GetEpsRevisions_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eps_trend

> models::GetEpsTrendResponse get_eps_trend(symbol, figi, isin, cusip, country, exchange)
EPS trend

The EPS trend endpoint provides detailed historical data on Earnings Per Share (EPS) trends over specified periods. It returns a comprehensive breakdown of estimated EPS changes, allowing users to track and analyze the progression of a company's earnings performance over time. This endpoint is ideal for users seeking to understand historical EPS fluctuations and assess financial growth patterns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**exchange** | Option<**String**> | Filter by exchange name |  |

### Return type

[**models::GetEpsTrendResponse**](GetEpsTrend_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_growth_estimates

> models::GetGrowthEstimatesResponse get_growth_estimates(symbol, figi, isin, cusip, country, exchange)
Growth estimates

The growth estimates endpoint provides consensus analyst projections on a company's growth rates over various timeframes. It aggregates and averages estimates from multiple analysts, focusing on key financial metrics such as earnings per share and revenue. This endpoint is useful for obtaining a comprehensive view of expected company performance based on expert analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | The FIGI of an instrument for which data is requested. This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**country** | Option<**String**> | The country where the instrument is traded, e.g., `United States` or `US` |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |

### Return type

[**models::GetGrowthEstimatesResponse**](GetGrowthEstimates_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_price_target

> models::GetPriceTargetResponse get_price_target(symbol, figi, isin, cusip, country, exchange)
Price target

The price target endpoint provides detailed projections of a security's future price as estimated by financial analysts. It returns data including the high, low, and average price targets. This endpoint is useful for users seeking to understand potential future valuations of specific securities based on expert analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**exchange** | Option<**String**> | Filter by exchange name |  |

### Return type

[**models::GetPriceTargetResponse**](GetPriceTarget_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recommendations

> models::GetRecommendationsResponse get_recommendations(symbol, figi, isin, cusip, country, exchange)
Recommendations

The recommendations endpoint provides a summary of analyst opinions for a specific stock, delivering an average recommendation categorized as Strong Buy, Buy, Hold, or Sell. It also includes a numerical recommendation score, offering a quick overview of market sentiment based on expert analysis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | The FIGI of an instrument for which data is requested. This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**country** | Option<**String**> | The country where the instrument is traded, e.g., `United States` or `US` |  |
**exchange** | Option<**String**> | The exchange name where the instrument is traded, e.g., `Nasdaq`, `NSE`. |  |

### Return type

[**models::GetRecommendationsResponse**](GetRecommendations_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_revenue_estimate

> models::GetRevenueEstimateResponse get_revenue_estimate(symbol, figi, isin, cusip, country, exchange, dp)
Revenue estimate

The revenue estimate endpoint provides a company's projected quarterly and annual revenue figures based on analysts' estimates. This data is useful for users seeking insights into expected company performance, allowing them to compare forecasted sales with historical data or other companies' estimates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Filter by symbol |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI). This request parameter is available starting with the <a href=\"https://twelvedata.com/pricing\">Ultra<a> plan |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN). ISIN access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Data add-ons</a> section |  |
**country** | Option<**String**> | Filter by country name or alpha code, e.g., `United States` or `US` |  |
**exchange** | Option<**String**> | Filter by exchange name |  |
**dp** | Option<**i64**> | Number of decimal places for floating values. Should be in range [0,11] inclusive |  |[default to 5]

### Return type

[**models::GetRevenueEstimateResponse**](GetRevenueEstimate_response.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

