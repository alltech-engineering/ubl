#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a power of attorney.
///
/// UBL Dictionary Entry Name: `Power Of Attorney. Details`
///
/// Generated from XSD type `PowerOfAttorneyType`.
pub struct PowerOfAttorney {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this power of attorney.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The date on which this power of attorney was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time at which this power of attorney was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Text describing this power of attorney.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The Party who notarises this Power of Attorney.
    #[serde(default, rename = "NotaryParty")]
    pub notary_party: Option<Box<Party>>,
/// The Party acting as an agent or fiduciary for the principal and holding this Power of Attorney on
/// behalf of the principal.
    #[serde(rename = "AgentParty")]
    pub agent_party: Box<Party>,
/// A Witness to this Power of Attorney.
    #[serde(default, rename = "WitnessParty")]
    pub witness_party: Vec<Party>,
/// A reference to a mandate associated with this power of attorney.
    #[serde(default, rename = "MandateDocumentReference")]
    pub mandate_document_reference: Vec<DocumentReference>,
}
