#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the distribution of a document to an interested party.
///
/// UBL Dictionary Entry Name: `Document Distribution. Details`
///
/// Generated from XSD type `DocumentDistributionType`.
pub struct DocumentDistribution {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this document distribution.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The type of document, expressed as a code.
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::Code>,
/// The type of distribution, expressed as a code.
    #[serde(default, rename = "DistributionTypeCode")]
    pub distribution_type_code: Option<cct::Code>,
/// The type of distribution, expressed as text.
    #[serde(default, rename = "DistributionType")]
    pub distribution_type: Vec<cct::Text>,
/// (Deprecated) Text describing the interested party’s rights and limitations for distributing
/// originals and copies of this document.
    #[serde(default, rename = "PrintQualifier")]
    pub print_qualifier: Option<cct::Text>,
/// (Deprecated) An indicator that the document in this ditribution is a copy (true) or the original
/// (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// (Deprecated) The maximum number of printed copies of the document that the interested party is
/// allowed to make.
    #[serde(default, rename = "MaximumCopiesNumeric")]
    pub maximum_copies_numeric: Option<cct::Numeric>,
/// (Deprecated) The maximum number of printed originals of the document that the interested party is
/// allowed to make.
    #[serde(default, rename = "MaximumOriginalsNumeric")]
    pub maximum_originals_numeric: Option<cct::Numeric>,
/// A Communication used for this document distribution.
    #[serde(default, rename = "Communication")]
    pub communication: Option<crate::Communication>,
/// The interested Party who receives this Document.
    #[serde(rename = "Party")]
    pub party: crate::Party,
}
