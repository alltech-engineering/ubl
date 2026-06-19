#[derive(Debug, Deserialize, Serialize)]
pub struct BuyerAssignedReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "BuyerReferenceCode")]
    pub buyer_reference_code: Option<cct::Code>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: Vec<cct::Text>,
}
