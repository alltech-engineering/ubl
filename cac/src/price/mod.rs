use serde::{Deserialize, Serialize};


include!("list.rs");
include!("extension.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Price {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "PriceAmount")]
    pub price_amount: cct::Amount,
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: Option<cct::Amount>,
    #[serde(default, rename = "BaseQuantity")]
    pub base_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "PriceChangeReason")]
    pub price_change_reason: Vec<cct::Text>,
    #[serde(default, rename = "PriceTypeCode")]
    pub price_type_code: Option<cct::Code>,
    #[serde(default, rename = "PriceType")]
    pub price_type: Option<cct::Text>,
    #[serde(default, rename = "OrderableUnitFactorRate")]
    pub orderable_unit_factor_rate: Option<cct::Numeric>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<crate::Period>,
    #[serde(default, rename = "PriceList")]
    pub price_list: Option<PriceList>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<crate::ExchangeRate>,
    #[serde(default, rename = "AlternativeCurrencyPrice")]
    pub alternative_currency_price: Vec<Price>,
}
