#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorShortList {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LimitationDescription")]
    pub limitation_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ExpectedQuantity")]
    pub expected_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "PreSelectedParty")]
    pub pre_selected_party: Vec<Party>,
}
