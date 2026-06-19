#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsProcessing {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
    #[serde(default, rename = "ProcessingParty")]
    pub processing_party: Option<crate::Party>,
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: Vec<crate::CriterionItem>,
    #[serde(default, rename = "SubGoodsProcessing")]
    pub sub_goods_processing: Vec<GoodsProcessing>,
}
