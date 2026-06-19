#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference provided by the buyer for internal routing or classification.
///
/// UBL Dictionary Entry Name: `Buyer Assigned Reference. Details`
///
/// Generated from XSD type `BuyerAssignedReferenceType`.
pub struct BuyerAssignedReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code identifying the buyer reference, such as a department or internal unit.
    #[serde(default, rename = "BuyerReferenceCode")]
    pub buyer_reference_code: Option<cct::Code>,
/// A textual description of the buyer reference.
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: Vec<cct::Text>,
}
