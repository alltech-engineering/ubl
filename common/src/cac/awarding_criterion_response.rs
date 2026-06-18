#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingCriterionResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AwardingCriterionID")]
    pub awarding_criterion_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AwardingCriterionDescription")]
    pub awarding_criterion_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "SubordinateAwardingCriterionResponse")]
    pub subordinate_awarding_criterion_response: Vec<AwardingCriterionResponse>,
}
