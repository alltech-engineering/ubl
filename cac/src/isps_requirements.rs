#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a set of ISPS Requirements.
///
/// UBL Dictionary Entry Name: `ISPS Requirements. Details`
///
/// Generated from XSD type `ISPSRequirementsType`.
pub struct IspsRequirements {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for these ISPS requirements.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An indicator of whether the International Ship Security Certificate (ISSC) is valid (true) or not
/// (false).
    #[serde(default, rename = "ValidISSCIndicator")]
    pub valid_issc_indicator: Option<udt::Indicator>,
/// A text describing the reason if not having a International Ship Security Certificate (ISSC).
    #[serde(default, rename = "ISSCAbsenceReason")]
    pub issc_absence_reason: Vec<cct::Text>,
/// The expiration date of the International Ship Security Certificate (ISSC).
    #[serde(default, rename = "ISSCExpiryDate")]
    pub issc_expiry_date: Option<udt::DateTime>,
/// An indicator of whether the vessel has a Ship Security Plan (SSP) on board (true) or not (false).
    #[serde(default, rename = "SSPOnBoardIndicator")]
    pub ssp_on_board_indicator: Option<udt::Indicator>,
/// An indication of whether the Ship Security Plan (SSP) meassures are applied (true) or not (false).
    #[serde(default, rename = "SSPSecurityMeasuresAppliedIndicator")]
    pub ssp_security_measures_applied_indicator: Option<udt::Indicator>,
/// A code describing the current operating security level.
    #[serde(default, rename = "CurrentOperatingSecurityLevelCode")]
    pub current_operating_security_level_code: Option<cct::Code>,
/// A textual description of any addidtional matters concerning these ISPS requirements.
    #[serde(default, rename = "AdditionalMattersDescription")]
    pub additional_matters_description: Vec<cct::Text>,
/// Security measures for these ISPS requirements in addition to those in the ship security plan, such
/// as special measures taken in response to unforeseen events.
    #[serde(default, rename = "AdditionalSecurityMeasure")]
    pub additional_security_measure: Vec<SecurityMeasure>,
/// The port call records for these ISPS requirements.
    #[serde(default, rename = "PortCallRecord")]
    pub port_call_record: Vec<PortCallRecord>,
/// The recordded ship to ship activities for these ISPS requirements.
    #[serde(default, rename = "ShipToShipActivityRecord")]
    pub ship_to_ship_activity_record: Vec<ShipToShipActivityRecord>,
/// The location where these ISPC requirements are reported.
    #[serde(default, rename = "ReportLocation")]
    pub report_location: Option<Location>,
/// The Party who issues the International Ship Security Certificate (ISSC).
    #[serde(default, rename = "ISSCIssuerParty")]
    pub issc_issuer_party: Option<Party>,
/// The security officer reponsible for these ISPC requirements.
    #[serde(default, rename = "SecurityOfficerPerson")]
    pub security_officer_person: Option<Person>,
}
