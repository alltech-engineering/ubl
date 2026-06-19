#[derive(Debug, Deserialize, Serialize)]
pub struct BuyerAssignedReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "BuyerReferenceCode")]
    pub buyer_reference_code: Option<cct::Code>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: Vec<cct::Text>,
}
