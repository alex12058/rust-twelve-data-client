# \AdvancedApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**advanced**](AdvancedApi.md#advanced) | **POST** /batch | Batches
[**get_api_usage**](AdvancedApi.md#get_api_usage) | **GET** /api_usage | API usage



## advanced

> models::Advanced200ResponseEnum advanced(key)
Batches

The batch request endpoint allows users to request data for multiple financial instruments, time intervals, and data types simultaneously. This endpoint is useful for efficiently gathering diverse financial data in a single operation, reducing the need for multiple individual requests. Errors in specific requests do not affect the processing of others, and each error is reported separately, enabling easy troubleshooting.  ### Request body Only JSON `POST` requests are supported. The request content structure consists of key-value items. The key is a unique request ID. The value is requested url.  ### Response The response contains key-value data. The key is a unique request ID. The value is returned data.  ### API credits <ul> <li>The number of concurrent requests is limited by your subscription plan.</li> <li>Credits are consumed per requested endpoint, with the total usage equal to the sum of individual requests in the batch.</li> <li>If the requested data exceeds your available credits, only partial data will be returned asynchronously until your quota is exhausted.</li> <li>If one or more requests in the batch contain errors (e.g., invalid symbols or unsupported intervals), it will not affect the successful processing of other requests. Errors are reported individually within the response, allowing you to identify and correct specific issues without impacting the entire batch.</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<[**std::collections::HashMap<String, models::AdvancedRequestValue>**](advanced_request_value.md)> | Map of requests |  |

### Return type

[**models::Advanced200ResponseEnum**](advanced_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_usage

> models::GetApiUsage200ResponseEnum get_api_usage(format, delimiter, timezone)
API usage

The API Usage endpoint provides detailed information on your current API usage statistics. It returns data such as the number of requests made, remaining requests, and the reset time for your usage limits. This endpoint is essential for monitoring and managing your API consumption to ensure you stay within your allocated limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | Output format |  |[default to JSON]
**delimiter** | Option<**String**> | Specify the delimiter used when downloading the CSV file |  |[default to ;]
**timezone** | Option<**String**> | Timezone at which output datetime will be displayed. Supports: <ul> <li>1. <code>UTC</code> for datetime at universal UTC standard</li> <li>2. Timezone name according to the IANA Time Zone Database. E.g. <code>America/New_York</code>, <code>Asia/Singapore</code>. Full list of timezones can be found <a href=\"https://en.wikipedia.org/wiki/List_of_tz_database_time_zones\" target=\"blank\">here</a>.</li> </ul> <i>Take note that the IANA Timezone name is case-sensitive</i> |  |[default to UTC]

### Return type

[**models::GetApiUsage200ResponseEnum**](GetApiUsage_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

