#[derive(Debug, Deserialize, Serialize)]
pub struct CommodityClassification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "NatureCode")]
    pub nature_code: Option<cct::Code>,
    #[serde(default, rename = "CargoTypeCode")]
    pub cargo_type_code: Option<cct::Code>,
    #[serde(default, rename = "CommodityCode")]
    pub commodity_code: Option<cct::Code>,
    #[serde(default, rename = "ItemClassificationCode")]
    pub item_classification_code: Option<cct::Code>,
}
