# \RegulatoryApi

All URIs are relative to *https://api.twelvedata.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_direct_holders**](RegulatoryApi.md#get_direct_holders) | **GET** /direct_holders | Direct holders
[**get_fund_holders**](RegulatoryApi.md#get_fund_holders) | **GET** /fund_holders | Fund holders
[**get_insider_transactions**](RegulatoryApi.md#get_insider_transactions) | **GET** /insider_transactions | Insider transaction
[**get_institutional_holders**](RegulatoryApi.md#get_institutional_holders) | **GET** /institutional_holders | Institutional holders
[**get_source_sanctioned_entities**](RegulatoryApi.md#get_source_sanctioned_entities) | **GET** /sanctions/{source} | Sanctioned entities
[**get_tax_info**](RegulatoryApi.md#get_tax_info) | **GET** /tax_info | Tax information



## get_direct_holders

> models::GetDirectHolders200ResponseEnum get_direct_holders(symbol, figi, isin, cusip, exchange, mic_code, country)
Direct holders

The direct holders endpoint provides detailed information about the number of shares directly held by individuals or entities as recorded in a company's official share registry. This data is essential for understanding the distribution of stock ownership within a company, helping users identify major shareholders and assess shareholder concentration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |

### Return type

[**models::GetDirectHolders200ResponseEnum**](GetDirectHolders_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fund_holders

> models::GetFundHolders200ResponseEnum get_fund_holders(symbol, figi, isin, cusip, exchange, mic_code, country)
Fund holders

The fund holders endpoint provides detailed information about the proportion of a company's stock that is owned by mutual fund holders. It returns data on the number of shares held, the percentage of total shares outstanding, and the names of the mutual funds involved. This endpoint is useful for users looking to understand mutual fund investment in a specific company.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |

### Return type

[**models::GetFundHolders200ResponseEnum**](GetFundHolders_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_insider_transactions

> models::GetInsiderTransactions200ResponseEnum get_insider_transactions(symbol, figi, isin, cusip, exchange, mic_code, country)
Insider transaction

The insider transaction endpoint provides detailed data on trades executed by company insiders, such as executives and directors. It returns information including the insider's name, their role, the transaction type, the number of shares, the transaction date, and the price per share. This endpoint is useful for tracking insider activity and understanding potential insider sentiment towards a company's stock.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested, e.g., `AAPL`, `TSLA`. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded, e.g., `Nasdaq`, `NSE` |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., United States or US. |  |

### Return type

[**models::GetInsiderTransactions200ResponseEnum**](GetInsiderTransactions_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_institutional_holders

> models::GetInstitutionalHolders200ResponseEnum get_institutional_holders(symbol, figi, isin, cusip, exchange, mic_code, country)
Institutional holders

The institutional holders endpoint provides detailed information on the percentage and amount of a company's stock owned by institutional investors, such as pension funds, insurance companies, and investment firms. This data is essential for understanding the influence and involvement of large entities in a company's ownership structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol ticker of instrument. For preffered stocks use dot(.) delimiter. E.g. `BRK.A` or `BRK.B` will be correct |  |
**figi** | Option<**String**> | Filter by financial instrument global identifier (FIGI) |  |
**isin** | Option<**String**> | Filter by international securities identification number (ISIN) |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | Exchange where instrument is traded |  |
**mic_code** | Option<**String**> | Market Identifier Code (MIC) under ISO 10383 standard |  |
**country** | Option<**String**> | Country where instrument is traded, e.g., `United States` or `US` |  |

### Return type

[**models::GetInstitutionalHolders200ResponseEnum**](GetInstitutionalHolders_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source_sanctioned_entities

> models::GetSourceSanctionedEntities200ResponseEnum get_source_sanctioned_entities(source)
Sanctioned entities

The sanctions entities endpoint provides a comprehensive list of entities sanctioned by a specified authority, such as OFAC, UK, EU, or AU. Users can retrieve detailed information about individuals, organizations, and other entities subject to sanctions from the chosen source, facilitating compliance and risk management processes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | **String** | Sanctions source | [required] |

### Return type

[**models::GetSourceSanctionedEntities200ResponseEnum**](GetSourceSanctionedEntities_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tax_info

> models::GetTaxInfo200ResponseEnum get_tax_info(symbol, isin, figi, cusip, exchange, mic_code)
Tax information

The tax information endpoint provides detailed tax-related data for a specified financial instrument, including applicable tax rates and relevant tax codes. This information is essential for users needing to understand the tax implications associated with trading or investing in specific instruments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> | The ticker symbol of an instrument for which data is requested, e.g., `SKYQ`, `AIRE`, `ALM:BME`, `HSI:HKEX`. |  |
**isin** | Option<**String**> | The ISIN of an instrument for which data is requested |  |
**figi** | Option<**String**> | The FIGI of an instrument for which data is requested |  |
**cusip** | Option<**String**> | The CUSIP of an instrument for which data is requested. CUSIP access is activating in the <a href=\"https://twelvedata.com/account/add-ons\">Add-ons</a> section |  |
**exchange** | Option<**String**> | The exchange name where the instrument is traded, e.g., `Nasdaq`, `Euronext` |  |
**mic_code** | Option<**String**> | The Market Identifier Code (MIC) of the exchange where the instrument is traded, e.g., `XNAS`, `XLON` |  |

### Return type

[**models::GetTaxInfo200ResponseEnum**](GetTaxInfo_200_responseEnum.md)

### Authorization

[authorizationHeader](../README.md#authorizationHeader)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

