#[derive(Debug, Deserialize, Serialize)]
pub struct UtilityItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: Option<super::cct::TextType>,
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<super::cct::TextType>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CurrentChargeType")]
    pub current_charge_type: Option<super::cct::TextType>,
    #[serde(default, rename = "CurrentChargeTypeCode")]
    pub current_charge_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "OneTimeChargeType")]
    pub one_time_charge_type: Option<super::cct::TextType>,
    #[serde(default, rename = "OneTimeChargeTypeCode")]
    pub one_time_charge_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Option<TaxCategory>,
    #[serde(default, rename = "Contract")]
    pub contract: Option<Contract>,
}
