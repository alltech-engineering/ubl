#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "ManifestTypeCode")]
    pub manifest_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "ManifestType")]
    pub manifest_type: Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: Option<cct::AmountType>,
    #[serde(rename = "SendingLogisticsOperatorParty")]
    pub sending_logistics_operator_party: cac::Party,
    #[serde(default, rename = "AuthorityParty")]
    pub authority_party: Option<cac::Party>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<cac::Party>,
    #[serde(default, rename = "CrewPerson")]
    pub crew_person: Vec<cac::Person>,
    #[serde(default, rename = "PassengerPerson")]
    pub passenger_person: Vec<cac::Person>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
