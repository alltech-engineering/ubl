#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueRequestLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: Option<Period>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<ItemLocationQuantity>,
    #[serde(rename = "Item")]
    pub item: Item,
}
