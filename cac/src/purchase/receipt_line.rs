#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line item in a purchase receipt.
///
/// UBL Dictionary Entry Name: `Purchase Receipt Line. Details`
///
/// Generated from XSD type `PurchaseReceiptLineType`.
pub struct PurchaseReceiptLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this purchase receipt line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for this invoice line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity (of items) on this purchase receipt line.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The total amount for this purchase receipt line, including allowances and charges but net of taxes.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this purchase receipt line, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// A period to which this purchase line applies.
    #[serde(default, rename = "PurchaseLinePeriod")]
    pub purchase_line_period: Option<crate::Period>,
/// A reference to an object, such as a subscription number, telephone number, meter, vehicle, person,
/// etc., to which this purchase line relates.
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Option<PurchaseReference>,
/// An allowance or charge associated with this purchase line.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// A total amount of taxes of a particular kind applicable to this invoice line.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// The item associated with this invoice line.
    #[serde(rename = "Item")]
    pub item: crate::Item,
/// The price of the item associated with this purchase line.
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
}
