#[derive(Debug, Deserialize, Serialize)]
pub struct CommodityClassification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "NatureCode")]
    pub nature_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CargoTypeCode")]
    pub cargo_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CommodityCode")]
    pub commodity_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ItemClassificationCode")]
    pub item_classification_code: Option<super::cct::CodeType>,
}
