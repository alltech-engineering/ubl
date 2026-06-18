#[derive(Debug, Deserialize, Serialize)]
pub struct ItemLocationQuantity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LeadTimeMeasure")]
    pub lead_time_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TradingRestrictions")]
    pub trading_restrictions: Vec<super::cct::TextType>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Vec<Address>,
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
    #[serde(default, rename = "DeliveryUnit")]
    pub delivery_unit: Vec<DeliveryUnit>,
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: Vec<TaxCategory>,
    #[serde(default, rename = "Package")]
    pub package: Option<Package>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "DependentPriceReference")]
    pub dependent_price_reference: Option<DependentPriceReference>,
    #[serde(default, rename = "ApplicableDeliveryPeriod")]
    pub applicable_delivery_period: Option<Period>,
}
