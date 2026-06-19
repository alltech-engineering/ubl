#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an endorsement of a document.
///
/// UBL Dictionary Entry Name: `Endorsement. Details`
///
/// Generated from XSD type `EndorsementType`.
pub struct Endorsement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this endorsement.
    #[serde(rename = "DocumentID")]
    pub document_id: cct::Identifier,
/// The status of this endorsement.
    #[serde(rename = "ApprovalStatus")]
    pub approval_status: cct::Text,
/// Remarks provided by the endorsing party.
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
/// The type of party providing this endorsement.
    #[serde(rename = "EndorserParty")]
    pub endorser_party: EndorserParty,
/// A signature applied to this endorsement.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<Signature>,
}
