#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to a transaction line in a billing document.
///
/// UBL Dictionary Entry Name: `Billing Reference Line. Details`
///
/// Generated from XSD type `BillingReferenceLineType`.
pub struct BillingReferenceLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this transaction line in a billing document.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The monetary amount of the transaction line, including any allowances and charges but excluding
/// taxes.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// An allowance or charge applicable to the transaction line.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
}
