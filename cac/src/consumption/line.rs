#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a line item for utility consumption. To specify more than one utility item, use
/// separate consumption lines.
///
/// UBL Dictionary Entry Name: `Consumption Line. Details`
///
/// Generated from XSD type `ConsumptionLineType`.
pub struct ConsumptionLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this consumption line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An identifier for the transaction line on a related document (such as an invoice) that covers this
/// consumption line.
    #[serde(default, rename = "ParentDocumentLineReferenceID")]
    pub parent_document_line_reference_id: Option<cct::Identifier>,
/// The quantity invoiced.
    #[serde(rename = "InvoicedQuantity")]
    pub invoiced_quantity: cct::Quantity,
/// The monetary amount, including discount, to be charged for this consumption line.
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: cct::Amount,
/// The total amount for this consumption line, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The period of time covered by this consumption line.
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
/// A delivery of the utility item on this consumption line.
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<crate::Delivery>,
/// An allowance or charge that applies to this consumption line.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// A total amount of taxes of a particular kind applicable to this consumption line.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// The utility item consumed.
    #[serde(rename = "UtilityItem")]
    pub utility_item: crate::UtilityItem,
/// The price associated with this consumption line, expressed in a data structure containing multiple
/// properties.
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
/// The price associated with this consumption line expressed in a less structured form that includes
/// just the amount and the time of use.
    #[serde(default, rename = "UnstructuredPrice")]
    pub unstructured_price: Option<crate::UnstructuredPrice>,
}
