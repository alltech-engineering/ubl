#[derive(Debug, Deserialize, Serialize)]
/// Defines the response for an awarding criterion from the tendering party.
///
/// UBL Dictionary Entry Name: `Awarding Criterion Response. Details`
///
/// Generated from XSD type `AwardingCriterionResponseType`.
pub struct AwardingCriterionResponse {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identification of this awarding criterion response.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// An identifer of the awarding criterion being referred to.
    #[serde(default, rename = "AwardingCriterionID")]
    pub awarding_criterion_id: Option<cct::Identifier>,
/// Describes the awarding criterion.
    #[serde(default, rename = "AwardingCriterionDescription")]
    pub awarding_criterion_description: Vec<cct::Text>,
/// Describes the awarding criterion response.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Specifies the quantity tendered for this awarding criterion.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// Specifies the monetary amount tendered for this awarding criterion.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// Defines responses to any subsidiary awarding criterion.
    #[serde(default, rename = "SubordinateAwardingCriterionResponse")]
    pub subordinate_awarding_criterion_response: Vec<AwardingCriterionResponse>,
}
