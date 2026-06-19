#[derive(Debug, Deserialize, Serialize)]
/// A class for information about pricing structure, lead time, delivery, and location associated with
/// an item.
///
/// UBL Dictionary Entry Name: `Item Location Quantity. Details`
///
/// Generated from XSD type `ItemLocationQuantityType`.
pub struct ItemLocationQuantity {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The lead time, i.e., the time taken from the time at which an item is ordered to the time of its
/// delivery.
    #[serde(default, rename = "LeadTimeMeasure")]
    pub lead_time_measure: Option<cct::Measure>,
/// The minimum quantity that can be ordered to qualify for a specific price.
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
/// The maximum quantity that can be ordered to qualify for a specific price.
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
/// An indication that the transported item, as delivered, in the stated quantity to the stated
/// location, is subject to an international regulation concerning the carriage of dangerous goods
/// (true) or not (false).
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
/// Text describing trade restrictions on the quantity of this item or on the item itself.
    #[serde(default, rename = "TradingRestrictions")]
    pub trading_restrictions: Vec<cct::Text>,
/// The applicable sales territory.
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Vec<crate::Address>,
/// The price associated with this item location quantity
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
/// A delivery unit in which the item is located.
    #[serde(default, rename = "DeliveryUnit")]
    pub delivery_unit: Vec<crate::DeliveryUnit>,
/// A tax category applicable to this item location quantity.
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: Vec<crate::TaxCategory>,
/// The package to which this price applies.
    #[serde(default, rename = "Package")]
    pub package: Option<crate::Package>,
/// An allowance or charge associated with this item location quantity.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// The price of the item as a percentage of the price of some other item.
    #[serde(default, rename = "DependentPriceReference")]
    pub dependent_price_reference: Option<crate::DependentPriceReference>,
/// The period during which item must be delivered for the price to apply
    #[serde(default, rename = "ApplicableDeliveryPeriod")]
    pub applicable_delivery_period: Option<crate::Period>,
}
