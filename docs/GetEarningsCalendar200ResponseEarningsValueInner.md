# GetEarningsCalendar200ResponseEarningsValueInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Instrument symbol (ticker) | [optional]
**name** | Option<**String**> | Full name of instrument | [optional]
**currency** | Option<**String**> | Currency in which instrument is traded by ISO 4217 standard | [optional]
**exchange** | Option<**String**> | Exchange where instrument is traded | [optional]
**mic_code** | Option<**String**> | Market identifier code (MIC) under ISO 10383 standard | [optional]
**country** | Option<**String**> | Country where exchange is located | [optional]
**time** | Option<**String**> | Can be either of the following values: `Pre Market`, `After Hours`, `Time Not Supplied` | [optional]
**eps_estimate** | Option<**f64**> | Analyst estimate of the future company earning | [optional]
**eps_actual** | Option<**f64**> | Actual value of reported earning | [optional]
**difference** | Option<**f64**> | Delta between `eps_actual` and `eps_estimate` | [optional]
**surprise_prc** | Option<**f64**> | Surprise in percentage of the `eps_actual` related to `eps_estimate` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


