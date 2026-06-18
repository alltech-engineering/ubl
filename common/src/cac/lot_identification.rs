#[derive(Debug, Deserialize, Serialize)]
pub struct LotIdentification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LotNumberID")]
    pub lot_number_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<ItemProperty>,
}
