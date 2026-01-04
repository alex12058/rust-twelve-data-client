# RevenueEstimateRevenueEstimateInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | Option<**String**> | Date of the estimate | [optional]
**period** | Option<**String**> | Period of estimation, can be `current_quarter`, `next_quarter`, `current_year`, or `next_year` | [optional]
**number_of_analysts** | Option<**i64**> | Number of analysts that made the estimation | [optional]
**avg_estimate** | Option<**f64**> | Average estimation across analysts | [optional]
**low_estimate** | Option<**f64**> | Lowest estimation given by an analyst | [optional]
**high_estimate** | Option<**f64**> | Highest estimation given by an analyst | [optional]
**year_ago_sales** | Option<**f64**> | Total revenue received a year ago relative to period | [optional]
**sales_growth** | Option<**f64**> | Estimated sales growth of the period in relation to year-ago sales in prc (%) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


