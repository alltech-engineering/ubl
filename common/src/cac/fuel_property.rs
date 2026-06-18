#[derive(Debug, Deserialize, Serialize)]
pub struct FuelProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "TypeID")]
    pub type_id: super::cct::IdentifierType,
    #[serde(rename = "Value")]
    pub value: super::cct::TextType,
}
