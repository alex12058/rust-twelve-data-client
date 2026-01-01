# MarketStateResponseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The full name of exchange | [optional]
**code** | Option<**String**> | Market identifier code (MIC) under ISO 10383 standard | [optional]
**country** | Option<**String**> | Country where the exchange is located | [optional]
**is_market_open** | Option<**bool**> | Indicates if the market is currently open | [optional]
**time_after_open** | Option<**String**> | Time after market opening in <code>HH:MM:SS</code> format; if currently closed - returns <code>00:00:00</code> | [optional]
**time_to_open** | Option<**String**> | Time to market opening in <code>HH:MM:SS</code> format; if currently open - returns <code>00:00:00</code> | [optional]
**time_to_close** | Option<**String**> | Time to market closing in <code>HH:MM:SS</code> format; if currently closed - returns <code>00:00:00</code> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


