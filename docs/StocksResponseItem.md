# StocksResponseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Instrument symbol (ticker) | [optional]
**name** | Option<**String**> | Full name of instrument | [optional]
**currency** | Option<**String**> | Currency of the instrument according to the ISO 4217 standard | [optional]
**exchange** | Option<**String**> | Exchange where instrument is traded | [optional]
**mic_code** | Option<**String**> | Market identifier code (MIC) under ISO 10383 standard | [optional]
**country** | Option<**String**> | Country where exchange is located | [optional]
**r#type** | Option<**String**> | Common issue type | [optional]
**figi_code** | Option<**String**> | Financial instrument global identifier (FIGI) | [optional]
**cfi_code** | Option<**String**> | Classification of Financial Instruments (CFI) | [optional]
**isin** | Option<**String**> | International securities identification number (ISIN), available by individual request to support | [optional]
**cusip** | Option<**String**> | A unique nine-character alphanumeric code used to identify financial securities, ensuring accurate data retrieval for the specified asset | [optional]
**access** | Option<[**models::EtfResponseItemAccess**](EtfResponseItem_access.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


