# EarningsItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | Option<**String**> | Date of earning release | [optional]
**time** | Option<**String**> | Time of earning release, can be either of the following values: `Pre Market`, `After Hours`, `Time Not Supplied` | [optional]
**eps_estimate** | Option<**f64**> | Analyst estimate of the future company earning | [optional]
**eps_actual** | Option<**f64**> | Actual value of reported earning | [optional]
**difference** | Option<**f64**> | Delta between `eps_actual` and `eps_estimate` | [optional]
**surprise_prc** | Option<**f64**> | Surprise in the percentage of the `eps_actual` related to `eps_estimate` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


