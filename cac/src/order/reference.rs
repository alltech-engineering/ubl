#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to an Order.
///
/// UBL Dictionary Entry Name: `Order Reference. Details`
///
/// Generated from XSD type `OrderReferenceType`.
pub struct OrderReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this order reference, assigned by the buyer.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An identifier for this order reference, assigned by the seller.
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<cct::Identifier>,
/// (Deprecated) Indicates whether the referenced Order is a copy (true) or the original (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for this order reference.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date on which the referenced Order was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time at which the referenced Order was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Text used for tagging purchasing card transactions.
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: Option<cct::Text>,
/// A code signifying the type of the referenced Order.
    #[serde(default, rename = "OrderTypeCode")]
    pub order_type_code: Option<cct::Code>,
/// A document associated with this reference to an Order.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<crate::DocumentReference>,
}
