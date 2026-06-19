#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueRequestLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: Option<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: Option<crate::Period>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
    #[serde(rename = "Item")]
    pub item: crate::Item,
}
