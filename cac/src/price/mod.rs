use serde::{Deserialize, Serialize};


include!("list.rs");
include!("extension.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a price, expressed in a data structure containing multiple properties (compare
/// with UnstructuredPrice).
///
/// UBL Dictionary Entry Name: `Price. Details`
///
/// Generated from XSD type `PriceType`.
pub struct Price {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The amount of the price.
    #[serde(rename = "PriceAmount")]
    pub price_amount: cct::Amount,
/// The amount of the price inclusive of all taxes.
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: Option<cct::Amount>,
/// The quantity at which this price applies.
    #[serde(default, rename = "BaseQuantity")]
    pub base_quantity: Option<cct::Quantity>,
/// A reason for a price change.
    #[serde(default, rename = "PriceChangeReason")]
    pub price_change_reason: Vec<cct::Text>,
/// The type of price, expressed as a code.
    #[serde(default, rename = "PriceTypeCode")]
    pub price_type_code: Option<cct::Code>,
/// The type of price, expressed as text.
    #[serde(default, rename = "PriceType")]
    pub price_type: Option<cct::Text>,
/// The factor by which the base price unit can be converted to the orderable unit.
    #[serde(default, rename = "OrderableUnitFactorRate")]
    pub orderable_unit_factor_rate: Option<cct::Numeric>,
/// A period during which this price is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<crate::Period>,
/// Information about a price list applicable to this price.
    #[serde(default, rename = "PriceList")]
    pub price_list: Option<PriceList>,
/// An allowance or charge associated with this price.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// The exchange rate applicable to this price, if it differs from the exchange rate applicable to the
/// document as a whole.
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<crate::ExchangeRate>,
/// The price expressed in an alternative currency
    #[serde(default, rename = "AlternativeCurrencyPrice")]
    pub alternative_currency_price: Vec<Price>,
}
