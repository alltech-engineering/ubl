#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsProcessing {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "ProcessingParty")]
    pub processing_party: Option<Party>,
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: Vec<CriterionItem>,
    #[serde(default, rename = "SubGoodsProcessing")]
    pub sub_goods_processing: Vec<GoodsProcessing>,
}
