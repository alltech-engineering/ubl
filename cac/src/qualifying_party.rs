#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the distinctive features or characteristics qualifying an economic operator to
/// be a party in a tendering process (e.g., number of employees, number of operating units, type of
/// business, technical and financial capabilities, completed projects).
///
/// UBL Dictionary Entry Name: `Qualifying Party. Details`
///
/// Generated from XSD type `QualifyingPartyType`.
pub struct QualifyingParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The extent to which this party is expected to participate in the tendering process, expressed as a
/// percentage.
    #[serde(default, rename = "ParticipationPercent")]
    pub participation_percent: Option<cct::Numeric>,
/// Text describing the personal situation of the qualifying party.
    #[serde(default, rename = "PersonalSituation")]
    pub personal_situation: Vec<cct::Text>,
/// The number of years that this qualifying party has been in operation.
    #[serde(default, rename = "OperatingYearsQuantity")]
    pub operating_years_quantity: Option<cct::Quantity>,
/// The number of people employed by this qualifying party.
    #[serde(default, rename = "EmployeeQuantity")]
    pub employee_quantity: Option<cct::Quantity>,
/// An identifier for an item of evidence to support the classification of this qualifying party.
    #[serde(default, rename = "BusinessClassificationEvidenceID")]
    pub business_classification_evidence_id: Option<cct::Identifier>,
/// An identifier for an item of evidence to support the business identity of this qualifying party.
    #[serde(default, rename = "BusinessIdentityEvidenceID")]
    pub business_identity_evidence_id: Option<cct::Identifier>,
/// A code stating the Tenderer Role.
    #[serde(default, rename = "TendererRoleCode")]
    pub tenderer_role_code: Option<cct::Code>,
/// The classification scheme used for the business profile.
    #[serde(default, rename = "BusinessClassificationScheme")]
    pub business_classification_scheme: Option<ClassificationScheme>,
/// A technical capability of this qualifying party.
    #[serde(default, rename = "TechnicalCapability")]
    pub technical_capability: Vec<Capability>,
/// A financial capability of this qualifying party.
    #[serde(default, rename = "FinancialCapability")]
    pub financial_capability: Vec<Capability>,
/// A former task completed by this qualifying party.
    #[serde(default, rename = "CompletedTask")]
    pub completed_task: Vec<CompletedTask>,
/// A declaration by this qualifying party. of certain characteristics or capabilities in fulfilment of
/// requirements specified in a call for tenders.
    #[serde(default, rename = "Declaration")]
    pub declaration: Vec<Declaration>,
/// The Party who qualifies to participate in the Tender.
    #[serde(default, rename = "Party")]
    pub party: Option<Party>,
/// A class to describe the tenderer contracting role.
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Option<EconomicOperatorRole>,
}
