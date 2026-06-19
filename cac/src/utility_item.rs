#[derive(Debug, Deserialize, Serialize)]
pub struct UtilityItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: Option<cct::Identifier>,
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: Option<cct::Text>,
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<cct::Text>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<cct::Code>,
    #[serde(default, rename = "CurrentChargeType")]
    pub current_charge_type: Option<cct::Text>,
    #[serde(default, rename = "CurrentChargeTypeCode")]
    pub current_charge_type_code: Option<cct::Code>,
    #[serde(default, rename = "OneTimeChargeType")]
    pub one_time_charge_type: Option<cct::Text>,
    #[serde(default, rename = "OneTimeChargeTypeCode")]
    pub one_time_charge_type_code: Option<cct::Code>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Option<TaxCategory>,
    #[serde(default, rename = "Contract")]
    pub contract: Option<Contract>,
}
