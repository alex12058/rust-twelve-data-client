# IncomeStatementBlock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fiscal_date** | Option<**String**> | Date of the income statement release | [optional]
**quarter** | Option<**i64**> | Fiscal quarter. Visible when `&period=quarterly` | [optional]
**year** | Option<**i64**> | Fiscal year | [optional]
**sales** | Option<**i64**> | Refers to total reported revenue | [optional]
**cost_of_goods** | Option<**i64**> | Refers to cost of revenue | [optional]
**gross_profit** | Option<**i64**> | Refers to net gross profit: `sales` - `cost_of_goods` | [optional]
**operating_expense** | Option<[**models::IncomeStatementBlockOperatingExpense**](IncomeStatementBlock_operating_expense.md)> |  | [optional]
**operating_income** | Option<**i64**> | Refers to net operating income: `gross_profit` - `research_and_development` - `selling_general_and_administrative` | [optional]
**non_operating_interest** | Option<[**models::IncomeStatementBlockNonOperatingInterest**](IncomeStatementBlock_non_operating_interest.md)> |  | [optional]
**other_income_expense** | Option<**i64**> | Refers to other incomes or expenses | [optional]
**pretax_income** | Option<**i64**> | Refers to earnings before tax: `operating_income` + `net_non_operating_interest` - `other_income_expense` | [optional]
**income_tax** | Option<**i64**> | Refers to a tax provision | [optional]
**net_income** | Option<**i64**> | Refers to net income: `pretax_income` - `income_tax` | [optional]
**eps_basic** | Option<**f64**> | Refers to earnings per share (EPS) | [optional]
**eps_diluted** | Option<**f64**> | Refers to diluted earnings per share (EPS) | [optional]
**basic_shares_outstanding** | Option<**i64**> | Refers for the shares outstanding held by all its shareholders | [optional]
**diluted_shares_outstanding** | Option<**i64**> | Refers to the total number of shares a company would have if all dilutive securities were exercised and converted into shares | [optional]
**ebit** | Option<**i64**> | Refers to earnings before interest and taxes (EBIT) measure | [optional]
**ebitda** | Option<**i64**> | Refers to EBITDA (earnings before interest, taxes, depreciation, and amortization) measure | [optional]
**net_income_continuous_operations** | Option<**i64**> | Refers to the after-tax earnings that a business has generated from its operational activities | [optional]
**minority_interests** | Option<**i64**> | Refers to amount of minority interests paid out | [optional]
**preferred_stock_dividends** | Option<**i64**> | Refers to dividend that is allocated to and paid on a company's preferred shares | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


