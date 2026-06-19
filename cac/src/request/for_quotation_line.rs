#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Request for Quotation.
///
/// UBL Dictionary Entry Name: `Request For Quotation Line. Details`
///
/// Generated from XSD type `RequestForQuotationLineType`.
pub struct RequestForQuotationLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this line in the request for quotation.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A universally unique identifier for this line in the request for quotation.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// An indication whether this line is optional (true) or not (false) for purposes of this request for
/// quotation.
    #[serde(default, rename = "OptionalLineItemIndicator")]
    pub optional_line_item_indicator: Option<udt::Indicator>,
/// A code signifying the level of confidentiality of this request for quotation line.
    #[serde(default, rename = "PrivacyCode")]
    pub privacy_code: Option<cct::Code>,
/// A code signifying the security classification of this request for quotation line.
    #[serde(default, rename = "SecurityClassificationCode")]
    pub security_classification_code: Option<cct::Code>,
/// A document associated with this request for quotation line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// A description of the item for which a quotation is requested.
    #[serde(rename = "LineItem")]
    pub line_item: crate::LineItem,
}
