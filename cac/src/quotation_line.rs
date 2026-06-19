#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Quotation.
///
/// UBL Dictionary Entry Name: `Quotation Line. Details`
///
/// Generated from XSD type `QuotationLineType`.
pub struct QuotationLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this quotation line.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of the item quoted.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The total amount for this quotation line, including allowance charges but net of taxes.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this quotation line, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The total tax amount for this quotation line.
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<cct::Amount>,
/// An identifier for the line in the Request for Quotation to which this line is a response.
    #[serde(default, rename = "RequestForQuotationLineID")]
    pub request_for_quotation_line_id: Option<cct::Identifier>,
/// A reference to a document associated with this quotation line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// The item that is the subject of this quotation line.
    #[serde(rename = "LineItem")]
    pub line_item: LineItem,
/// An item proposed by the seller as a substitute for the item that is the subject of this quotation
/// line.
    #[serde(default, rename = "SellerProposedSubstituteLineItem")]
    pub seller_proposed_substitute_line_item: Vec<LineItem>,
/// An item proposed by the seller as an alternative to the item that is the subject of this quotation
/// line.
    #[serde(default, rename = "AlternativeLineItem")]
    pub alternative_line_item: Vec<LineItem>,
/// A reference to the line in the Request for Quotation to which this line is a response.
    #[serde(default, rename = "RequestLineReference")]
    pub request_line_reference: Option<LineReference>,
}
