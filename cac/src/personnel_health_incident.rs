#[derive(Debug, Deserialize, Serialize)]
pub struct PersonnelHealthIncident {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "JoinedShipDate")]
    pub joined_ship_date: Option<udt::DateTime>,
    #[serde(default, rename = "NatureOfIllnessDescription")]
    pub nature_of_illness_description: Vec<cct::Text>,
    #[serde(default, rename = "OnsetDate")]
    pub onset_date: Option<udt::DateTime>,
    #[serde(default, rename = "ReportedToMedicalOfficerIndicator")]
    pub reported_to_medical_officer_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "GivenTreatmentDescription")]
    pub given_treatment_description: Vec<cct::Text>,
    #[serde(default, rename = "StillIllIndicator")]
    pub still_ill_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "DiedIndicator")]
    pub died_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "StillOnBoardIndicator")]
    pub still_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "EvacuatedIndicator")]
    pub evacuated_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "BuriedAtSeaIndicator")]
    pub buried_at_sea_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Person")]
    pub person: Option<Person>,
}
