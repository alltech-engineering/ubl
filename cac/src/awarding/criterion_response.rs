#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingCriterionResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "AwardingCriterionID")]
    pub awarding_criterion_id: Option<cct::Identifier>,
    #[serde(default, rename = "AwardingCriterionDescription")]
    pub awarding_criterion_description: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "SubordinateAwardingCriterionResponse")]
    pub subordinate_awarding_criterion_response: Vec<AwardingCriterionResponse>,
}
