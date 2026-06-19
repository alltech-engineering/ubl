#[derive(Debug, Deserialize, Serialize)]
pub struct PowerOfAttorney {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "NotaryParty")]
    pub notary_party: Option<Box<Party>>,
    #[serde(rename = "AgentParty")]
    pub agent_party: Box<Party>,
    #[serde(default, rename = "WitnessParty")]
    pub witness_party: Vec<Party>,
    #[serde(default, rename = "MandateDocumentReference")]
    pub mandate_document_reference: Vec<DocumentReference>,
}
