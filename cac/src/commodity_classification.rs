#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the classification of a commodity.
///
/// UBL Dictionary Entry Name: `Commodity Classification. Details`
///
/// Generated from XSD type `CommodityClassificationType`.
pub struct CommodityClassification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code defined by a specific maintenance agency signifying the high-level nature of the commodity.
    #[serde(default, rename = "NatureCode")]
    pub nature_code: Option<cct::Code>,
/// A mutually agreed code signifying the type of cargo for purposes of commodity classification.
    #[serde(default, rename = "CargoTypeCode")]
    pub cargo_type_code: Option<cct::Code>,
/// The harmonized international commodity code for cross border and regulatory (customs and trade
/// statistics) purposes.
    #[serde(default, rename = "CommodityCode")]
    pub commodity_code: Option<cct::Code>,
/// A code signifying the trade classification of the commodity.
    #[serde(default, rename = "ItemClassificationCode")]
    pub item_classification_code: Option<cct::Code>,
}
