# CashFlowStruct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fiscal_date** | Option<**String**> | Date of the cash flow release | [optional]
**quarter** | Option<**String**> | Fiscal quarter. Visible when `&period=quarterly` | [optional]
**year** | Option<**i64**> | Fiscal year | [optional]
**operating_activities** | Option<[**models::CashFlowStructOperatingActivities**](CashFlowStruct_operating_activities.md)> |  | [optional]
**investing_activities** | Option<[**models::CashFlowStructInvestingActivities**](CashFlowStruct_investing_activities.md)> |  | [optional]
**financing_activities** | Option<[**models::CashFlowStructFinancingActivities**](CashFlowStruct_financing_activities.md)> |  | [optional]
**end_cash_position** | Option<**f64**> | Returns the amount of cash a company has when adding the change in cash and beginning cash balance for the current fiscal period | [optional]
**income_tax_paid** | Option<**f64**> | Refers to supplemental data about income tax paid | [optional]
**interest_paid** | Option<**f64**> | Refers to supplemental data about interest paid | [optional]
**free_cash_flow** | Option<**f64**> | Represents the cash a company generates after accounting for cash outflows to support operations and maintain its capital assets | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


