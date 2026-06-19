#[derive(Debug, Deserialize, Serialize)]
pub struct ItemLocationQuantity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LeadTimeMeasure")]
    pub lead_time_measure: Option<cct::Measure>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TradingRestrictions")]
    pub trading_restrictions: Vec<cct::Text>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Vec<crate::Address>,
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
    #[serde(default, rename = "DeliveryUnit")]
    pub delivery_unit: Vec<crate::DeliveryUnit>,
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: Vec<crate::TaxCategory>,
    #[serde(default, rename = "Package")]
    pub package: Option<crate::Package>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "DependentPriceReference")]
    pub dependent_price_reference: Option<crate::DependentPriceReference>,
    #[serde(default, rename = "ApplicableDeliveryPeriod")]
    pub applicable_delivery_period: Option<crate::Period>,
}
