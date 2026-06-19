#[derive(Debug, Deserialize, Serialize)]
pub struct Endorsement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "DocumentID")]
    pub document_id: cct::Identifier,
    #[serde(rename = "ApprovalStatus")]
    pub approval_status: cct::Text,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
    #[serde(rename = "EndorserParty")]
    pub endorser_party: EndorserParty,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<Signature>,
}
