#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the party endorsing a document.
///
/// UBL Dictionary Entry Name: `Endorser Party. Details`
///
/// Generated from XSD type `EndorserPartyType`.
pub struct EndorserParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code specifying the role of the party providing the endorsement (e.g., issuer, embassy, insurance,
/// etc.).
    #[serde(rename = "RoleCode")]
    pub role_code: cct::Code,
/// A number indicating the order of the endorsement provided by this party in the sequence in which
/// endorsements are to be applied.
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: cct::Numeric,
/// The Party who endorses the application.
    #[serde(rename = "Party")]
    pub party: Party,
/// The individual representing the exporter who signs the Certificate of Origin application before
/// submitting it to the issuer party.
    #[serde(rename = "SignatoryContact")]
    pub signatory_contact: Contact,
}
