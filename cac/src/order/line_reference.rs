#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to an order line.
///
/// UBL Dictionary Entry Name: `Order Line Reference. Details`
///
/// Generated from XSD type `OrderLineReferenceType`.
pub struct OrderLineReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the referenced order line, assigned by the buyer.
    #[serde(rename = "LineID")]
    pub line_id: cct::Identifier,
/// An identifier for the referenced order line, assigned by the seller.
    #[serde(default, rename = "SalesOrderLineID")]
    pub sales_order_line_id: Option<cct::Identifier>,
/// A universally unique identifier for this order line reference.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// A code signifying the status of the referenced order line with respect to its original state.
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
/// A reference to the Order containing the referenced order line.
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<OrderReference>,
}
