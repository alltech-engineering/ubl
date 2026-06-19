#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorShortList {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "LimitationDescription")]
    pub limitation_description: Vec<cct::Text>,
    #[serde(default, rename = "ExpectedQuantity")]
    pub expected_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "PreSelectedParty")]
    pub pre_selected_party: Vec<crate::Party>,
}
