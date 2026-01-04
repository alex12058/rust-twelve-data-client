# Quote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol passed | [optional]
**name** | Option<**String**> | Name of the instrument | [optional]
**exchange** | Option<**String**> | Exchange where instrument is traded | [optional]
**mic_code** | Option<**String**> | Market identifier code (MIC) under ISO 10383 standard. Available for stocks, ETFs, mutual funds, bonds | [optional]
**currency** | Option<**String**> | Currency in which the equity is denominated. Available for stocks, ETFs, mutual funds, bonds | [optional]
**datetime** | Option<**String**> | Datetime in defined timezone referring to when the bar with specified interval was opened | [optional]
**timestamp** | Option<**i64**> | Unix timestamp representing the opening candle of the specified interval | [optional]
**last_quote_at** | Option<**i64**> | Unix timestamp of last minute candle | [optional]
**open** | Option<**String**> | Price at the opening of current bar | [optional]
**high** | Option<**String**> | Highest price which occurred during the current bar | [optional]
**low** | Option<**String**> | Lowest price which occurred during the current bar | [optional]
**close** | Option<**String**> | Close price at the end of the bar | [optional]
**volume** | Option<**String**> | Trading volume during the bar. Available not for all instrument types | [optional]
**previous_close** | Option<**String**> | Close price at the end of the previous bar | [optional]
**change** | Option<**String**> | Close - previous_close | [optional]
**percent_change** | Option<**String**> | (Close - previous_close) / previous_close * 100 | [optional]
**average_volume** | Option<**String**> | Average volume of the specified period. Available not for all instrument types | [optional]
**rolling_1d_change** | Option<**String**> | Percent change in price between the current and the backward one, where period is 1 day. Available for crypto | [optional]
**rolling_7d_change** | Option<**String**> | Percent change in price between the current and the backward one, where period is 7 days. Available for crypto | [optional]
**rolling_change** | Option<**String**> | Percent change in price between the current and the backward one, where period specified in request param rolling_period. Available for crypto | [optional]
**is_market_open** | Option<**bool**> | True if market is open; false if closed | [optional]
**fifty_two_week** | Option<[**models::QuoteFiftyTwoWeek**](Quote_fifty_two_week.md)> |  | [optional]
**extended_change** | Option<**String**> | Diff between the regular close price and the latest extended price. Displayed only if prepost is true | [optional]
**extended_percent_change** | Option<**String**> | Percent change in price between the regular close price and the latest extended price. Displayed only if prepost is true | [optional]
**extended_price** | Option<**String**> | Latest extended price. Displayed only if prepost is true | [optional]
**extended_timestamp** | Option<**String**> | Unix timestamp of the last extended price. Displayed only if prepost is true | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


