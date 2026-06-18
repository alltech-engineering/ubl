#[derive(Debug, Deserialize, Serialize)]
pub struct BuyerAssignedReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "BuyerReferenceCode")]
    pub buyer_reference_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: Vec<super::cct::TextType>,
}
