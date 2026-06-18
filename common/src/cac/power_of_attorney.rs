#[derive(Debug, Deserialize, Serialize)]
pub struct PowerOfAttorney {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "NotaryParty")]
    pub notary_party: Option<Box<Party>>,
    #[serde(rename = "AgentParty")]
    pub agent_party: Box<Party>,
    #[serde(default, rename = "WitnessParty")]
    pub witness_party: Vec<Party>,
    #[serde(default, rename = "MandateDocumentReference")]
    pub mandate_document_reference: Vec<DocumentReference>,
}
