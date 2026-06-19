#[derive(Debug, Deserialize, Serialize)]
pub struct IspsRequirements {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ValidISSCIndicator")]
    pub valid_issc_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ISSCAbsenceReason")]
    pub issc_absence_reason: Vec<cct::Text>,
    #[serde(default, rename = "ISSCExpiryDate")]
    pub issc_expiry_date: Option<udt::DateTime>,
    #[serde(default, rename = "SSPOnBoardIndicator")]
    pub ssp_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "SSPSecurityMeasuresAppliedIndicator")]
    pub ssp_security_measures_applied_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "CurrentOperatingSecurityLevelCode")]
    pub current_operating_security_level_code: Option<cct::Code>,
    #[serde(default, rename = "AdditionalMattersDescription")]
    pub additional_matters_description: Vec<cct::Text>,
    #[serde(default, rename = "AdditionalSecurityMeasure")]
    pub additional_security_measure: Vec<SecurityMeasure>,
    #[serde(default, rename = "PortCallRecord")]
    pub port_call_record: Vec<PortCallRecord>,
    #[serde(default, rename = "ShipToShipActivityRecord")]
    pub ship_to_ship_activity_record: Vec<ShipToShipActivityRecord>,
    #[serde(default, rename = "ReportLocation")]
    pub report_location: Option<Location>,
    #[serde(default, rename = "ISSCIssuerParty")]
    pub issc_issuer_party: Option<Party>,
    #[serde(default, rename = "SecurityOfficerPerson")]
    pub security_officer_person: Option<Person>,
}
