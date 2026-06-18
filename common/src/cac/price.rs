#[derive(Debug, Deserialize, Serialize)]
pub struct Price {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "PriceAmount")]
    pub price_amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "BaseQuantity")]
    pub base_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "PriceChangeReason")]
    pub price_change_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "PriceTypeCode")]
    pub price_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PriceType")]
    pub price_type: Option<super::cct::TextType>,
    #[serde(default, rename = "OrderableUnitFactorRate")]
    pub orderable_unit_factor_rate: Option<super::cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<Period>,
    #[serde(default, rename = "PriceList")]
    pub price_list: Option<PriceList>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<ExchangeRate>,
    #[serde(default, rename = "AlternativeCurrencyPrice")]
    pub alternative_currency_price: Vec<Price>,
}
