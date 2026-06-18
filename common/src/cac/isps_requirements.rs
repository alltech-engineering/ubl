#[derive(Debug, Deserialize, Serialize)]
pub struct IspsRequirements {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ValidISSCIndicator")]
    pub valid_issc_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ISSCAbsenceReason")]
    pub issc_absence_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "ISSCExpiryDate")]
    pub issc_expiry_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "SSPOnBoardIndicator")]
    pub ssp_on_board_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SSPSecurityMeasuresAppliedIndicator")]
    pub ssp_security_measures_applied_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CurrentOperatingSecurityLevelCode")]
    pub current_operating_security_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AdditionalMattersDescription")]
    pub additional_matters_description: Vec<super::cct::TextType>,
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
