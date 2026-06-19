#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a revenue.
///
/// UBL Dictionary Entry Name: `Fee. Details`
///
/// Generated from XSD type `FeeType`.
pub struct Fee {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the type of this fee.
    #[serde(default, rename = "FeeTypeCode")]
    pub fee_type_code: Option<cct::Code>,
/// The amount of a fee.
    #[serde(default, rename = "FeeAmount")]
    pub fee_amount: Option<cct::Amount>,
/// Text describing this fee.
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<cct::Text>,
}
