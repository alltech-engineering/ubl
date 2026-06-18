#[derive(Debug, Deserialize, Serialize)]
pub struct Endorsement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "DocumentID")]
    pub document_id: super::cct::IdentifierType,
    #[serde(rename = "ApprovalStatus")]
    pub approval_status: super::cct::TextType,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<super::cct::TextType>,
    #[serde(rename = "EndorserParty")]
    pub endorser_party: EndorserParty,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<Signature>,
}
