#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the processing of goods and products
///
/// UBL Dictionary Entry Name: `Goods Processing. Details`
///
/// Generated from XSD type `GoodsProcessingType`.
pub struct GoodsProcessing {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this goods processing.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A type code for this goods processing.
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
/// A description for this goods processing expressed in one or more languages
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The period within this goods processing was performed
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
/// The Party who processes the goods.
    #[serde(default, rename = "ProcessingParty")]
    pub processing_party: Option<crate::Party>,
/// A reference to a criterion item that applies to this goods processing
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: Vec<crate::CriterionItem>,
/// A subordinate processing to this goods processing
    #[serde(default, rename = "SubGoodsProcessing")]
    pub sub_goods_processing: Vec<GoodsProcessing>,
}
