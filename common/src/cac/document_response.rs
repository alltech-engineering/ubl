#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Response")]
    pub response: Response,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
    #[serde(default, rename = "RecipientParty")]
    pub recipient_party: Option<Party>,
    #[serde(default, rename = "LineResponse")]
    pub line_response: Vec<LineResponse>,
}
