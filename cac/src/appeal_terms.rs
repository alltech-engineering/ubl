#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the terms and conditions, set by the contracting authority, under which an
/// appeal can be lodged for a tender award.
///
/// UBL Dictionary Entry Name: `Appeal Terms. Details`
///
/// Generated from XSD type `AppealTermsType`.
pub struct AppealTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Text describing the terms of an appeal.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The period during which an appeal can be presented.
    #[serde(default, rename = "PresentationPeriod")]
    pub presentation_period: Option<Period>,
/// The Party who presents the information for the appeal.
    #[serde(default, rename = "AppealInformationParty")]
    pub appeal_information_party: Option<Party>,
/// The Party who receives the appeal.
    #[serde(default, rename = "AppealReceiverParty")]
    pub appeal_receiver_party: Option<Party>,
/// The Party who mediates any appeal.
    #[serde(default, rename = "MediationParty")]
    pub mediation_party: Option<Party>,
}
