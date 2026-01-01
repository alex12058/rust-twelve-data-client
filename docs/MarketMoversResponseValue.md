# MarketMoversResponseValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The exchange symbol ticker | [optional]
**name** | Option<**String**> | The official name of the instrument | [optional]
**exchange** | Option<**String**> | Exchange where instrument is traded | [optional]
**mic_code** | Option<**String**> | Market identifier code (MIC) under ISO 10383 standard | [optional]
**datetime** | Option<**String**> | The last updated datetime timestamp | [optional]
**last** | Option<**f64**> | The latest available price for the symbol today | [optional]
**high** | Option<**f64**> | The highest price for the symbol today | [optional]
**low** | Option<**f64**> | The lowest price for the symbol today | [optional]
**volume** | Option<**i64**> | The trading volume of the symbol today | [optional]
**change** | Option<**f64**> | The value of the change since the previous day | [optional]
**percent_change** | Option<**f64**> | The percentage change since the previous day | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


